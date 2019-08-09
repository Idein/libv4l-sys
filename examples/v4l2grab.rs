use std::ffi::{CStr, CString};
use std::fmt;
use std::fs;
use std::io::Write;
use std::iter::Iterator;
use std::mem;
use std::ptr;
use std::slice;

use log::*;

use libv4l_sys as v4l;

macro_rules! errno {
    () => {
        unsafe { *libc::__errno_location() }
    };
}

#[derive(Debug, Clone)]
struct Framesize {
    pub width: u32,
    pub height: u32,
}

impl fmt::Display for Framesize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}", self.width, self.height)
    }
}

#[derive(Debug, Clone)]
enum FramesizeKind {
    Discrete(Vec<Framesize>),
    Stepwise {
        min_width: u32,
        max_width: u32,
        min_height: u32,
        max_height: u32,
        step_width: u32,
        step_height: u32,
    },
}

impl FramesizeKind {
    fn iter(&self) -> FramesizeIter {
        FramesizeIter::new(self.clone())
    }
}

#[derive(Debug, Clone)]
struct FramesizeIter {
    kind: FramesizeKind,
    next: Option<Framesize>,
}

impl FramesizeIter {
    fn new(kind: FramesizeKind) -> Self {
        use FramesizeKind::*;
        match kind {
            Discrete(mut fs) => {
                let next = fs.pop();
                FramesizeIter {
                    kind: Discrete(fs),
                    next,
                }
            }
            Stepwise {
                min_width,
                min_height,
                ..
            } => FramesizeIter {
                kind,
                next: Some(Framesize {
                    width: min_width,
                    height: min_height,
                }),
            },
        }
    }
}

impl Iterator for FramesizeIter {
    type Item = Framesize;
    fn next(&mut self) -> Option<Self::Item> {
        use FramesizeKind::*;
        let curr = self.next.clone();
        if let Some(ref frm) = &curr {
            self.next = match &mut self.kind {
                Discrete(ref mut fs) => fs.pop(),
                Stepwise {
                    ref min_width,
                    ref max_width,
                    ref step_width,
                    ref max_height,
                    ref step_height,
                    ..
                } => {
                    if frm.width < *max_width {
                        Some(Framesize {
                            width: frm.width + step_width,
                            height: frm.height,
                        })
                    } else {
                        if frm.height + step_height < *max_height {
                            Some(Framesize {
                                width: *min_width,
                                height: frm.height + step_height,
                            })
                        } else {
                            None
                        }
                    }
                }
            }
        };
        curr
    }
}

#[derive(Debug)]
struct buffer {
    start: *mut libc::c_void,
    length: libc::size_t,
}

fn strerror() -> String {
    let errno = errno!();
    unsafe { CStr::from_ptr(libc::strerror(errno)) }
        .to_string_lossy()
        .into()
}

fn xioctl(fd: libc::c_int, request: libc::c_uint, arg: *mut libc::c_void) -> libc::c_int {
    let mut r = 0;

    loop {
        r = unsafe { v4l::v4l2_ioctl(fd, request, arg) };
        if r == -1 && ((errno!() == libc::EINTR) || (errno!() == libc::EAGAIN)) {
            continue;
        } else {
            break;
        }
    }
    r
    /*
    if r == -1 {
        error!("error {}, {}", errno!(), strerror());
        panic!()
    }
    */
}

fn main() {
    println!("v4l2grab");
    env_logger::init();

    let dev_name = CString::new("/dev/video0").unwrap();

    let fd = unsafe {
        let fd = v4l::v4l2_open(dev_name.as_ptr(), libc::O_RDWR | libc::O_NONBLOCK, 0);
        if fd == -1 {
            error!("open device error: {}: {}", fd, strerror());
            panic!()
        }
        fd
    };

    let V4L2_PIX_FMT_RGB24: u32 =
        ((b'R' as u32) << 0) | ((b'G' as u32) << 8) | ((b'B' as u32) << 16) | ((b'3' as u32) << 24);

    /*
    VIDIOC_ENUM_FRAMESIZES
    ==> _IOWR('V', 74, struct v4l2_frmsizeenum)
    ...
    ==> ((3U  << 30)  |
         ('V' << 8) |
         74 |
         (sizeof(v4l2_frmsizeenum) << 16))
    */
    let VIDIOC_ENUM_FRAMESIZES: libc::c_uint = ((3 as libc::c_uint) << 30)
        | ((b'V' as libc::c_uint) << 8)
        | (74 as libc::c_uint)
        | ((mem::size_of::<v4l::v4l2_frmsizeenum>() as libc::c_uint) << 16);
    let mut framesize = unsafe {
        let mut framesize: v4l::v4l2_frmsizeenum = mem::zeroed();
        framesize.pixel_format = V4L2_PIX_FMT_RGB24;
        framesize
    };
    let mut idx = 0;
    if xioctl(
        fd,
        VIDIOC_ENUM_FRAMESIZES,
        &mut framesize as *mut _ as *mut libc::c_void,
    ) == -1
    {
        error!("error {}, {}", errno!(), strerror());
        panic!()
    }
    let supported_framesize =
        if framesize.type_ == v4l::v4l2_frmivaltypes_V4L2_FRMIVAL_TYPE_DISCRETE {
            let mut frame_sizes = vec![];
            loop {
                let discrete = unsafe { framesize.__bindgen_anon_1.discrete };
                debug!("discrete: {}x{}", discrete.width, discrete.height);
                frame_sizes.push(Framesize {
                    width: discrete.width,
                    height: discrete.height,
                });
                idx += 1;
                if xioctl(
                    fd,
                    VIDIOC_ENUM_FRAMESIZES,
                    &mut framesize as *mut _ as *mut libc::c_void,
                ) == -1
                {
                    break;
                }
                assert!(framesize.type_ == v4l::v4l2_frmivaltypes_V4L2_FRMIVAL_TYPE_DISCRETE);
                let discrete = unsafe { framesize.__bindgen_anon_1.discrete };
                frame_sizes.push(Framesize {
                    width: discrete.width,
                    height: discrete.height,
                });
            }
            FramesizeKind::Discrete(frame_sizes)
        } else {
            let stepwise = unsafe { framesize.__bindgen_anon_1.stepwise };
            debug!(
                "[{},{}]({})x[{},{}]({})",
                stepwise.min_width,
                stepwise.max_width,
                stepwise.step_width,
                stepwise.min_height,
                stepwise.max_height,
                stepwise.step_height
            );
            FramesizeKind::Stepwise {
                min_width: stepwise.min_width,
                max_width: stepwise.max_width,
                min_height: stepwise.min_height,
                max_height: stepwise.max_height,
                step_width: stepwise.step_width,
                step_height: stepwise.step_height,
            }
        };

    debug!("supported_framesize: {:?}", supported_framesize);

    /*
    VIDIOC_G_FMT
    ==> _IOWR('V', 4, struct v4l2_format)
    ...
    ==> ((3U  << 30)  |
         ('V' << 8) |
         4 |
         (sizeof(v4l2_format) << 16))
    */
    /*
    let VIDIOC_G_FMT: libc::c_uint = ((3 as libc::c_uint) << 30)
        | ((b'V' as libc::c_uint) << 8)
        | (4 as libc::c_uint)
        | ((mem::size_of::<v4l::v4l2_format>() as libc::c_uint) << 16);

    xioctl(fd, VIDIOC_G_FMT, &mut fmt as *mut _ as *mut libc::c_void);
    */

    let min_frame = supported_framesize
        .iter()
        .min_by_key(|frm| frm.width * frm.height)
        .expect("there is no supported frame size");
    debug!("min_frame: {}", min_frame);
    let mut fmt = unsafe {
        let mut fmt: v4l::v4l2_format = mem::zeroed();
        fmt.type_ = v4l::v4l2_buf_type_V4L2_BUF_TYPE_VIDEO_CAPTURE;
        fmt.fmt.pix.width = min_frame.width;
        fmt.fmt.pix.height = min_frame.height;
        fmt.fmt.pix.pixelformat = V4L2_PIX_FMT_RGB24;
        fmt.fmt.pix.field = v4l::v4l2_field_V4L2_FIELD_INTERLACED;
        fmt
    };

    /*
    VIDIOC_S_FMT
    ==> _IOWR('V', 5, struct v4l2_format)
    ==> _IOC(_IOC_READ|_IOC_WRITE, ('V'), (5), (_IOC_TYPECHECK(struct v4l2_format)))
    ==> _IOC(       2U|_IOC_WRITE, ('V'), (5), (_IOC_TYPECHECK(struct v4l2_format)))
    ==> _IOC(       2U|        1U, ('V'), (5), (_IOC_TYPECHECK(struct v4l2_format)))
    ==> _IOC(       2U|        1U, ('V'), (5), (sizeof(struct v4l2_format)))
    ==> (((3U)  << _IOC_DIRSHIFT)  |
         (('V') << _IOC_TYPESHIFT) |
         ((5)   << _IOC_NRSHIFT)   |
         ((sizeof(v4l2_format)) << _IOC_SIZESHIFT))
    ==> (((3U)  << (_IOC_SIZESHIFT + _IOC_SIZEBITS))  |
         (('V') << (_IOC_NRSHIFT + _IOC_NRBITS)) |
         ((5)   << 0)   |
         ((sizeof(v4l2_format)) << (_IOC_TYPESHIFT + _IOC_TYPEBITS)))
    ==> (((3U)  << ((_IOC_TYPESHIFT  _IOC_TYPEBITS) + 14))  |
         (('V') << (0 + 8)) |
         ((5)   << 0)   |
         ((sizeof(v4l2_format)) << ((_IOC_NRSHIFT + _IOC_NRBITS) + 8)))
    ==> (((3U)  << (((_IOC_NRSHIFT + _IOC_NRBITS) + 8) + 14))  |
         (('V') << 8) |
         ((5)   << 0) |
         ((sizeof(v4l2_format)) << ((0 + 8) + 8)))
    ==> (((3U)  << (((0 + 8) + 8) + 14))  |
         (('V') << 8) |
         ((5)   << 0) |
         ((sizeof(v4l2_format)) << ((0 + 8) + 8)))
    ==> ((3U  << 30)  |
         ('V' << 8) |
         5 |
         (sizeof(v4l2_format) << 16))
    */
    let VIDIOC_S_FMT: libc::c_uint = ((3 as libc::c_uint) << 30)
        | ((b'V' as libc::c_uint) << 8)
        | (5 as libc::c_uint)
        | ((mem::size_of::<v4l::v4l2_format>() as libc::c_uint) << 16);

    xioctl(fd, VIDIOC_S_FMT, &mut fmt as *mut _ as *mut libc::c_void);
    if unsafe { fmt.fmt.pix.pixelformat != V4L2_PIX_FMT_RGB24 } {
        println!("Libv4l didn't accept RGB24 format. Can't proceed.");
        panic!()
    }
    if unsafe { (fmt.fmt.pix.width != 640) || (fmt.fmt.pix.height != 480) } {
        println!(
            "Warning: driver is sending image at {}x{}",
            unsafe { fmt.fmt.pix.width },
            unsafe { fmt.fmt.pix.height }
        );
    }

    let mut req = unsafe {
        let mut req: v4l::v4l2_requestbuffers = mem::zeroed();
        req.count = 2;
        req.type_ = v4l::v4l2_buf_type_V4L2_BUF_TYPE_VIDEO_CAPTURE;
        req.memory = v4l::v4l2_memory_V4L2_MEMORY_MMAP;
        req
    };

    /*
    VIDIOC_REQBUFS
    ==> _IOWR('V', 8, struct v4l2_requestbuffers)
    ==> _IOC(       2U|        1U, ('V'), (8), (sizeof(struct v4l2_requestbuffers)))
    ...
    ==> (((3U)  << _IOC_DIRSHIFT)  |
         (('V') << _IOC_TYPESHIFT) |
         ((5)   << _IOC_NRSHIFT)   |
         ((sizeof(v4l2_requestbuffers)) << _IOC_SIZESHIFT))
    ...
    ==> (((3U)  << (((0 + 8) + 8) + 14))  |
         (('V') << 8) |
         ((5)   << 0) |
         ((sizeof(v4l2_requestbuffers)) << ((0 + 8) + 8)))
    */
    let VIDIOC_REQBUFS: libc::c_uint = ((3 as libc::c_uint) << 30)
        | ((b'V' as libc::c_uint) << 8)
        | (8 as libc::c_uint)
        | ((mem::size_of::<v4l::v4l2_requestbuffers>() as libc::c_uint) << 16);
    xioctl(fd, VIDIOC_REQBUFS, &mut req as *mut _ as *mut libc::c_void);

    let mut buffers = Vec::new();
    debug!("req: {:?}", req);
    for n_buffers in 0..req.count {
        debug!("n_buffers: {}", n_buffers);
        let mut buf = unsafe {
            let mut buf: v4l::v4l2_buffer = mem::zeroed();
            buf.type_ = v4l::v4l2_buf_type_V4L2_BUF_TYPE_VIDEO_CAPTURE;
            buf.memory = v4l::v4l2_memory_V4L2_MEMORY_MMAP;
            buf.index = n_buffers;
            buf
        };
        /*
        VIDIOC_QUERYBUF
        ==> _IOWR('V', 9, struct v4l2_buffer)
        ==> _IOC(       2U|        1U, ('V'), (9), (sizeof(struct v4l2_buffer)))
        ...
        ==> (((3U)  << _IOC_DIRSHIFT)  |
             (('V') << _IOC_TYPESHIFT) |
             ((9)   << _IOC_NRSHIFT)   |
             ((sizeof(v4l2_buffer)) << _IOC_SIZESHIFT))
        ...
        ==> (((3U)  << (((0 + 8) + 8) + 14))  |
             (('V') << 8) |
             ((9)   << 0) |
             ((sizeof(v4l2_buffer)) << ((0 + 8) + 8)))
        */
        let VIDIOC_QUERYBUF: libc::c_uint = ((3 as libc::c_uint) << 30)
            | ((b'V' as libc::c_uint) << 8)
            | (9 as libc::c_uint)
            | ((mem::size_of::<v4l::v4l2_buffer>() as libc::c_uint) << 16);
        xioctl(fd, VIDIOC_QUERYBUF, &mut buf as *mut _ as *mut libc::c_void);
        debug!("buf.length: {}", buf.length);
        debug!("buf.m.offset: {}", unsafe { buf.m.offset });
        let buffer = unsafe {
            buffer {
                start: v4l::v4l2_mmap(
                    ptr::null_mut(),
                    buf.length as usize,
                    libc::PROT_READ | libc::PROT_WRITE,
                    libc::MAP_SHARED,
                    fd,
                    buf.m.offset as i64,
                ),
                length: buf.length as libc::size_t,
            }
        };

        if libc::MAP_FAILED == buffer.start {
            error!("mmap");
            panic!()
        }
        buffers.push(buffer);
    }

    for i in 0..buffers.len() {
        debug!("buffers: {}", i);
        let mut buf = unsafe {
            let mut buf: v4l::v4l2_buffer = mem::zeroed();
            buf.type_ = v4l::v4l2_buf_type_V4L2_BUF_TYPE_VIDEO_CAPTURE;
            buf.memory = v4l::v4l2_memory_V4L2_MEMORY_MMAP;
            buf.index = i as u32;
            buf
        };

        let VIDIOC_QBUF: libc::c_uint = ((3 as libc::c_uint) << 30)
            | ((b'V' as libc::c_uint) << 8)
            | (15 as libc::c_uint)
            | ((mem::size_of::<v4l::v4l2_buffer>() as libc::c_uint) << 16);
        debug!("VIDIOC_QBUF");
        xioctl(fd, VIDIOC_QBUF, &mut buf as *mut _ as *mut libc::c_void);
    }

    debug!("V4L2_BUF_TYPE_VIDEO_CAPTURE");
    let mut type_ = v4l::v4l2_buf_type_V4L2_BUF_TYPE_VIDEO_CAPTURE;
    /*
    VIDIOC_STREAMON
    ==> _IOW('V', 18, int)
    ==> _IOC(_IOC_WRITE, ('V'), (5), (_IOC_TYPECHECK(int)))
    ...
    ==> ((1U  << 30)  |
         ('V' << 8) |
         18 |
         (sizeof(int) << 16))
    */
    let VIDIOC_STREAMON: libc::c_uint = ((1 as libc::c_uint) << 30)
        | ((b'V' as libc::c_uint) << 8)
        | (18 as libc::c_uint)
        | ((mem::size_of::<libc::c_int>() as libc::c_uint) << 16);
    debug!("VIDIOC_STREAMON: {:0X}", VIDIOC_STREAMON);
    xioctl(
        fd,
        VIDIOC_STREAMON,
        &mut type_ as *mut _ as *mut libc::c_void,
    );

    for i in 0..20 {
        debug!("0..20: {}", i);

        unsafe {
            let mut fds: libc::fd_set = mem::zeroed();
            loop {
                libc::FD_ZERO(&mut fds);
                libc::FD_SET(fd, &mut fds);
                let mut tv = libc::timeval {
                    tv_sec: 2,
                    tv_usec: 0,
                };
                let r = libc::select(fd + 1, &mut fds, ptr::null_mut(), ptr::null_mut(), &mut tv);
                if !(r == -1 && (errno!() == libc::EINTR)) {
                    if cfg!(target_os = "linux") {
                        debug!("time left: {}.{:06}", tv.tv_sec, tv.tv_usec);
                    }
                    break;
                }
                if r == -1 {
                    error!("select error {}, {}", errno!(), strerror());
                    panic!()
                }
            }
        }

        let mut buf = unsafe {
            let mut buf: v4l::v4l2_buffer = mem::zeroed();
            buf.type_ = v4l::v4l2_buf_type_V4L2_BUF_TYPE_VIDEO_CAPTURE;
            buf.memory = v4l::v4l2_memory_V4L2_MEMORY_MMAP;
            buf
        };

        let VIDIOC_DQBUF: libc::c_uint = ((3 as libc::c_uint) << 30)
            | ((b'V' as libc::c_uint) << 8)
            | (17 as libc::c_uint)
            | ((mem::size_of::<v4l::v4l2_buffer>() as libc::c_uint) << 16);
        debug!("VIDIOC_DQBUF");
        xioctl(fd, VIDIOC_DQBUF, &mut buf as *mut _ as *mut libc::c_void);

        {
            let mut fout = fs::File::create(&format!("out{:03}.ppm", i)).unwrap();
            write!(
                fout,
                "P6\n{} {} 255\n",
                unsafe { fmt.fmt.pix.width },
                unsafe { fmt.fmt.pix.height }
            );

            unsafe {
                fout.write_all(slice::from_raw_parts(
                    buffers[buf.index as usize].start as *const u8,
                    buf.bytesused as usize,
                ))
                .unwrap();
            }
        }

        let VIDIOC_QBUF: libc::c_uint = ((3 as libc::c_uint) << 30)
            | ((b'V' as libc::c_uint) << 8)
            | (15 as libc::c_uint)
            | ((mem::size_of::<v4l::v4l2_buffer>() as libc::c_uint) << 16);
        debug!("VIDIOC_QBUF");
        xioctl(fd, VIDIOC_QBUF, &mut buf as *mut _ as *mut libc::c_void);
    }

    let mut type_ = v4l::v4l2_buf_type_V4L2_BUF_TYPE_VIDEO_CAPTURE;
    /*
    VIDIOC_STREAMON
    ==> _IOW('V', 19, int)
    ==> _IOC(_IOC_WRITE, ('V'), (5), (_IOC_TYPECHECK(int)))
    ...
    ==> ((1U  << 30)  |
         ('V' << 8) |
         19 |
         (sizeof(int) << 16))
    */
    let VIDIOC_STREAMOFF: libc::c_uint = ((1 as libc::c_uint) << 30)
        | ((b'V' as libc::c_uint) << 8)
        | (19 as libc::c_uint)
        | ((mem::size_of::<libc::c_int>() as libc::c_uint) << 16);
    debug!("VIDIOC_STREAMOFF");
    xioctl(
        fd,
        VIDIOC_STREAMOFF,
        &mut type_ as *mut _ as *mut libc::c_void,
    );

    unsafe {
        for buf in buffers {
            v4l::v4l2_munmap(buf.start, buf.length);
        }
        v4l::v4l2_close(fd);
    }
}
