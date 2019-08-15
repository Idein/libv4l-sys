mod ioc {
    pub const NRBITS: libc::c_ulong = 8;
    pub const TYPEBITS: libc::c_ulong = 8;
    pub const SIZEBITS: libc::c_ulong = 14;
    pub const DIRBITS: libc::c_ulong = 2;

    pub const NRSHIFT: libc::c_ulong = 0;
    pub const TYPESHIFT: libc::c_ulong = NRSHIFT + NRBITS;
    pub const SIZESHIFT: libc::c_ulong = TYPESHIFT + TYPEBITS;
    pub const DIRSHIFT: libc::c_ulong = SIZESHIFT + SIZEBITS;

    pub const NONE: u8 = 0;
    pub const WRITE: u8 = 1;
    pub const READ: u8 = 2;
}

pub use self::ioc::*;

/// _IOC
macro_rules! ioc {
    ($dir:expr, $ty:expr, $nr:expr, $sz:expr) => {
        ($dir << DIRSHIFT) | ($ty << TYPESHIFT) | ($nr << NRSHIFT) | ($sz << SIZESHIFT)
    };
}

/// IOR
macro_rules! ior {
    ($ty:expr, $nr:expr, $sz:expr) => {
        ioc!(READ, $ty, $nr, ::std::mem::size_of::<$sz>())
    };
}

/// IOW
macro_rules! iow {
    ($ty:expr, $nr:expr, $sz:expr) => {
        ioc!(WRITE, $ty, $nr, ::std::mem::size_of::<$sz>())
    };
}

/// IOWR
macro_rules! iowr {
    ($ty:expr, $nr:expr, $sz:expr) => {
        ioc!(READ | WRITE, $ty, $nr, ::std::mem::size_of::<$sz>())
    };
}
