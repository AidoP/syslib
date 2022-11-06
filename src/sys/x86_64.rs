use core::{arch::asm, marker::PhantomData, fmt::Debug};
use crate::{enumeration, sock::{self, Ancillary}};

pub mod epoll;

mod fcntl;
pub use fcntl::Fcntl;

pub mod mmap;

mod stat;
pub use stat::{Device, Stat};

macro_rules! syscall {
    ($num:literal() -> $ret:ident) => {
        asm!(
            concat!("mov rax, ", $num),
            "syscall",
            out("rax") $ret,
            out("rcx") _,
            out("r11") _
        );
    };
    ($num:literal($arg1:expr) -> $ret:ident) => {
        asm!(
            concat!("mov rax, ", $num),
            "syscall",
            in("rdi") $arg1,
            out("rax") $ret,
            out("rcx") _,
            out("r11") _
        );
    };
    ($num:literal($arg1:expr, $arg2:expr) -> $ret:ident) => {
        asm!(
            concat!("mov rax, ", $num),
            "syscall",
            in("rdi") $arg1,
            in("rsi") $arg2,
            out("rax") $ret,
            out("rcx") _,
            out("r11") _
        );
    };
    ($num:literal($arg1:expr, $arg2:expr, $arg3:expr) -> $ret:ident) => {
        asm!(
            concat!("mov rax, ", $num),
            "syscall",
            in("rdi") $arg1,
            in("rsi") $arg2,
            in("rdx") $arg3,
            out("rax") $ret,
            out("rcx") _,
            out("r11") _
        );
    };
    ($num:literal($arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr) -> $ret:ident) => {
        asm!(
            concat!("mov rax, ", $num),
            "syscall",
            in("rdi") $arg1,
            in("rsi") $arg2,
            in("rdx") $arg3,
            in("r10") $arg4,
            out("rax") $ret,
            out("rcx") _,
            out("r11") _
        );
    };
    ($num:literal($arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr) -> $ret:ident) => {
        asm!(
            concat!("mov rax, ", $num),
            "syscall",
            in("rdi") $arg1,
            in("rsi") $arg2,
            in("rdx") $arg3,
            in("r10") $arg4,
            in("r8") $arg5,
            out("rax") $ret,
            out("rcx") _,
            out("r11") _
        );
    };
    ($num:literal($arg1:expr, $arg2:expr, $arg3:expr, $arg4:expr, $arg5:expr, $arg6:expr) -> $ret:ident) => {
        asm!(
            concat!("mov rax, ", $num),
            "syscall",
            in("rdi") $arg1,
            in("rsi") $arg2,
            in("rdx") $arg3,
            in("r10") $arg4,
            in("r8") $arg5,
            in("r9") $arg6,
            out("rax") $ret,
            out("rcx") _,
            out("r11") _
        );
    };
}

enumeration!{
    pub struct Error(u32) {
        #["Operation not permitted"]
        EPERM = 1,
        #["No such file or directory"]
        ENOENT = 2,
        #["No such process"]
        ESRCH = 3,
        #["Interrupted system call"]
        EINTR = 4,
        #["I/O error"]
        EIO = 5,
        #["No such device or address"]
        ENXIO = 6,
        #["Argument list too long"]
        E2BIG = 7,
        #["Exec format error"]
        ENOEXEC = 8,
        #["Bad file number"]
        EBADF = 9,
        #["No child processes"]
        ECHILD = 10,
        #["Try again"]
        EAGAIN = 11,
        #["Out of memory"]
        ENOMEM = 12,
        #["Permission denied"]
        EACCES = 13,
        #["Bad address"]
        EFAULT = 14,
        #["Block device required"]
        ENOTBLK = 15,
        #["Device or resource busy"]
        EBUSY = 16,
        #["File exists"]
        EEXIST = 17,
        #["Cross-device link"]
        EXDEV = 18,
        #["No such device"]
        ENODEV = 19,
        #["Not a directory"]
        ENOTDIR = 20,
        #["Is a directory"]
        EISDIR = 21,
        #["Invalid argument"]
        EINVAL = 22,
        #["File table overflow"]
        ENFILE = 23,
        #["Too many open files"]
        EMFILE = 24,
        #["Not a typewriter"]
        ENOTTY = 25,
        #["Text file busy"]
        ETXTBSY = 26,
        #["File too large"]
        EFBIG = 27,
        #["No space left on device"]
        ENOSPC = 28,
        #["Illegal seek"]
        ESPIPE = 29,
        #["Read-only file system"]
        EROFS = 30,
        #["Too many links"]
        EMLINK = 31,
        #["Broken pipe"]
        EPIPE = 32,
        #["Math argument out of domain of func"]
        EDOM = 33,
        #["Math result not representable"]
        ERANGE = 34,
        #["Resource deadlock would occur"]
        EDEADLK = 35,
        #["File name too long"]
        ENAMETOOLONG = 36,
        #["No record locks available"]
        ENOLCK = 37,
        #["Invalid system call number"]
        ENOSYS = 38,
        #["Directory not empty"]
        ENOTEMPTY = 39,
        #["Too many symbolic links encountered"]
        ELOOP = 40,
        #["No message of desired type"]
        ENOMSG = 42,
        #["Identifier removed"]
        EIDRM = 43,
        #["Channel number out of range"]
        ECHRNG = 44,
        #["Level 2 not synchronized"]
        EL2NSYNC = 45,
        #["Level 3 halted"]
        EL3HLT = 46,
        #["Level 3 reset"]
        EL3RST = 47,
        #["Link number out of range"]
        ELNRNG = 48,
        #["Protocol driver not attached"]
        EUNATCH = 49,
        #["No CSI structure available"]
        ENOCSI = 50,
        #["Level 2 halted"]
        EL2HLT = 51,
        #["Invalid exchange"]
        EBADE = 52,
        #["Invalid request descriptor"]
        EBADR = 53,
        #["Exchange full"]
        EXFULL = 54,
        #["No anode"]
        ENOANO = 55,
        #["Invalid request code"]
        EBADRQC = 56,
        #["Invalid slot"]
        EBADSLT = 57,
        #["Bad font file format"]
        EBFONT = 59,
        #["Device not a stream"]
        ENOSTR = 60,
        #["No data available"]
        ENODATA = 61,
        #["Timer expired"]
        ETIME = 62,
        #["Out of streams resources"]
        ENOSR = 63,
        #["Machine is not on the network"]
        ENONET = 64,
        #["Package not installed"]
        ENOPKG = 65,
        #["Object is remote"]
        EREMOTE = 66,
        #["Link has been severed"]
        ENOLINK = 67,
        #["Advertise error"]
        EADV = 68,
        #["Srmount error"]
        ESRMNT = 69,
        #["Communication error on send"]
        ECOMM = 70,
        #["Protocol error"]
        EPROTO = 71,
        #["Multihop attempted"]
        EMULTIHOP = 72,
        #["RFS specific error"]
        EDOTDOT = 73,
        #["Not a data message"]
        EBADMSG = 74,
        #["Value too large for defined data type"]
        EOVERFLOW = 75,
        #["Name not unique on network"]
        ENOTUNIQ = 76,
        #["File descriptor in bad state"]
        EBADFD = 77,
        #["Remote address changed"]
        EREMCHG = 78,
        #["Can not access a needed shared library"]
        ELIBACC = 79,
        #["Accessing a corrupted shared library"]
        ELIBBAD = 80,
        #[".lib section in a.out corrupted"]
        ELIBSCN = 81,
        #["Attempting to link in too many shared libraries"]
        ELIBMAX = 82,
        #["Cannot exec a shared library directly"]
        ELIBEXEC = 83,
        #["Illegal byte sequence"]
        EILSEQ = 84,
        #["Interrupted system call should be restarted"]
        ERESTART = 85,
        #["Streams pipe error"]
        ESTRPIPE = 86,
        #["Too many users"]
        EUSERS = 87,
        #["Socket operation on non-socket"]
        ENOTSOCK = 88,
        #["Destination address required"]
        EDESTADDRREQ = 89,
        #["Message too long"]
        EMSGSIZE = 90,
        #["Protocol wrong type for socket"]
        EPROTOTYPE = 91,
        #["Protocol not available"]
        ENOPROTOOPT = 92,
        #["Protocol not supported"]
        EPROTONOSUPPORT = 93,
        #["Socket type not supported"]
        ESOCKTNOSUPPORT = 94,
        #["Operation not supported on transport endpoint"]
        EOPNOTSUPP = 95,
        #["Protocol family not supported"]
        EPFNOSUPPORT = 96,
        #["Address family not supported by protocol"]
        EAFNOSUPPORT = 97,
        #["Address already in use"]
        EADDRINUSE = 98,
        #["Cannot assign requested address"]
        EADDRNOTAVAIL = 99,
        #["Network is down"]
        ENETDOWN = 100,
        #["Network is unreachable"]
        ENETUNREACH = 101,
        #["Network dropped connection because of reset"]
        ENETRESET = 102,
        #["Software caused connection abort"]
        ECONNABORTED = 103,
        #["Connection reset by peer"]
        ECONNRESET = 104,
        #["No buffer space available"]
        ENOBUFS = 105,
        #["Transport endpoint is already connected"]
        EISCONN = 106,
        #["Transport endpoint is not connected"]
        ENOTCONN = 107,
        #["Cannot send after transport endpoint shutdown"]
        ESHUTDOWN = 108,
        #["Too many references: cannot splice"]
        ETOOMANYREFS = 109,
        #["Connection timed out"]
        ETIMEDOUT = 110,
        #["Connection refused"]
        ECONNREFUSED = 111,
        #["Host is down"]
        EHOSTDOWN = 112,
        #["No route to host"]
        EHOSTUNREACH = 113,
        #["Operation already in progress"]
        EALREADY = 114,
        #["Operation now in progress"]
        EINPROGRESS = 115,
        #["Stale file handle"]
        ESTALE = 116,
        #["Structure needs cleaning"]
        EUCLEAN = 117,
        #["Not a XENIX named type file"]
        ENOTNAM = 118,
        #["No XENIX semaphores available"]
        ENAVAIL = 119,
        #["Is a named type file"]
        EISNAM = 120,
        #["Remote I/O error"]
        EREMOTEIO = 121,
        #["Quota exceeded"]
        EDQUOT = 122,
        #["No medium found"]
        ENOMEDIUM = 123,
        #["Wrong medium type"]
        EMEDIUMTYPE = 124,
        #["Operation Canceled"]
        ECANCELED = 125,
        #["Required key not available"]
        ENOKEY = 126,
        #["Key has expired"]
        EKEYEXPIRED = 127,
        #["Key has been revoked"]
        EKEYREVOKED = 128,
        #["Key was rejected by service"]
        EKEYREJECTED = 129,
        #["Owner died"]
        EOWNERDEAD = 130,
        #["State not recoverable"]
        ENOTRECOVERABLE = 131,
        #["Operation not possible due to RF-kill"]
        ERFKILL = 132,
        #["Memory page has hardware error"]
        EHWPOISON = 133
    }
}

impl Error {
    #[inline]
    pub fn maybe(maybe: isize) -> Result<(), Error> {
        if maybe < 0 {
            Err(Error(-maybe as u32))
        } else {
            Ok(())
        }
    }
    #[inline]
    pub fn maybe_usize(maybe: isize) -> Result<usize, Error> {
        if maybe < 0 {
            Err(Error(-maybe as u32))
        } else {
            Ok(maybe as usize)
        }
    }
    #[inline]
    pub fn maybe_ptr(maybe: isize) -> Result<*mut core::ffi::c_void, Error> {
        if maybe < 0 {
            Err(Error(-maybe as u32))
        } else {
            Ok(maybe as *mut _)
        }
    }
    #[inline]
    pub fn maybe_u32(maybe: isize) -> Result<u32, Error> {
        if maybe < 0 {
            Err(Error(-maybe as u32))
        } else {
            Ok(maybe as u32)
        }
    }
}

pub trait FileDescriptor {
    fn raw(&self) -> u32;
}

/// An un-owned file descriptor.
/// 
/// The owned equivalent is `File`. 
/// The lifetime of a Fd is "loose" as it is safe to close an in-use `File`,
/// although generally undesirable.
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Fd<'a>(u32, PhantomData<&'a u32>);
impl<'a> Fd<'a> {
    /// Create an unowned file descriptor from a raw file descriptor.
    pub fn from_raw(fd: u32) -> Self {
        Self(fd, PhantomData)
    }
    /// Make this file descriptor owned.
    /// 
    /// If this file descriptor is already owned this may lead to accidental double-closes.
    pub fn owned(self) -> File {
        File(self.0)
    }
    /// Extend the lifetime of the file descriptor to `'static`
    pub fn extend(self) -> Fd<'static> {
        Fd(self.0, PhantomData)
    }
}
impl Fd<'static> {
    #[allow(non_upper_case_globals)]
    pub const stdin: &'static Self = &Self(0, PhantomData);
    #[allow(non_upper_case_globals)]
    pub const stdout: &'static Self = &Self(1, PhantomData);
    #[allow(non_upper_case_globals)]
    pub const stderr: &'static Self = &Self(2, PhantomData);
}
impl<'a, 'b> AsRef<Fd<'b>> for Fd<'b> {
    fn as_ref(&self) -> &Fd<'b> {
        self
    }
}
impl<'a> FileDescriptor for Fd<'a> {
    #[inline(always)]
    fn raw(&self) -> u32 {
        self.0
    }
}

#[repr(transparent)]
pub struct File(u32);
impl File {
    /// Read from the file in to a buffer.
    /// 
    /// Returns a slice from the buffer that was written.
    #[inline]
    pub fn read<'a>(&self, buf: &'a mut [u8]) -> Result<&'a [u8], Error> {
        read(self, buf)
    }
    /// Write the buffer to the file.
    /// 
    /// Returns the number of bytes written.
    #[inline]
    pub fn write(&self, buf: &[u8]) -> Result<usize, Error> {
        write(self, buf)
    }
    /// Get the un-owned, raw file descriptor
    pub fn fd<'a>(&'a self) -> Fd<'a> {
        Fd(self.0, PhantomData)
    }
}
impl TryFrom<isize> for File {
    type Error = Error;
    fn try_from(maybe: isize) -> Result<Self, Self::Error> {
        if maybe < 0 {
            Err(Error(-maybe as u32))
        } else {
            Ok(Self(maybe as u32))
        }
    }
}
impl Debug for File {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("File")
            .field("fd", &self.0)
            .field("path", &"todo!()")
            .finish()
    }
}
impl FileDescriptor for File {
    #[inline(always)]
    fn raw(&self) -> u32 {
        self.0
    }
}
impl<'a> AsRef<Fd<'a>> for File {
    fn as_ref(&self) -> &'a Fd<'a> {
        // Safety: File and Fd are both `repr(transparent)` over u32
        unsafe { core::mem::transmute(self) }
    }
}
impl Drop for File {
    fn drop(&mut self) {
        let _ = close(self);
    }
}

#[derive(Debug)]
#[repr(transparent)]
pub struct Socket(u32);
impl Socket {
    /// Get the un-owned, raw file descriptor
    pub fn fd<'a>(&'a self) -> Fd<'a> {
        Fd(self.0, PhantomData)
    }
}
impl TryFrom<isize> for Socket {
    type Error = Error;
    fn try_from(maybe: isize) -> Result<Self, Self::Error> {
        if maybe < 0 {
            Err(Error(-maybe as u32))
        } else {
            Ok(Self(maybe as u32))
        }
    }
}
impl FileDescriptor for Socket {
    #[inline(always)]
    fn raw(&self) -> u32 {
        self.0
    }
}
impl<'a> AsRef<Fd<'a>> for Socket {
    fn as_ref(&self) -> &'a Fd<'a> {
        // Safety: Socket and Fd are both `repr(transparent)` over u32
        unsafe { core::mem::transmute(self) }
    }
}
impl Drop for Socket {
    fn drop(&mut self) {
        let _ = close(self);
    }
}

#[derive(Debug)]
#[repr(C)]
pub struct IoVec<'a> {
    pub buffer: *const u8,
    buffer_len: usize,
    _marker: core::marker::PhantomData<&'a [u8]>
}
impl<'a> IoVec<'a> {
    /// Construct a new `IoVec` from a slice.
    #[inline(always)]
    pub fn new(buffer: &'a [u8]) -> Self {
        Self {
            buffer: buffer.as_ptr(),
            buffer_len: buffer.len(),
            _marker: core::marker::PhantomData
        }
    }
    /// Construct a new `IoVec` that may point to uninitialised data.
    /// # Safety
    /// `buffer` must be readable for `buffer_len` bytes.
    #[inline(always)]
    pub unsafe fn maybe_uninit(buffer: *const u8, buffer_len: usize) -> Self {
        Self {
            buffer,
            buffer_len,
            _marker: core::marker::PhantomData
        }
    }
}
impl<'a> From<&'a [u8]> for IoVec<'a> {
    fn from(buffer: &'a [u8]) -> Self {
        Self {
            buffer: buffer.as_ptr(),
            buffer_len: buffer.len(),
            _marker: core::marker::PhantomData
        }
    }
}
#[derive(Debug)]
#[repr(C)]
pub struct IoVecMut<'a> {
    buffer: *mut u8,
    buffer_len: usize,
    _marker: core::marker::PhantomData<&'a [u8]>
}
impl<'a> IoVecMut<'a> {
    /// Construct a new `IoVecMut` from a slice.
    #[inline(always)]
    pub fn new(buffer: &mut [u8]) -> Self {
        Self {
            buffer: buffer.as_mut_ptr(),
            buffer_len: buffer.len(),
            _marker: core::marker::PhantomData
        }
    }
    /// Construct a new `IoVecMut` that may point to uninitialised data.
    /// # Safety
    /// `buffer` must be writable for `buffer_len` bytes.
    #[inline(always)]
    pub unsafe fn maybe_uninit(buffer: *mut u8, buffer_len: usize) -> Self {
        Self {
            buffer,
            buffer_len,
            _marker: core::marker::PhantomData
        }
    }
}
impl<'a> From<&'a mut [u8]> for IoVecMut<'a> {
    fn from(buffer: &'a mut [u8]) -> Self {
        Self {
            buffer: buffer.as_mut_ptr(),
            buffer_len: buffer.len(),
            _marker: core::marker::PhantomData
        }
    }
}

/// Read in the next available bytes from a file.
/// The buffer may be uninitialised.
/// 
/// # Safety
/// `buffer` must be readable and writable for `buffer_len` bytes.
/// The lifetime assigned to the returned slice must not exceed that of buffer.
#[inline]
pub unsafe fn read_uninit<'a, 'b, F: AsRef<Fd<'b>>>(fd: F, buffer: *mut u8, buffer_len: usize) -> Result<&'a [u8], Error> {
    let count;
    syscall!{
        0x00(fd.as_ref().raw(), buffer, buffer_len) -> count 
    }
    Error::maybe_usize(count).map(|len| std::slice::from_raw_parts(buffer, len))
}
/// Read in the next available bytes from a file.
/// The buffer may not be filled, extra bytes are left unmodified.
#[inline]
pub fn read<'a, 'b, F: AsRef<Fd<'b>>>(fd: F, buffer: &'a mut [u8]) -> Result<&'a [u8], Error> {
    unsafe { read_uninit(fd.as_ref(), buffer.as_mut_ptr(), buffer.len()) }
}
/// Write a slice of bytes to a file.
/// The number of bytes successfully written is returned.
#[inline]
pub fn write<'a, 'b, F: AsRef<Fd<'b>>>(fd: F, buffer: &[u8]) -> Result<usize, Error> {
    let count;
    unsafe {
        syscall!{
            0x01(fd.as_ref().raw(), buffer.as_ptr(), buffer.len()) -> count 
        }
    }
    Error::maybe_usize(count)
}

pub mod open {
    use super::Error;
    crate::c_flags!{
        pub Flags(u32) {
            READ_ONLY = 0b00,
            WRITE_ONLY = 0b01,
            READ_WRITE = 0b10,
            CREATE = 0o100,
            APPEND = 0o2000,
            NON_BLOCKING = 0o4000,
            CLOSE_ON_EXEC = 0o2000000,
            NO_ACCESS_TIME = 0o1000000
        } _ => Err(Error::EINVAL)
    }
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct Mode(pub u32);
    impl Mode {
        pub const NONE: Self = Self(0);
        pub const USER: Self = Self(0o700);
        pub const GROUP: Self = Self(0o070);
        pub const OTHER: Self = Self(0o007);
        pub const READ: Self = Self(0b100);
        pub const WRITE: Self = Self(0b010);
        pub const EXEC: Self = Self(0b001);
        pub const SETUID: Self = Self(0o4000);
        pub const SETGID: Self = Self(0o2000);
        pub const RESTRICTED: Self = Self(0o1000);
        /// Returns true if any of the bits are set
        pub fn any(self, bits: Self) -> bool {
            self & bits != Self::NONE
        }
        /// Returns true if all of the bits are set
        pub fn all(self, bits: Self) -> bool {
            self & bits == bits
        }
    }

    impl ::core::ops::BitAnd for Mode {
        type Output = Self;
        fn bitand(self, rhs: Self) -> Self::Output {
            Self(self.0 & rhs.0)
        }
    }
    impl ::core::ops::BitAndAssign for Mode {
        fn bitand_assign(&mut self, rhs: Self) {
            self.0 &= rhs.0
        }
    }
    impl ::core::ops::BitOr for Mode {
        type Output = Self;
        fn bitor(self, rhs: Self) -> Self::Output {
            Self(self.0 | rhs.0)
        }
    }
    impl ::core::ops::BitOrAssign for Mode {
        fn bitor_assign(&mut self, rhs: Self) {
            self.0 |= rhs.0
        }
    }
    impl ::core::ops::BitXor for Mode {
        type Output = Self;
        fn bitxor(self, rhs: Self) -> Self::Output {
            Self(self.0 ^ rhs.0)
        }
    }
    impl ::core::ops::BitXorAssign for Mode {
        fn bitxor_assign(&mut self, rhs: Self) {
            self.0 ^= rhs.0
        }
    }
    impl ::core::ops::Not for Mode {
        type Output = Self;
        fn not(self) -> Self::Output {
            Self(!self.0)
        }
    }
    impl ::core::ops::Shl<u32> for Mode {
        type Output = Self;
        fn shl(self, rhs: u32) -> Self::Output {
            Self(self.0 << rhs)
        }
    }
    impl ::core::ops::Shr<u32> for Mode {
        type Output = Self;
        fn shr(self, rhs: u32) -> Self::Output {
            Self(self.0 >> rhs)
        }
    }
    impl core::fmt::Debug for Mode {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            struct Domain(&'static str, Mode);
            impl core::fmt::Debug for Domain {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    #[derive(Debug)]
                    struct None;
                    #[derive(Debug)]
                    struct Read;
                    #[derive(Debug)]
                    struct Write;
                    #[derive(Debug)]
                    struct Exec;
                    let mut debug = f.debug_tuple(self.0);
                    if self.1.any(Mode::READ) { debug.field(&Read); }
                    if self.1.any(Mode::WRITE) { debug.field(&Write); }
                    if self.1.any(Mode::EXEC) { debug.field(&Exec); }
                    if !self.1.any(Mode::OTHER) { debug.field(&None); }
                    debug.finish()
                }
            }
            #[derive(Debug)]
            struct Setuid;
            #[derive(Debug)]
            struct Setgid;
            #[derive(Debug)]
            struct Restricted;
            let mut debug = f.debug_tuple("Mode");
            if self.any(Self::SETUID) { debug.field(&Setuid); }
            if self.any(Self::SETGID) { debug.field(&Setgid); }
            if self.any(Self::RESTRICTED) { debug.field(&Restricted); }
            debug.field(&Domain("User", (*self & Self::USER) >> 6))
                .field(&Domain("Group", (*self & Self::GROUP) >> 3))
                .field(&Domain("Other", *self & Self::OTHER));
            debug.finish()
        }
    }
}
/// Open a file from the file system.
/// 
/// # Safety
/// `path` must be null-terminated.
#[inline]
pub unsafe fn open_unsafe(path: *const u8, open::Flags(flags): open::Flags, open::Mode(mode): open::Mode) -> Result<File, Error> {
    let fd: isize;
    syscall!{
        0x02(path, flags, mode) -> fd
    }
    fd.try_into()
}
#[inline]
#[cfg(feature = "std")]
pub fn open<P: AsRef<std::path::Path>>(path: P, flags: open::Flags, mode: open::Mode) -> Result<File, Error> {
    use std::os::unix::prelude::OsStrExt;
    let path = std::ffi::CString::new(path.as_ref().as_os_str().as_bytes()).map_err(|_| Error::EINVAL)?;
    unsafe { open_unsafe(path.as_ptr() as *const u8, flags, mode) }
}

#[inline]
pub fn close<'a, F: AsRef<Fd<'a>>>(fd: F) -> Result<(), Error> {
    let err;
    unsafe {
        syscall!{
            0x03(fd.as_ref().raw()) -> err
        }
    }
    Error::maybe(err)
}
/// Get information about a file without opening it.
/// 
/// # Safety
/// `path` must be null-terminated.
#[inline]
pub unsafe fn stat_unsafe(path: *const u8) -> Result<Stat, Error> {
    let mut stat = core::mem::MaybeUninit::uninit();
    let err;
    syscall!{
        0x04(path, stat.as_mut_ptr()) -> err
    }
    Error::maybe(err).map(|_| stat.assume_init())
}
#[cfg(feature = "std")]
#[inline]
pub fn stat<P: AsRef<std::path::Path>>(path: P) -> Result<Stat, Error> {
    use std::os::unix::prelude::OsStrExt;
    let path = std::ffi::CString::new(path.as_ref().as_os_str().as_bytes()).map_err(|_| Error::EINVAL)?;
    unsafe { stat_unsafe(path.as_ptr() as *const u8) }
}
#[inline]
pub fn fstat<'a, F: AsRef<Fd<'a>>>(fd: F) -> Result<Stat, Error> {
    let mut stat = core::mem::MaybeUninit::uninit();
    let err;
    unsafe {
        syscall!{
            5(fd.as_ref().raw(), stat.as_mut_ptr()) -> err
        }
        Error::maybe(err).map(|_| stat.assume_init())
    }
}
/// Get information about a file without opening it and without following symlinks.
/// 
/// # Safety
/// `path` must be null-terminated.
#[inline]
pub unsafe fn lstat_unsafe(path: *const u8) -> Result<Stat, Error> {
    let mut stat = core::mem::MaybeUninit::uninit();
    let err;
    syscall!{
        6(path, stat.as_mut_ptr()) -> err
    }
    Error::maybe(err).map(|_| stat.assume_init())
}
#[cfg(feature = "std")]
#[inline]
pub fn lstat<P: AsRef<std::path::Path>>(path: P) -> Result<Stat, Error> {
    use std::os::unix::prelude::OsStrExt;
    let path = std::ffi::CString::new(path.as_ref().as_os_str().as_bytes()).map_err(|_| Error::EINVAL)?;
    unsafe { lstat_unsafe(path.as_ptr() as *const u8) }
}

/// Map a memory object in to the processes address space.
/// 
/// # Safety
/// Though creating a memory mapping can be considered safe, use of the memory mapping is likely quite unsafe.
/// Extra care must be taken when using a shared memory mapping.
#[inline]
pub fn mmap<'a, F: AsRef<Fd<'a>>>(address: usize, length: usize, protection: mmap::Protection, flags: mmap::Flags, fd: F, offset: usize) -> Result<*mut core::ffi::c_void, Error> {
    let ptr;
    let flags: u32 = flags.into();
    let protection: u32 = protection.into();
    unsafe {
        syscall!{
            9(address, length, protection, flags, fd.as_ref().raw(), offset) -> ptr
        }
    }
    Error::maybe_ptr(ptr)
}
/// Change the access protections for a region of memory.
/// 
/// Changes will be done in page-sized chunks.
/// 
/// # Safety
/// Changing the memory protection for a region pointed to by a reference is undefined behaviour.
#[inline]
pub unsafe fn mprotect(address: *mut core::ffi::c_void, length: usize, protection: usize) -> Result<(), Error> {
    let err;
    syscall!{
        10(address, length, protection) -> err
    }
    Error::maybe(err)
}
/// Unmap a region of memory from the processes address space.
/// 
/// # Safety
/// Unmapping memory that is in use or is pointed to by a reference is undefined behaviour.
#[inline]
pub unsafe fn munmap(address: *mut core::ffi::c_void, length: usize) -> Result<(), Error> {
    let err;
    syscall!{
        11(address, length) -> err
    }
    Error::maybe(err)
}

/// Send an I/O control command to a stream device.
/// 
/// # Safety
/// - `arg` must be appropriate for the given command and device of the file descriptor.
/// - `*mut T` must be a thin pointer.
#[inline]
pub unsafe fn ioctl<'a, T, F: AsRef<Fd<'a>>>(fd: F, cmd: u32, arg: *mut T) -> Result<(), Error> {
    let err;
    syscall!{
        16(fd.as_ref().raw(), cmd, arg) -> err
    }
    Error::maybe(err)
}

/// Vectorized read. The same operation as read but specifying a set of destination buffers.
/// The buffers may be uninitialised.
#[inline]
pub fn readv<'a, F: AsRef<Fd<'a>>>(fd: F, iov: &[IoVec]) -> Result<usize, Error> {
    // Safety: IoVec can only be constructed with potentially invalid values through an unsafe function.
    let count;
    unsafe {
        syscall!{
            19(fd.as_ref().raw(), iov.as_ptr(), iov.len()) -> count
        }
    }
    Error::maybe_usize(count)
}

/// Remap an existing memory mapping.
/// 
/// # Safety
/// Remapping memory that is in use or is pointed to by a reference is undefined behaviour if `MAY_MOVE` is specified or if the mapping is shrunk.
#[inline]
pub unsafe fn mremap(old_address: *mut core::ffi::c_void, old_size: usize, new_size: usize, flags: mmap::RemapFlags) -> Result<*mut core::ffi::c_void, Error> {
    let maybe;
    let flags: u32 = flags.into();
    syscall!{
        25(old_address, old_size, new_size, flags) -> maybe
    }
    Error::maybe_ptr(maybe)
}

/// Create a socket file descriptor.
#[inline]
pub fn socket(domain: sock::Domain, ty: sock::Type, protocol: sock::Protocol) -> Result<Socket, Error> {
    let fd: isize;
    let domain: u32 = domain.into();
    let ty: u32 = ty.into();
    let protocol: u32 = protocol.into();
    unsafe {
        syscall!{
            41(domain, ty, protocol) -> fd
        }
    }
    fd.try_into()
}
/// Initiate a connection on a socket.
#[inline]
pub fn connect<'a, F: AsRef<Fd<'a>>>(socket: F, address: sock::Address) -> Result<(), Error> {
    let error;
    unsafe {
        syscall!{
            42(socket.as_ref().raw(), address.0.as_ptr(), address.0.len()) -> error
        }
    }
    Error::maybe(error)
}
/// Accept a connection on a socket.
#[inline]
pub fn accept<'a, F: AsRef<Fd<'a>>>(socket: F) -> Result<Socket, Error> {
    let fd: isize;
    unsafe {
        syscall!{
            43(socket.as_ref().raw(), core::ptr::null_mut::<u8>(), core::ptr::null_mut::<u32>()) -> fd
        }
    }
    fd.try_into()
}

/// Send a message to a socket.
pub fn sendmsg<'a, T, const N: usize, F: AsRef<Fd<'a>>>(socket: F, iov: &[IoVec], ancillary: Option<&Ancillary<T, N>>, flags: sock::Flags) -> Result<usize, Error> {
    #[repr(C)]
    struct MessageHeader<'a, T, const N: usize> {
        address: *const u8,
        address_len: u32,
        iov: *const IoVec<'a>,
        iov_len: usize,
        ancillary: *const Ancillary<T, N>,
        ancillary_len: usize,
        flags: u32,
    }
    let msg = MessageHeader {
        // TODO: sockaddr
        address: std::ptr::null(),
        address_len: 0,
        iov: iov.as_ptr(),
        iov_len: iov.len(),
        ancillary_len: if let Some(ancillary) = ancillary { ancillary.len } else { 0 },
        ancillary: ancillary.map(|a| a as *const _).unwrap_or(std::ptr::null()),
        flags: 0
    };
    let count;
    let flags: u32 = flags.into();
    // Safety: IoVec can only be constructed with potentially invalid values through an unsafe function.
    unsafe {
        syscall!{
            46(socket.as_ref().raw(), &msg, flags) -> count
        }
    }
    Error::maybe_usize(count)
}
/// Recieve a message from a socket.
/// 
/// `flags` is written with the message return flags.
pub fn recvmsg<'a, T, const N: usize, F: AsRef<Fd<'a>>>(socket: F, iov: &[IoVecMut], ancillary: Option<&mut Ancillary<T, N>>, flags: sock::Flags) -> Result<usize, Error> {
    #[repr(C)]
    struct MessageHeader<'a, T, const N: usize> {
        address: *mut u8,
        address_len: u32,
        iov: *const IoVecMut<'a>,
        iov_len: usize,
        ancillary: *mut Ancillary<T, N>,
        ancillary_len: usize,
        flags: u32,
    }
    let mut msg = MessageHeader {
        // TODO: sockaddr
        address: std::ptr::null_mut(),
        address_len: 0,
        iov: iov.as_ptr(),
        iov_len: iov.len(),
        ancillary_len: if ancillary.is_none() { 0 } else { std::mem::size_of::<Ancillary<T, N>>() },
        ancillary: ancillary.map(|a| a as *mut _).unwrap_or(std::ptr::null_mut()),
        flags: 0
    };
    // Safety: IoVec can only be constructed with potentially invalid values through an unsafe function.
    let count;
    let flags: u32 = flags.into();
    unsafe {
        syscall!{
            47(socket.as_ref().raw(), &mut msg, flags) -> count
        }
    }
    Error::maybe_usize(count)
}

/// Bind a name to a socket.
#[inline]
pub fn bind<'a, F: AsRef<Fd<'a>>>(socket: F, address: sock::Address) -> Result<(), Error> {
    let maybe: isize;
    unsafe {
        syscall!{
            49(socket.as_ref().raw(), address.0.as_ptr(), address.0.len()) -> maybe
        }
    }
    Error::maybe(maybe)
}

/// Listen for connections on a socket.
#[inline]
pub fn listen<'a, F: AsRef<Fd<'a>>>(socket: F, backlog: u32) -> Result<(), Error> {
    let maybe: isize;
    unsafe {
        syscall!{
            50(socket.as_ref().raw(), backlog) -> maybe
        }
    }
    Error::maybe(maybe)
}

/// Terminate the process, returning a code to the parent process.
/// 
/// Linux will clean up used resources, however, language termination functions such as `Drop` will not be run.
#[inline]
pub fn exit(code: i32) -> ! {
    unsafe {
        asm!(
            "mov rax, 60",
            "syscall",
            in("rdi") code,
            options(noreturn)
        )
    }
}

/// Manipulate a file descriptor.
/// 
/// # Safety
/// - `arg` must be appropriate for the given command and device of the file descriptor.
/// - `*mut T` must be a thin pointer.
#[inline]
pub fn fcntl<'a, F: AsRef<Fd<'a>>>(fd: F, cmd: Fcntl) -> Result<u32, Error> {
    let maybe;
    unsafe {
        if let Some(arg) = cmd.arg() {
            syscall!{
                72(fd.as_ref().raw(), cmd.cmd(), arg) -> maybe
            }
        } else {
            syscall!{
                72(fd.as_ref().raw(), cmd.cmd()) -> maybe
            }
        }
    }
    Error::maybe_u32(maybe)
}

/// Remove an entry from the file system.
/// 
/// # Safety
/// `path` must be null-terminated.
#[inline]
pub unsafe fn unlink_unsafe(path: *const u8) -> Result<(), Error> {
    let maybe: isize;
    syscall!{
        87(path) -> maybe
    }
    Error::maybe(maybe)
}
/// Remove an entry from the file system.
#[inline]
#[cfg(feature = "std")]
pub fn unlink<P: AsRef<std::path::Path>>(path: P) -> Result<(), Error> {
    use std::os::unix::prelude::OsStrExt;
    let path = std::ffi::CString::new(path.as_ref().as_os_str().as_bytes()).map_err(|_| Error::EINVAL)?;
    unsafe { unlink_unsafe(path.as_ptr() as *const u8) }
}

/// Wait for an entry to enter the epoll ready list.
#[inline]
pub fn epoll_wait<'a, 'b, E: AsRef<Fd<'b>>>(epoll: E, events: &'a mut [std::mem::MaybeUninit<epoll::Event>], timeout: u32) -> Result<&'a [epoll::Event], Error> {
    let maybe: isize;
    unsafe {
        syscall!{
            232(epoll.as_ref().raw(), events.as_mut_ptr(), events.len(), timeout) -> maybe
        }
    }
    Error::maybe_u32(maybe).map(|count| unsafe { std::mem::transmute(&events[..count as usize]) })
}
/// Modify an entry in the epoll wait list.
#[inline]
pub fn epoll_ctl<'a, 'b, E: AsRef<Fd<'a>>, F: AsRef<Fd<'b>>>(epoll: E, fd: F, cmd: epoll::Cntl) -> Result<(), Error> {
    let maybe: isize;
    unsafe {
        syscall!{
            233(epoll.as_ref().raw(), cmd.cmd(), fd.as_ref().raw(), cmd.arg().map(|p| p as *const epoll::Event).unwrap_or(core::ptr::null())) -> maybe
        }
    }
    Error::maybe(maybe)
}
/// Open an epoll file descriptor.
#[inline]
pub fn epoll_create(flags: epoll::Flags) -> Result<File, Error> {
    let maybe: isize;
    let flags : u32 = flags.into();
    unsafe {
        syscall!{
            291(flags) -> maybe
        }
    }
    maybe.try_into()
}

/// Create an anonymous file.
/// 
/// # Safety
/// `name` must be null-terminated.
#[inline]
pub unsafe fn memfd_create_unsafe(name: *const u8, flags: u32) -> Result<File, Error> {
    let fd: isize;
    syscall!{
        319(name, flags) -> fd
    }
    fd.try_into()
}
/// Create an anonymous file.
#[inline]
#[cfg(feature = "std")]
pub fn memfd_create<P: AsRef<std::path::Path>>(name: &str, flags: u32) -> Result<File, Error> {
    let name = std::ffi::CString::new(name.as_bytes()).map_err(|_| Error::EINVAL)?;
    unsafe { memfd_create_unsafe(name.as_ptr() as *const u8, flags) }
}