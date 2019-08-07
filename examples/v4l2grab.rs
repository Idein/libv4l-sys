use std::mem;
use std::ffi::CStr;

use log::*;

use libv4l_sys as v4l;

macro_rules! errno {
    () => (unsafe { *libc::__errno_location() })
}

fn strerror() -> String {
    let errno = errno!();
    unsafe {
        CStr::from_ptr(libc::strerror(errno))
    }.to_string_lossy().into()
}

fn xioctl(fd: libc::c_int, request: libc::c_uint, arg: *mut libc::c_void) {
    let mut r = 0;

    while r == -1 && ((errno!() == libc::EINTR) || (errno!() == libc::EAGAIN)) {
        r = unsafe { v4l::v4l2_ioctl(fd, request, arg) };
    }

    if r == -1 {
        error!("error {}, {}", errno!(), strerror());
        panic!()
    }
}

fn main() {
    println!("v4l2grab");
    env_logger::init();

    let dev_name = b"/dev/video0";

    let fd = unsafe {
        let fd = v4l::v4l2_open(dev_name.as_ptr(), libc::O_RDWR | libc::O_NONBLOCK, 0);
        if fd == -1 {
            error!("open device error: {}: {}", fd, strerror());
            panic!()
        }
        fd
    };

    let V4L2_PIX_FMT_RGB24: u32 = 
        ((b'R' as u32) <<  0) |
        ((b'G' as u32) <<  8) |
        ((b'B' as u32) << 16) |
        ((b'3' as u32) << 24);

    let mut fmt = unsafe {
        let mut fmt:v4l::v4l2_format = mem::zeroed();
        fmt.type_ = v4l::v4l2_buf_type_V4L2_BUF_TYPE_VIDEO_CAPTURE;
        fmt.fmt.pix.width       = 640;
        fmt.fmt.pix.height      = 480;
        fmt.fmt.pix.pixelformat = V4L2_PIX_FMT_RGB24;
        fmt.fmt.pix.field       = v4l::v4l2_field_V4L2_FIELD_INTERLACED;
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
    let VIDIOC_S_FMT: libc::c_uint =
        ((  3  as libc::c_uint) << 30) |
        ((b'V' as libc::c_uint) <<  8) |
        (   5  as libc::c_uint       ) |
        ((mem::size_of::<v4l::v4l2_format>() as libc::c_uint) << 16)
        ;

    xioctl(fd, VIDIOC_S_FMT, &mut fmt as *mut _ as *mut libc::c_void);
    /*
    if (fmt.fmt.pix.pixelformat != V4L2_PIX_FMT_RGB24) {
            printf("Libv4l didn't accept RGB24 format. Can't proceed.\n");
            exit(EXIT_FAILURE);
    }
    if ((fmt.fmt.pix.width != 640) || (fmt.fmt.pix.height != 480))
            printf("Warning: driver is sending image at %dx%d\n",
                            fmt.fmt.pix.width, fmt.fmt.pix.height);
    */
}
