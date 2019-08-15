/// IOCTL CODES FOR VIDEO DEVICES
///
///
pub mod codes {
    const VIDEODEV2_IOC_MAGIC: u8 = b'V';

    pub const VIDIOC_QUERYCAP: libc::c_ulong = ior!(VIDEODEV2_IOC_MAGIC, 0, crate::v4l2_capability);
    pub const VIDIOC_RESERVED: libc::c_ulong = io!(VIDEODEV2_IOC_MAGIC, 1);
    pub const VIDIOC_ENUM_FMT: libc::c_ulong = iowr!(VIDEODEV2_IOC_MAGIC, 2, crate::v4l2_fmtdesc);
    pub const VIDIOC_G_FMT: libc::c_ulong = iowr!(VIDEODEV2_IOC_MAGIC, 4, crate::v4l2_format);
    pub const VIDIOC_S_FMT: libc::c_ulong = iowr!(VIDEODEV2_IOC_MAGIC, 5, crate::v4l2_format);
    pub const VIDIOC_REQBUFS: libc::c_ulong =
        iowr!(VIDEODEV2_IOC_MAGIC, 8, crate::v4l2_requestbuffers);
    pub const VIDIOC_QUERYBUF: libc::c_ulong = iowr!(VIDEODEV2_IOC_MAGIC, 9, crate::v4l2_buffer);
    pub const VIDIOC_G_FBUF: libc::c_ulong = ior!(VIDEODEV2_IOC_MAGIC, 10, crate::v4l2_framebuffer);
    pub const VIDIOC_S_FBUF: libc::c_ulong = iow!(VIDEODEV2_IOC_MAGIC, 11, crate::v4l2_framebuffer);
    pub const VIDIOC_OVERLAY: libc::c_ulong = iow!(VIDEODEV2_IOC_MAGIC, 14, ::std::os::raw::c_int);
    pub const VIDIOC_QBUF: libc::c_ulong = iowr!(VIDEODEV2_IOC_MAGIC, 15, crate::v4l2_buffer);
    pub const VIDIOC_EXPBUF: libc::c_ulong =
        iowr!(VIDEODEV2_IOC_MAGIC, 16, crate::v4l2_exportbuffer);
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

#[cfg(test)]
mod test {
    use super::*;
    use crate as v4l;
    use std::mem;

    fn cmp_with_naive() {
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
