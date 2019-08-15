const VIDEODEV2_IOC_MAGIC: u8 = b'V';

const VIDIOC_QUERYCAP: libc::c_ulong = ior!(VIDEODEV2_IOC_MAGIC, 0, crate::v4l2_capability);
