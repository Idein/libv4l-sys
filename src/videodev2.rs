///! import linux/videodev2.h

/// ioctl codes for video devices
/// ref. https://www.linuxtv.org/downloads/v4l-dvb-apis-new/uapi/v4l/user-func.html
pub mod codes {
    const VIDEODEV2_IOC_MAGIC: u8 = b'V';

    pub const VIDIOC_QUERYCAP: libc::c_ulong = ior!(VIDEODEV2_IOC_MAGIC, 0, crate::v4l2_capability);
    pub const VIDIOC_RESERVED: libc::c_ulong = io!(VIDEODEV2_IOC_MAGIC, 1);
    pub const VIDIOC_ENUM_FMT: libc::c_ulong = iowr!(VIDEODEV2_IOC_MAGIC, 2, crate::v4l2_fmtdesc);
    /// These ioctls are used to negotiate the format of data (typically image format) exchanged between driver and application.
    pub const VIDIOC_G_FMT: libc::c_ulong = iowr!(VIDEODEV2_IOC_MAGIC, 4, crate::v4l2_format);
    /// These ioctls are used to negotiate the format of data (typically image format) exchanged between driver and application.
    pub const VIDIOC_S_FMT: libc::c_ulong = iowr!(VIDEODEV2_IOC_MAGIC, 5, crate::v4l2_format);
    /// allocates the desired number of buffers, this is a required step in the initialization sequence.
    pub const VIDIOC_REQBUFS: libc::c_ulong =
        iowr!(VIDEODEV2_IOC_MAGIC, 8, crate::v4l2_requestbuffers);
    /// Buffers are individually mapped. The offset and size of each buffer can be determined with the ioctl VIDIOC_QUERYBUF ioctl.
    pub const VIDIOC_QUERYBUF: libc::c_ulong = iowr!(VIDEODEV2_IOC_MAGIC, 9, crate::v4l2_buffer);
    pub const VIDIOC_G_FBUF: libc::c_ulong = ior!(VIDEODEV2_IOC_MAGIC, 10, crate::v4l2_framebuffer);
    pub const VIDIOC_S_FBUF: libc::c_ulong = iow!(VIDEODEV2_IOC_MAGIC, 11, crate::v4l2_framebuffer);
    pub const VIDIOC_OVERLAY: libc::c_ulong = iow!(VIDEODEV2_IOC_MAGIC, 14, ::std::os::raw::c_int);
    /// Applications call the VIDIOC_QBUF ioctl to enqueue an empty (capturing) or filled (output) buffer in the driver’s incoming queue.
    /// The semantics depend on the selected I/O method.
    pub const VIDIOC_QBUF: libc::c_ulong = iowr!(VIDEODEV2_IOC_MAGIC, 15, crate::v4l2_buffer);
    pub const VIDIOC_EXPBUF: libc::c_ulong =
        iowr!(VIDEODEV2_IOC_MAGIC, 16, crate::v4l2_exportbuffer);
    /// Applications call the VIDIOC_QBUF ioctl to enqueue an empty (capturing) or filled (output) buffer in the driver’s incoming queue.
    /// The semantics depend on the selected I/O method.
    pub const VIDIOC_DQBUF: libc::c_ulong = iowr!(VIDEODEV2_IOC_MAGIC, 17, crate::v4l2_buffer);
    pub const VIDIOC_STREAMON: libc::c_ulong = iow!(VIDEODEV2_IOC_MAGIC, 18, ::std::os::raw::c_int);
    pub const VIDIOC_STREAMOFF: libc::c_ulong =
        iow!(VIDEODEV2_IOC_MAGIC, 19, ::std::os::raw::c_int);
    pub const VIDIOC_G_PARM: libc::c_ulong = iowr!(VIDEODEV2_IOC_MAGIC, 21, crate::v4l2_streamparm);
    pub const VIDIOC_S_PARM: libc::c_ulong = iowr!(VIDEODEV2_IOC_MAGIC, 22, crate::v4l2_streamparm);
    pub const VIDIOC_G_STD: libc::c_ulong = ior!(VIDEODEV2_IOC_MAGIC, 23, crate::v4l2_std_id);
    pub const VIDIOC_S_STD: libc::c_ulong = iow!(VIDEODEV2_IOC_MAGIC, 24, crate::v4l2_std_id);
    pub const VIDIOC_ENUMSTD: libc::c_ulong = iowr!(VIDEODEV2_IOC_MAGIC, 25, crate::v4l2_standard);
    pub const VIDIOC_ENUMINPUT: libc::c_ulong = iowr!(VIDEODEV2_IOC_MAGIC, 26, crate::v4l2_input);
    pub const VIDIOC_G_CTRL: libc::c_ulong = iowr!(VIDEODEV2_IOC_MAGIC, 27, crate::v4l2_control);
    pub const VIDIOC_S_CTRL: libc::c_ulong = iowr!(VIDEODEV2_IOC_MAGIC, 28, crate::v4l2_control);
    pub const VIDIOC_G_TUNER: libc::c_ulong = iowr!(VIDEODEV2_IOC_MAGIC, 29, crate::v4l2_tuner);
    pub const VIDIOC_S_TUNER: libc::c_ulong = iow!(VIDEODEV2_IOC_MAGIC, 30, crate::v4l2_tuner);
    pub const VIDIOC_G_AUDIO: libc::c_ulong = ior!(VIDEODEV2_IOC_MAGIC, 33, crate::v4l2_audio);
    pub const VIDIOC_S_AUDIO: libc::c_ulong = iow!(VIDEODEV2_IOC_MAGIC, 34, crate::v4l2_audio);
    pub const VIDIOC_QUERYCTRL: libc::c_ulong =
        iowr!(VIDEODEV2_IOC_MAGIC, 36, crate::v4l2_queryctrl);
    pub const VIDIOC_QUERYMENU: libc::c_ulong =
        iowr!(VIDEODEV2_IOC_MAGIC, 37, crate::v4l2_querymenu);
    pub const VIDIOC_G_INPUT: libc::c_ulong = ior!(VIDEODEV2_IOC_MAGIC, 38, ::std::os::raw::c_int);
    pub const VIDIOC_S_INPUT: libc::c_ulong = iowr!(VIDEODEV2_IOC_MAGIC, 39, ::std::os::raw::c_int);
    pub const VIDIOC_G_EDID: libc::c_ulong = iowr!(VIDEODEV2_IOC_MAGIC, 40, crate::v4l2_edid);
    pub const VIDIOC_S_EDID: libc::c_ulong = iowr!(VIDEODEV2_IOC_MAGIC, 41, crate::v4l2_edid);
    pub const VIDIOC_G_OUTPUT: libc::c_ulong = ior!(VIDEODEV2_IOC_MAGIC, 46, ::std::os::raw::c_int);
    pub const VIDIOC_S_OUTPUT: libc::c_ulong =
        iowr!(VIDEODEV2_IOC_MAGIC, 47, ::std::os::raw::c_int);
    pub const VIDIOC_ENUMOUTPUT: libc::c_ulong = iowr!(VIDEODEV2_IOC_MAGIC, 48, crate::v4l2_output);
    pub const VIDIOC_G_AUDOUT: libc::c_ulong = ior!(VIDEODEV2_IOC_MAGIC, 49, crate::v4l2_audioout);
    pub const VIDIOC_S_AUDOUT: libc::c_ulong = iow!(VIDEODEV2_IOC_MAGIC, 50, crate::v4l2_audioout);
    pub const VIDIOC_G_MODULATOR: libc::c_ulong =
        iowr!(VIDEODEV2_IOC_MAGIC, 54, crate::v4l2_modulator);
    pub const VIDIOC_S_MODULATOR: libc::c_ulong =
        iow!(VIDEODEV2_IOC_MAGIC, 55, crate::v4l2_modulator);
    pub const VIDIOC_G_FREQUENCY: libc::c_ulong =
        iowr!(VIDEODEV2_IOC_MAGIC, 56, crate::v4l2_frequency);
    pub const VIDIOC_S_FREQUENCY: libc::c_ulong =
        iow!(VIDEODEV2_IOC_MAGIC, 57, crate::v4l2_frequency);
    pub const VIDIOC_CROPCAP: libc::c_ulong = iowr!(VIDEODEV2_IOC_MAGIC, 58, crate::v4l2_cropcap);
    pub const VIDIOC_G_CROP: libc::c_ulong = iowr!(VIDEODEV2_IOC_MAGIC, 59, crate::v4l2_crop);
    pub const VIDIOC_S_CROP: libc::c_ulong = iow!(VIDEODEV2_IOC_MAGIC, 60, crate::v4l2_crop);
    pub const VIDIOC_G_JPEGCOMP: libc::c_ulong =
        ior!(VIDEODEV2_IOC_MAGIC, 61, crate::v4l2_jpegcompression);
    pub const VIDIOC_S_JPEGCOMP: libc::c_ulong =
        iow!(VIDEODEV2_IOC_MAGIC, 62, crate::v4l2_jpegcompression);
    pub const VIDIOC_QUERYSTD: libc::c_ulong = ior!(VIDEODEV2_IOC_MAGIC, 63, crate::v4l2_std_id);
    pub const VIDIOC_TRY_FMT: libc::c_ulong = iowr!(VIDEODEV2_IOC_MAGIC, 64, crate::v4l2_format);
    pub const VIDIOC_ENUMAUDIO: libc::c_ulong = iowr!(VIDEODEV2_IOC_MAGIC, 65, crate::v4l2_audio);
    pub const VIDIOC_ENUMAUDOUT: libc::c_ulong =
        iowr!(VIDEODEV2_IOC_MAGIC, 66, crate::v4l2_audioout);
    pub const VIDIOC_G_PRIORITY: libc::c_ulong = ior!(VIDEODEV2_IOC_MAGIC, 67, crate::__u32) /* enum v4l2_priority */;
    pub const VIDIOC_S_PRIORITY: libc::c_ulong = iow!(VIDEODEV2_IOC_MAGIC, 68, crate::__u32) /* enum v4l2_priority */;
    pub const VIDIOC_G_SLICED_VBI_CAP: libc::c_ulong =
        iowr!(VIDEODEV2_IOC_MAGIC, 69, crate::v4l2_sliced_vbi_cap);
    pub const VIDIOC_LOG_STATUS: libc::c_ulong = io!(VIDEODEV2_IOC_MAGIC, 70);
    pub const VIDIOC_G_EXT_CTRLS: libc::c_ulong =
        iowr!(VIDEODEV2_IOC_MAGIC, 71, crate::v4l2_ext_controls);
    pub const VIDIOC_S_EXT_CTRLS: libc::c_ulong =
        iowr!(VIDEODEV2_IOC_MAGIC, 72, crate::v4l2_ext_controls);
    pub const VIDIOC_TRY_EXT_CTRLS: libc::c_ulong =
        iowr!(VIDEODEV2_IOC_MAGIC, 73, crate::v4l2_ext_controls);
    pub const VIDIOC_ENUM_FRAMESIZES: libc::c_ulong =
        iowr!(VIDEODEV2_IOC_MAGIC, 74, crate::v4l2_frmsizeenum);
    pub const VIDIOC_ENUM_FRAMEINTERVALS: libc::c_ulong =
        iowr!(VIDEODEV2_IOC_MAGIC, 75, crate::v4l2_frmivalenum);
    pub const VIDIOC_G_ENC_INDEX: libc::c_ulong =
        ior!(VIDEODEV2_IOC_MAGIC, 76, crate::v4l2_enc_idx);
    pub const VIDIOC_ENCODER_CMD: libc::c_ulong =
        iowr!(VIDEODEV2_IOC_MAGIC, 77, crate::v4l2_encoder_cmd);
    pub const VIDIOC_TRY_ENCODER_CMD: libc::c_ulong =
        iowr!(VIDEODEV2_IOC_MAGIC, 78, crate::v4l2_encoder_cmd);
}

/// Construct four-character-code (FOURCC)
#[macro_export]
macro_rules! fourcc {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        ($a as u32) | (($b as u32) << 8) | (($c as u32) << 16) | (($d as u32) << 24)
    };
}

#[macro_export]
macro_rules! fourcc_be {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        fourcc!($a, $b, $c, $d) | ((1 as u32) << 31)
    };
}

pub mod pixel_format {
    /// Pixel format         FOURCC                          depth  Description

    /// RGB formats
    mod rgb {
        ///   8  RGB-3-3-2
        pub const V4L2_PIX_FMT_RGB332: u32 = fourcc!(b'R', b'G', b'B', b'1');
        ///  16  xxxxrrrr ggggbbbb
        pub const V4L2_PIX_FMT_RGB444: u32 = fourcc!(b'R', b'4', b'4', b'4');
        ///  16  aaaarrrr ggggbbbb
        pub const V4L2_PIX_FMT_ARGB444: u32 = fourcc!(b'A', b'R', b'1', b'2');
        ///  16  xxxxrrrr ggggbbbb
        pub const V4L2_PIX_FMT_XRGB444: u32 = fourcc!(b'X', b'R', b'1', b'2');
        ///  16  RGB-5-5-5
        pub const V4L2_PIX_FMT_RGB555: u32 = fourcc!(b'R', b'G', b'B', b'O');
        ///  16  ARGB-1-5-5-5
        pub const V4L2_PIX_FMT_ARGB555: u32 = fourcc!(b'A', b'R', b'1', b'5');
        ///  16  XRGB-1-5-5-5
        pub const V4L2_PIX_FMT_XRGB555: u32 = fourcc!(b'X', b'R', b'1', b'5');
        ///  16  RGB-5-6-5
        pub const V4L2_PIX_FMT_RGB565: u32 = fourcc!(b'R', b'G', b'B', b'P');
        ///  16  RGB-5-5-5 BE
        pub const V4L2_PIX_FMT_RGB555X: u32 = fourcc!(b'R', b'G', b'B', b'Q');
        ///  16  ARGB-5-5-5 BE
        pub const V4L2_PIX_FMT_ARGB555X: u32 = fourcc_be!(b'A', b'R', b'1', b'5');
        ///  16  XRGB-5-5-5 BE
        pub const V4L2_PIX_FMT_XRGB555X: u32 = fourcc_be!(b'X', b'R', b'1', b'5');
        ///  16  RGB-5-6-5 BE
        pub const V4L2_PIX_FMT_RGB565X: u32 = fourcc!(b'R', b'G', b'B', b'R');
        ///  18  BGR-6-6-6
        pub const V4L2_PIX_FMT_BGR666: u32 = fourcc!(b'B', b'G', b'R', b'H');
        ///  24  BGR-8-8-8
        pub const V4L2_PIX_FMT_BGR24: u32 = fourcc!(b'B', b'G', b'R', b'3');
        ///  24  RGB-8-8-8
        pub const V4L2_PIX_FMT_RGB24: u32 = fourcc!(b'R', b'G', b'B', b'3');
        ///  32  BGR-8-8-8-8
        pub const V4L2_PIX_FMT_BGR32: u32 = fourcc!(b'B', b'G', b'R', b'4');
        ///  32  BGRA-8-8-8-8
        pub const V4L2_PIX_FMT_ABGR32: u32 = fourcc!(b'A', b'R', b'2', b'4');
        ///  32  BGRX-8-8-8-8
        pub const V4L2_PIX_FMT_XBGR32: u32 = fourcc!(b'X', b'R', b'2', b'4');
        ///  32  RGB-8-8-8-8
        pub const V4L2_PIX_FMT_RGB32: u32 = fourcc!(b'R', b'G', b'B', b'4');
        ///  32  ARGB-8-8-8-8
        pub const V4L2_PIX_FMT_ARGB32: u32 = fourcc!(b'B', b'A', b'2', b'4');
        ///  32  XRGB-8-8-8-8
        pub const V4L2_PIX_FMT_XRGB32: u32 = fourcc!(b'B', b'X', b'2', b'4');
    }
    pub use rgb::*;

    /// Grey formats
    mod grey {
        ///   8  Greyscale
        pub const V4L2_PIX_FMT_GREY: u32 = fourcc!(b'G', b'R', b'E', b'Y');
        ///   4  Greyscale
        pub const V4L2_PIX_FMT_Y4: u32 = fourcc!(b'Y', b'0', b'4', b' ');
        ///   6  Greyscale
        pub const V4L2_PIX_FMT_Y6: u32 = fourcc!(b'Y', b'0', b'6', b' ');
        ///  10  Greyscale
        pub const V4L2_PIX_FMT_Y10: u32 = fourcc!(b'Y', b'1', b'0', b' ');
        ///  12  Greyscale
        pub const V4L2_PIX_FMT_Y12: u32 = fourcc!(b'Y', b'1', b'2', b' ');
        ///  16  Greyscale
        pub const V4L2_PIX_FMT_Y16: u32 = fourcc!(b'Y', b'1', b'6', b' ');
        ///  16  Greyscale BE
        pub const V4L2_PIX_FMT_Y16_BE: u32 = fourcc_be!(b'Y', b'1', b'6', b' ');
    }
    pub use grey::*;

    /// Grey bit-packed formats
    mod grey_bit_packed {
        ///  10  Greyscale bit-packed
        pub const V4L2_PIX_FMT_Y10BPACK: u32 = fourcc!(b'Y', b'1', b'0', b'B');
    }
    pub use grey_bit_packed::*;

    /// Palette formats
    mod palette {
        ///   8  8-bit palette
        pub const V4L2_PIX_FMT_PAL8: u32 = fourcc!(b'P', b'A', b'L', b'8');
    }
    pub use palette::*;

    /// Chrominance formats
    mod chrominance {
        ///   8  UV 4:4
        pub const V4L2_PIX_FMT_UV8: u32 = fourcc!(b'U', b'V', b'8', b' ');
    }
    pub use chrominance::*;

    /// Luminance+Chrominance formats
    mod luminance_chrominance {
        ///  16  YUV 4:2:2
        pub const V4L2_PIX_FMT_YUYV: u32 = fourcc!(b'Y', b'U', b'Y', b'V');
        ///  16  YUV 4:2:2
        pub const V4L2_PIX_FMT_YYUV: u32 = fourcc!(b'Y', b'Y', b'U', b'V');
        ///  16 YVU 4:2:2
        pub const V4L2_PIX_FMT_YVYU: u32 = fourcc!(b'Y', b'V', b'Y', b'U');
        ///  16  YUV 4:2:2
        pub const V4L2_PIX_FMT_UYVY: u32 = fourcc!(b'U', b'Y', b'V', b'Y');
        ///  16  YUV 4:2:2
        pub const V4L2_PIX_FMT_VYUY: u32 = fourcc!(b'V', b'Y', b'U', b'Y');
        ///  12  YUV 4:1:1
        pub const V4L2_PIX_FMT_Y41P: u32 = fourcc!(b'Y', b'4', b'1', b'P');
        ///  16  xxxxyyyy uuuuvvvv
        pub const V4L2_PIX_FMT_YUV444: u32 = fourcc!(b'Y', b'4', b'4', b'4');
        ///  16  YUV-5-5-5
        pub const V4L2_PIX_FMT_YUV555: u32 = fourcc!(b'Y', b'U', b'V', b'O');
        ///  16  YUV-5-6-5
        pub const V4L2_PIX_FMT_YUV565: u32 = fourcc!(b'Y', b'U', b'V', b'P');
        ///  32  YUV-8-8-8-8
        pub const V4L2_PIX_FMT_YUV32: u32 = fourcc!(b'Y', b'U', b'V', b'4');
        ///   8  8-bit color
        pub const V4L2_PIX_FMT_HI240: u32 = fourcc!(b'H', b'I', b'2', b'4');
        ///   8  YUV 4:2:0 16x16 macroblocks
        pub const V4L2_PIX_FMT_HM12: u32 = fourcc!(b'H', b'M', b'1', b'2');
        ///  12  YUV 4:2:0 2 lines y, 1 line uv interleaved
        pub const V4L2_PIX_FMT_M420: u32 = fourcc!(b'M', b'4', b'2', b'0');
    }
    pub use luminance_chrominance::*;

    /// two planes -- one Y, one Cr + Cb interleaved
    mod two_planes {
        ///  12  Y/CbCr 4:2:0
        pub const V4L2_PIX_FMT_NV12: u32 = fourcc!(b'N', b'V', b'1', b'2');
        ///  12  Y/CrCb 4:2:0
        pub const V4L2_PIX_FMT_NV21: u32 = fourcc!(b'N', b'V', b'2', b'1');
        ///  16  Y/CbCr 4:2:2
        pub const V4L2_PIX_FMT_NV16: u32 = fourcc!(b'N', b'V', b'1', b'6');
        ///  16  Y/CrCb 4:2:2
        pub const V4L2_PIX_FMT_NV61: u32 = fourcc!(b'N', b'V', b'6', b'1');
        ///  24  Y/CbCr 4:4:4
        pub const V4L2_PIX_FMT_NV24: u32 = fourcc!(b'N', b'V', b'2', b'4');
        ///  24  Y/CrCb 4:4:4
        pub const V4L2_PIX_FMT_NV42: u32 = fourcc!(b'N', b'V', b'4', b'2');
    }
    pub use two_planes::*;

    /// two non contiguous planes - one Y, one Cr + Cb interleaved
    mod two_non_contiguous_planes {
        ///  12  Y/CbCr 4:2:0
        pub const V4L2_PIX_FMT_NV12M: u32 = fourcc!(b'N', b'M', b'1', b'2');
        ///  21  Y/CrCb 4:2:0
        pub const V4L2_PIX_FMT_NV21M: u32 = fourcc!(b'N', b'M', b'2', b'1');
        ///  16  Y/CbCr 4:2:2
        pub const V4L2_PIX_FMT_NV16M: u32 = fourcc!(b'N', b'M', b'1', b'6');
        ///  16  Y/CrCb 4:2:2
        pub const V4L2_PIX_FMT_NV61M: u32 = fourcc!(b'N', b'M', b'6', b'1');
        ///  12  Y/CbCr 4:2:0 64x32 macroblocks
        pub const V4L2_PIX_FMT_NV12MT: u32 = fourcc!(b'T', b'M', b'1', b'2');
        ///  12  Y/CbCr 4:2:0 16x16 macroblocks
        pub const V4L2_PIX_FMT_NV12MT_16X16: u32 = fourcc!(b'V', b'M', b'1', b'2');
    }
    pub use two_non_contiguous_planes::*;

    /// three planes - Y Cb, Cr
    mod three_plances {
        ///   9  YUV 4:1:0
        pub const V4L2_PIX_FMT_YUV410: u32 = fourcc!(b'Y', b'U', b'V', b'9');
        ///   9  YVU 4:1:0
        pub const V4L2_PIX_FMT_YVU410: u32 = fourcc!(b'Y', b'V', b'U', b'9');
        ///  12  YVU411 planar
        pub const V4L2_PIX_FMT_YUV411P: u32 = fourcc!(b'4', b'1', b'1', b'P');
        ///  12  YUV 4:2:0
        pub const V4L2_PIX_FMT_YUV420: u32 = fourcc!(b'Y', b'U', b'1', b'2');
        ///  12  YVU 4:2:0
        pub const V4L2_PIX_FMT_YVU420: u32 = fourcc!(b'Y', b'V', b'1', b'2');
        ///  16  YVU422 planar
        pub const V4L2_PIX_FMT_YUV422P: u32 = fourcc!(b'4', b'2', b'2', b'P');
    }
    pub use three_plances::*;

    /// three non contiguous planes - Y, Cb, Cr
    mod three_non_contiguous_planes {
        ///  12  YUV420 planar
        pub const V4L2_PIX_FMT_YUV420M: u32 = fourcc!(b'Y', b'M', b'1', b'2');
        ///  12  YVU420 planar
        pub const V4L2_PIX_FMT_YVU420M: u32 = fourcc!(b'Y', b'M', b'2', b'1');
        ///  16  YUV422 planar
        pub const V4L2_PIX_FMT_YUV422M: u32 = fourcc!(b'Y', b'M', b'1', b'6');
        ///  16  YVU422 planar
        pub const V4L2_PIX_FMT_YVU422M: u32 = fourcc!(b'Y', b'M', b'6', b'1');
        ///  24  YUV444 planar
        pub const V4L2_PIX_FMT_YUV444M: u32 = fourcc!(b'Y', b'M', b'2', b'4');
        ///  24  YVU444 planar
        pub const V4L2_PIX_FMT_YVU444M: u32 = fourcc!(b'Y', b'M', b'4', b'2');
    }
    pub use three_non_contiguous_planes::*;

    /// Bayer formats - see http://www.siliconimaging.com/RGB%20Bayer.htm
    mod bayer {
        ///   8  BGBG.. GRGR..
        pub const V4L2_PIX_FMT_SBGGR8: u32 = fourcc!(b'B', b'A', b'8', b'1');
        ///   8  GBGB.. RGRG..
        pub const V4L2_PIX_FMT_SGBRG8: u32 = fourcc!(b'G', b'B', b'R', b'G');
        ///   8  GRGR.. BGBG..
        pub const V4L2_PIX_FMT_SGRBG8: u32 = fourcc!(b'G', b'R', b'B', b'G');
        ///   8  RGRG.. GBGB..
        pub const V4L2_PIX_FMT_SRGGB8: u32 = fourcc!(b'R', b'G', b'G', b'B');
        ///  10  BGBG.. GRGR..
        pub const V4L2_PIX_FMT_SBGGR10: u32 = fourcc!(b'B', b'G', b'1', b'0');
        ///  10  GBGB.. RGRG..
        pub const V4L2_PIX_FMT_SGBRG10: u32 = fourcc!(b'G', b'B', b'1', b'0');
        ///  10  GRGR.. BGBG..
        pub const V4L2_PIX_FMT_SGRBG10: u32 = fourcc!(b'B', b'A', b'1', b'0');
        ///  10  RGRG.. GBGB..
        pub const V4L2_PIX_FMT_SRGGB10: u32 = fourcc!(b'R', b'G', b'1', b'0');
        /// 10bit raw bayer packed, 5 bytes for every 4 pixels
        mod raw_bayer_packed_10bit {
            pub const V4L2_PIX_FMT_SBGGR10P: u32 = fourcc!(b'p', b'B', b'A', b'A');
            pub const V4L2_PIX_FMT_SGBRG10P: u32 = fourcc!(b'p', b'G', b'A', b'A');
            pub const V4L2_PIX_FMT_SGRBG10P: u32 = fourcc!(b'p', b'g', b'A', b'A');
            pub const V4L2_PIX_FMT_SRGGB10P: u32 = fourcc!(b'p', b'R', b'A', b'A');
        }
        pub use raw_bayer_packed_10bit::*;
        /// 10bit raw bayer a-law compressed to 8 bits
        mod raw_bayer_law_compressed {
            pub const V4L2_PIX_FMT_SBGGR10ALAW8: u32 = fourcc!(b'a', b'B', b'A', b'8');
            pub const V4L2_PIX_FMT_SGBRG10ALAW8: u32 = fourcc!(b'a', b'G', b'A', b'8');
            pub const V4L2_PIX_FMT_SGRBG10ALAW8: u32 = fourcc!(b'a', b'g', b'A', b'8');
            pub const V4L2_PIX_FMT_SRGGB10ALAW8: u32 = fourcc!(b'a', b'R', b'A', b'8');
        }
        pub use raw_bayer_law_compressed::*;
        /// 10bit raw bayer DPCM compressed to 8 bits
        mod raw_bayer_dpcm_compressed {
            pub const V4L2_PIX_FMT_SBGGR10DPCM8: u32 = fourcc!(b'b', b'B', b'A', b'8');
            pub const V4L2_PIX_FMT_SGBRG10DPCM8: u32 = fourcc!(b'b', b'G', b'A', b'8');
            pub const V4L2_PIX_FMT_SGRBG10DPCM8: u32 = fourcc!(b'B', b'D', b'1', b'0');
            pub const V4L2_PIX_FMT_SRGGB10DPCM8: u32 = fourcc!(b'b', b'R', b'A', b'8');
            ///  12  BGBG.. GRGR..
            pub const V4L2_PIX_FMT_SBGGR12: u32 = fourcc!(b'B', b'G', b'1', b'2');
            ///  12  GBGB.. RGRG..
            pub const V4L2_PIX_FMT_SGBRG12: u32 = fourcc!(b'G', b'B', b'1', b'2');
            ///  12  GRGR.. BGBG..
            pub const V4L2_PIX_FMT_SGRBG12: u32 = fourcc!(b'B', b'A', b'1', b'2');
            ///  12  RGRG.. GBGB..
            pub const V4L2_PIX_FMT_SRGGB12: u32 = fourcc!(b'R', b'G', b'1', b'2');
            ///  16  BGBG.. GRGR..
            pub const V4L2_PIX_FMT_SBGGR16: u32 = fourcc!(b'B', b'Y', b'R', b'2');
            ///  16  GBGB.. RGRG..
            pub const V4L2_PIX_FMT_SGBRG16: u32 = fourcc!(b'G', b'B', b'1', b'6');
            ///  16  GRGR.. BGBG..
            pub const V4L2_PIX_FMT_SGRBG16: u32 = fourcc!(b'G', b'R', b'1', b'6');
            ///  16  RGRG.. GBGB..
            pub const V4L2_PIX_FMT_SRGGB16: u32 = fourcc!(b'R', b'G', b'1', b'6');
        }
        pub use raw_bayer_dpcm_compressed::*;
    }
    pub use bayer::*;

    /// HSV formats
    mod hsv {
        pub const V4L2_PIX_FMT_HSV24: u32 = fourcc!(b'H', b'S', b'V', b'3');
        pub const V4L2_PIX_FMT_HSV32: u32 = fourcc!(b'H', b'S', b'V', b'4');
    }
    pub use hsv::*;

    /// compressed formats
    mod compressed {
        ///  Motion-JPEG
        pub const V4L2_PIX_FMT_MJPEG: u32 = fourcc!(b'M', b'J', b'P', b'G');
        ///  JFIF JPEG
        pub const V4L2_PIX_FMT_JPEG: u32 = fourcc!(b'J', b'P', b'E', b'G');
        ///  1394
        pub const V4L2_PIX_FMT_DV: u32 = fourcc!(b'd', b'v', b's', b'd');
        ///  MPEG-1/2/4 Multiplexed
        pub const V4L2_PIX_FMT_MPEG: u32 = fourcc!(b'M', b'P', b'E', b'G');
        ///  H264 with start codes
        pub const V4L2_PIX_FMT_H264: u32 = fourcc!(b'H', b'2', b'6', b'4');
        ///  H264 without start codes
        pub const V4L2_PIX_FMT_H264_NO_SC: u32 = fourcc!(b'A', b'V', b'C', b'1');
        ///  H264 MVC
        pub const V4L2_PIX_FMT_H264_MVC: u32 = fourcc!(b'M', b'2', b'6', b'4');
        ///  H263
        pub const V4L2_PIX_FMT_H263: u32 = fourcc!(b'H', b'2', b'6', b'3');
        ///  MPEG-1 ES
        pub const V4L2_PIX_FMT_MPEG1: u32 = fourcc!(b'M', b'P', b'G', b'1');
        ///  MPEG-2 ES
        pub const V4L2_PIX_FMT_MPEG2: u32 = fourcc!(b'M', b'P', b'G', b'2');
        ///  MPEG-4 part 2 ES
        pub const V4L2_PIX_FMT_MPEG4: u32 = fourcc!(b'M', b'P', b'G', b'4');
        ///  Xvid
        pub const V4L2_PIX_FMT_XVID: u32 = fourcc!(b'X', b'V', b'I', b'D');
        ///  SMPTE 421M Annex G compliant stream
        pub const V4L2_PIX_FMT_VC1_ANNEX_G: u32 = fourcc!(b'V', b'C', b'1', b'G');
        ///  SMPTE 421M Annex L compliant stream
        pub const V4L2_PIX_FMT_VC1_ANNEX_L: u32 = fourcc!(b'V', b'C', b'1', b'L');
        ///  VP8
        pub const V4L2_PIX_FMT_VP8: u32 = fourcc!(b'V', b'P', b'8', b'0');
        ///  VP9
        pub const V4L2_PIX_FMT_VP9: u32 = fourcc!(b'V', b'P', b'9', b'0');
    }
    pub use compressed::*;

    /// Vendor-specific formats
    mod vendor_specific {
        ///  cpia1 YUV
        pub const V4L2_PIX_FMT_CPIA1: u32 = fourcc!(b'C', b'P', b'I', b'A');
        ///  Winnov hw compress
        pub const V4L2_PIX_FMT_WNVA: u32 = fourcc!(b'W', b'N', b'V', b'A');
        ///  SN9C10x compression
        pub const V4L2_PIX_FMT_SN9C10X: u32 = fourcc!(b'S', b'9', b'1', b'0');
        ///  SN9C20x YUV 4:2:0
        pub const V4L2_PIX_FMT_SN9C20X_I420: u32 = fourcc!(b'S', b'9', b'2', b'0');
        ///  pwc older webcam
        pub const V4L2_PIX_FMT_PWC1: u32 = fourcc!(b'P', b'W', b'C', b'1');
        ///  pwc newer webcam
        pub const V4L2_PIX_FMT_PWC2: u32 = fourcc!(b'P', b'W', b'C', b'2');
        ///  ET61X251 compression
        pub const V4L2_PIX_FMT_ET61X251: u32 = fourcc!(b'E', b'6', b'2', b'5');
        ///  YUYV per line
        pub const V4L2_PIX_FMT_SPCA501: u32 = fourcc!(b'S', b'5', b'0', b'1');
        ///  YYUV per line
        pub const V4L2_PIX_FMT_SPCA505: u32 = fourcc!(b'S', b'5', b'0', b'5');
        ///  YUVY per line
        pub const V4L2_PIX_FMT_SPCA508: u32 = fourcc!(b'S', b'5', b'0', b'8');
        ///  compressed GBRG bayer
        pub const V4L2_PIX_FMT_SPCA561: u32 = fourcc!(b'S', b'5', b'6', b'1');
        ///  compressed BGGR bayer
        pub const V4L2_PIX_FMT_PAC207: u32 = fourcc!(b'P', b'2', b'0', b'7');
        ///  compressed BGGR bayer
        pub const V4L2_PIX_FMT_MR97310A: u32 = fourcc!(b'M', b'3', b'1', b'0');
        ///  compressed RGGB bayer
        pub const V4L2_PIX_FMT_JL2005BCD: u32 = fourcc!(b'J', b'L', b'2', b'0');
        ///  compressed GBRG bayer
        pub const V4L2_PIX_FMT_SN9C2028: u32 = fourcc!(b'S', b'O', b'N', b'X');
        ///  compressed RGGB bayer
        pub const V4L2_PIX_FMT_SQ905C: u32 = fourcc!(b'9', b'0', b'5', b'C');
        ///  Pixart 73xx JPEG
        pub const V4L2_PIX_FMT_PJPG: u32 = fourcc!(b'P', b'J', b'P', b'G');
        ///  ov511 JPEG
        pub const V4L2_PIX_FMT_OV511: u32 = fourcc!(b'O', b'5', b'1', b'1');
        ///  ov518 JPEG
        pub const V4L2_PIX_FMT_OV518: u32 = fourcc!(b'O', b'5', b'1', b'8');
        ///  stv0680 bayer
        pub const V4L2_PIX_FMT_STV0680: u32 = fourcc!(b'S', b'6', b'8', b'0');
        ///  tm5600/tm60x0
        pub const V4L2_PIX_FMT_TM6000: u32 = fourcc!(b'T', b'M', b'6', b'0');
        ///  one line of Y then 1 line of VYUY
        pub const V4L2_PIX_FMT_CIT_YYVYUY: u32 = fourcc!(b'C', b'I', b'T', b'V');
        ///  YUV420 planar in blocks of 256 pixels
        pub const V4L2_PIX_FMT_KONICA420: u32 = fourcc!(b'K', b'O', b'N', b'I');
        ///  JPEG-Lite
        pub const V4L2_PIX_FMT_JPGL: u32 = fourcc!(b'J', b'P', b'G', b'L');
        ///  se401 janggu compressed rgb
        pub const V4L2_PIX_FMT_SE401: u32 = fourcc!(b'S', b'4', b'0', b'1');
        ///  S5C73M3 interleaved UYVY/JPEG
        pub const V4L2_PIX_FMT_S5C_UYVY_JPG: u32 = fourcc!(b'S', b'5', b'C', b'I');
        ///  Greyscale 8-bit L/R interleaved
        pub const V4L2_PIX_FMT_Y8I: u32 = fourcc!(b'Y', b'8', b'I', b' ');
        ///  Greyscale 12-bit L/R interleaved
        pub const V4L2_PIX_FMT_Y12I: u32 = fourcc!(b'Y', b'1', b'2', b'I');
        ///  Depth data 16-bit
        pub const V4L2_PIX_FMT_Z16: u32 = fourcc!(b'Z', b'1', b'6', b' ');
        ///  Mediatek compressed block mode
        pub const V4L2_PIX_FMT_MT21C: u32 = fourcc!(b'M', b'T', b'2', b'1');
        ///  Intel Planar Greyscale 10-bit and Depth 16-bit
        pub const V4L2_PIX_FMT_INZI: u32 = fourcc!(b'I', b'N', b'Z', b'I');
    }
    pub use vendor_specific::*;

    /// SDR formats - used only for Software Defined Radio devices
    mod sdr {
        ///  IQ u8
        pub const V4L2_SDR_FMT_CU8: u32 = fourcc!(b'C', b'U', b'0', b'8');
        ///  IQ u16le
        pub const V4L2_SDR_FMT_CU16LE: u32 = fourcc!(b'C', b'U', b'1', b'6');
        ///  complex s8
        pub const V4L2_SDR_FMT_CS8: u32 = fourcc!(b'C', b'S', b'0', b'8');
        ///  complex s14le
        pub const V4L2_SDR_FMT_CS14LE: u32 = fourcc!(b'C', b'S', b'1', b'4');
        ///  real u12le
        pub const V4L2_SDR_FMT_RU12LE: u32 = fourcc!(b'R', b'U', b'1', b'2');
        ///  planar complex u16be
        pub const V4L2_SDR_FMT_PCU16BE: u32 = fourcc!(b'P', b'C', b'1', b'6');
        ///  planar complex u18be
        pub const V4L2_SDR_FMT_PCU18BE: u32 = fourcc!(b'P', b'C', b'1', b'8');
        ///  planar complex u20be
        pub const V4L2_SDR_FMT_PCU20BE: u32 = fourcc!(b'P', b'C', b'2', b'0');
    }
    pub use sdr::*;

    /// Touch formats - used for Touch devices
    mod touch {
        ///  16-bit signed deltas
        pub const V4L2_TCH_FMT_DELTA_TD16: u32 = fourcc!(b'T', b'D', b'1', b'6');
        ///  8-bit signed deltas
        pub const V4L2_TCH_FMT_DELTA_TD08: u32 = fourcc!(b'T', b'D', b'0', b'8');
        ///  16-bit unsigned touch data
        pub const V4L2_TCH_FMT_TU16: u32 = fourcc!(b'T', b'U', b'1', b'6');
        ///  8-bit unsigned touch data
        pub const V4L2_TCH_FMT_TU08: u32 = fourcc!(b'T', b'U', b'0', b'8');
    }
    pub use touch::*;

    /// Meta-data formats
    mod meta_data {
        ///  R-Car VSP1 1-D Histogram
        pub const V4L2_META_FMT_VSP1_HGO: u32 = fourcc!(b'V', b'S', b'P', b'H');
        ///  R-Car VSP1 2-D Histogram
        pub const V4L2_META_FMT_VSP1_HGT: u32 = fourcc!(b'V', b'S', b'P', b'T');
    }
    pub use meta_data::*;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate as v4l;
    use std::mem;

    #[test]
    fn make_fourcc() {
        let V4L2_PIX_FMT_RGB24: u32 =
            (b'R' as u32) | ((b'G' as u32) << 8) | ((b'B' as u32) << 16) | ((b'3' as u32) << 24);

        assert_eq!(pixel_format::V4L2_PIX_FMT_RGB24, V4L2_PIX_FMT_RGB24);
    }

    #[test]
    fn ioctl_code() {
        let VIDIOC_S_FMT: libc::c_ulong = ((3 as libc::c_ulong) << 30)
            | ((b'V' as libc::c_ulong) << 8)
            | (5 as libc::c_ulong)
            | ((mem::size_of::<v4l::v4l2_format>() as libc::c_ulong) << 16);
        assert_eq!(codes::VIDIOC_S_FMT, VIDIOC_S_FMT);

        let VIDIOC_REQBUFS: libc::c_ulong = ((3 as libc::c_ulong) << 30)
            | ((b'V' as libc::c_ulong) << 8)
            | (8 as libc::c_ulong)
            | ((mem::size_of::<v4l::v4l2_requestbuffers>() as libc::c_ulong) << 16);
        assert_eq!(codes::VIDIOC_REQBUFS, VIDIOC_REQBUFS);

        let VIDIOC_QUERYBUF: libc::c_ulong = ((3 as libc::c_ulong) << 30)
            | ((b'V' as libc::c_ulong) << 8)
            | (9 as libc::c_ulong)
            | ((mem::size_of::<v4l::v4l2_buffer>() as libc::c_ulong) << 16);
        assert_eq!(codes::VIDIOC_QUERYBUF, VIDIOC_QUERYBUF);

        let VIDIOC_QBUF: libc::c_ulong = ((3 as libc::c_ulong) << 30)
            | ((b'V' as libc::c_ulong) << 8)
            | (15 as libc::c_ulong)
            | ((mem::size_of::<v4l::v4l2_buffer>() as libc::c_ulong) << 16);
        assert_eq!(codes::VIDIOC_QBUF, VIDIOC_QBUF);

        let VIDIOC_STREAMON: libc::c_ulong = ((1 as libc::c_ulong) << 30)
            | ((b'V' as libc::c_ulong) << 8)
            | (18 as libc::c_ulong)
            | ((mem::size_of::<libc::c_int>() as libc::c_ulong) << 16);
        assert_eq!(codes::VIDIOC_STREAMON, VIDIOC_STREAMON);

        let VIDIOC_DQBUF: libc::c_ulong = ((3 as libc::c_ulong) << 30)
            | ((b'V' as libc::c_ulong) << 8)
            | (17 as libc::c_ulong)
            | ((mem::size_of::<v4l::v4l2_buffer>() as libc::c_ulong) << 16);
        assert_eq!(codes::VIDIOC_DQBUF, VIDIOC_DQBUF);

        let VIDIOC_QBUF: libc::c_ulong = ((3 as libc::c_ulong) << 30)
            | ((b'V' as libc::c_ulong) << 8)
            | (15 as libc::c_ulong)
            | ((mem::size_of::<v4l::v4l2_buffer>() as libc::c_ulong) << 16);
        assert_eq!(codes::VIDIOC_QBUF, VIDIOC_QBUF);

        let VIDIOC_STREAMOFF: libc::c_ulong = ((1 as libc::c_ulong) << 30)
            | ((b'V' as libc::c_ulong) << 8)
            | (19 as libc::c_ulong)
            | ((mem::size_of::<libc::c_int>() as libc::c_ulong) << 16);
        assert_eq!(codes::VIDIOC_STREAMOFF, VIDIOC_STREAMOFF);
    }
}
