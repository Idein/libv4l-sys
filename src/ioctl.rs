mod ioc {
    pub const NRBITS: libc::c_ulong = 8;
    pub const TYPEBITS: libc::c_ulong = 8;
    pub const SIZEBITS: u8 = 14;
    pub const DIRBITS: u8 = 2;

    pub const NRSHIFT: libc::c_ulong = 0;
    pub const TYPESHIFT: libc::c_ulong = NRSHIFT + NRBITS;
    pub const SIZESHIFT: libc::c_ulong = TYPESHIFT + TYPEBITS;
    pub const DIRSHIFT: libc::c_ulong = SIZESHIFT + SIZEBITS as libc::c_ulong;

    pub const NONE: u8 = 0;
    pub const WRITE: u8 = 1;
    pub const READ: u8 = 2;
}

pub use self::ioc::*;

/// _IOC
macro_rules! ioc {
    ($dir:expr, $ty:expr, $nr:expr, $sz:expr) => {
        (($dir as libc::c_ulong) << $crate::ioctl::DIRSHIFT)
            | (($ty as libc::c_ulong) << $crate::ioctl::TYPESHIFT)
            | (($nr as libc::c_ulong) << $crate::ioctl::NRSHIFT)
            | (($sz as libc::c_ulong) << $crate::ioctl::SIZESHIFT)
    };
}

/// _IO
#[macro_export]
macro_rules! io {
    ($ty:expr, $nr:expr) => {
        ioc!($crate::ioctl::NONE, $ty, $nr, 0)
    };
}

/// _IOR
#[macro_export]
macro_rules! ior {
    ($ty:expr, $nr:expr, $sz:ty) => {
        ioc!($crate::ioctl::READ, $ty, $nr, ::std::mem::size_of::<$sz>())
    };
}

/// _IOW
#[macro_export]
macro_rules! iow {
    ($ty:expr, $nr:expr, $sz:ty) => {
        ioc!($crate::ioctl::WRITE, $ty, $nr, ::std::mem::size_of::<$sz>())
    };
}

/// _IOWR
#[macro_export]
macro_rules! iowr {
    ($ty:expr, $nr:expr, $sz:ty) => {
        ioc!(
            $crate::ioctl::READ | $crate::ioctl::WRITE,
            $ty,
            $nr,
            ::std::mem::size_of::<$sz>()
        )
    };
}
