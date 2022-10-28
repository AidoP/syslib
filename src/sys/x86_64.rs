use core::arch::asm;

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

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Error(u32);
impl Error {
    pub const EPERM: Self = Self(1);
    pub const ENOENT: Self = Self(2);
    pub const ESRCH: Self = Self(3);
    pub const EINTR: Self = Self(4);
    pub const EIO: Self = Self(5);
    pub const ENXIO: Self = Self(6);
    pub const E2BIG: Self = Self(7);
    pub const ENOEXEC: Self = Self(8);
    pub const EBADF: Self = Self(9);
    pub const ECHILD: Self = Self(10);
    pub const EAGAIN: Self = Self(11);
    pub const ENOMEM: Self = Self(12);
    pub const EACCES: Self = Self(13);
    pub const EFAULT: Self = Self(14);
    pub const ENOTBLK: Self = Self(15);
    pub const EBUSY: Self = Self(16);
    pub const EEXIST: Self = Self(17);
    pub const EXDEV: Self = Self(18);
    pub const ENODEV: Self = Self(19);
    pub const ENOTDIR: Self = Self(20);
    pub const EISDIR: Self = Self(21);
    pub const EINVAL: Self = Self(22);
    pub const ENFILE: Self = Self(23);
    pub const EMFILE: Self = Self(24);
    pub const ENOTTY: Self = Self(25);
    pub const ETXTBSY: Self = Self(26);
    pub const EFBIG: Self = Self(27);
    pub const ENOSPC: Self = Self(28);
    pub const ESPIPE: Self = Self(29);
    pub const EROFS: Self = Self(30);
    pub const EMLINK: Self = Self(31);
    pub const EPIPE: Self = Self(32);
    pub const EDOM: Self = Self(33);
    pub const ERANGE: Self = Self(34);
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
impl core::fmt::Debug for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let error = self.0;
        match *self {
            Self::EPERM => write!(f, "EPERM({error})"),
            Self::ENOENT => write!(f, "ENOENT({error})"),
            Self::ESRCH => write!(f, "ESRCH({error})"),
            Self::EINTR => write!(f, "EINTR({error})"),
            Self::EIO => write!(f, "EIO({error})"),
            Self::ENXIO => write!(f, "ENXIO({error})"),
            Self::E2BIG => write!(f, "E2BIG({error})"),
            Self::ENOEXEC => write!(f, "ENOEXEC({error})"),
            Self::EBADF => write!(f, "EBADF({error})"),
            Self::ECHILD => write!(f, "ECHILD({error})"),
            Self::EAGAIN => write!(f, "EAGAIN({error})"),
            Self::ENOMEM => write!(f, "ENOMEM({error})"),
            Self::EACCES => write!(f, "EACCES({error})"),
            Self::EFAULT => write!(f, "EFAULT({error})"),
            Self::ENOTBLK => write!(f, "ENOTBLK({error})"),
            Self::EBUSY => write!(f, "EBUSY({error})"),
            Self::EEXIST => write!(f, "EEXIST({error})"),
            Self::EXDEV => write!(f, "EXDEV({error})"),
            Self::ENODEV => write!(f, "ENODEV({error})"),
            Self::ENOTDIR => write!(f, "ENOTDIR({error})"),
            Self::EISDIR => write!(f, "EISDIR({error})"),
            Self::EINVAL => write!(f, "EINVAL({error})"),
            Self::ENFILE => write!(f, "ENFILE({error})"),
            Self::EMFILE => write!(f, "EMFILE({error})"),
            Self::ENOTTY => write!(f, "ENOTTY({error})"),
            Self::ETXTBSY => write!(f, "ETXTBSY({error})"),
            Self::EFBIG => write!(f, "EFBIG({error})"),
            Self::ENOSPC => write!(f, "ENOSPC({error})"),
            Self::ESPIPE => write!(f, "ESPIPE({error})"),
            Self::EROFS => write!(f, "EROFS({error})"),
            Self::EMLINK => write!(f, "EMLINK({error})"),
            Self::EPIPE => write!(f, "EPIPE({error})"),
            Self::EDOM => write!(f, "EDOM({error})"),
            Self::ERANGE => write!(f, "ERANGE({error})"),
            _ => write!(f, "UNKNOWN({error})"),
        }
    }
}
impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match *self {
            Self::EPERM => f.write_str("Operation not permitted"),
            Self::ENOENT => f.write_str("No such file or directory"),
            Self::ESRCH => f.write_str("No such process"),
            Self::EINTR => f.write_str("Interrupted system call"),
            Self::EIO => f.write_str("I/O error"),
            Self::ENXIO => f.write_str("No such device or address"),
            Self::E2BIG => f.write_str("Argument list too long"),
            Self::ENOEXEC => f.write_str("Exec format error"),
            Self::EBADF => f.write_str("Bad file number"),
            Self::ECHILD => f.write_str("No child processes"),
            Self::EAGAIN => f.write_str("Try again"),
            Self::ENOMEM => f.write_str("Out of memory"),
            Self::EACCES => f.write_str("Permission denied"),
            Self::EFAULT => f.write_str("Bad address"),
            Self::ENOTBLK => f.write_str("Block device required"),
            Self::EBUSY => f.write_str("Device or resource busy"),
            Self::EEXIST => f.write_str("File exists"),
            Self::EXDEV => f.write_str("Cross-device link"),
            Self::ENODEV => f.write_str("No such device"),
            Self::ENOTDIR => f.write_str("Not a directory"),
            Self::EISDIR => f.write_str("Is a directory"),
            Self::EINVAL => f.write_str("Invalid argument"),
            Self::ENFILE => f.write_str("File table overflow"),
            Self::EMFILE => f.write_str("Too many open files"),
            Self::ENOTTY => f.write_str("Not a typewriter"),
            Self::ETXTBSY => f.write_str("Text file busy"),
            Self::EFBIG => f.write_str("File too large"),
            Self::ENOSPC => f.write_str("No space left on device"),
            Self::ESPIPE => f.write_str("Illegal seek"),
            Self::EROFS => f.write_str("Read-only file system"),
            Self::EMLINK => f.write_str("Too many links"),
            Self::EPIPE => f.write_str("Broken pipe"),
            Self::EDOM => f.write_str("Math argument out of domain of func"),
            Self::ERANGE => f.write_str("Math result not representable"),
            Self(error) => write!(f, "Unknown, error {error}")
        }
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct Fd(u32);
impl Fd {
    #[allow(non_upper_case_globals)]
    pub const stdin: Self = Self(0);
    #[allow(non_upper_case_globals)]
    pub const stdout: Self = Self(1);
    #[allow(non_upper_case_globals)]
    pub const stderr: Self = Self(2);
    #[inline(always)]
    pub fn raw(self) -> u32 {
        self.0
    }
    #[inline(always)]
    pub fn from_raw(fd: u32) -> Self {
        Self(fd)
    }
}
impl TryFrom<isize> for Fd {
    type Error = Error;
    fn try_from(fd: isize) -> Result<Self, Self::Error> {
        if fd < 0 {
            Err(Error(-fd as u32))
        } else {
            Ok(Fd(fd as u32))
        }
    }
}

/// Read in the next available bytes from a file.
/// The buffer may be uninitialised.
/// # Safety
/// `buffer` must be writable for `buffer_len` bytes.
/// The lifetime assigned to the returned slice must not exceed that of buffer.
pub unsafe fn read_uninit<'a>(fd: Fd, buffer: *mut u8, buffer_len: usize) -> Result<&'a [u8], Error> {
    let count;
    syscall!{
        0x00(fd.raw(), buffer, buffer_len) -> count 
    }
    Error::maybe_usize(count).map(|len| std::slice::from_raw_parts(buffer, len))
}
/// Read in the next available bytes from a file.
/// The buffer may not be filled, extra bytes are left unmodified.
pub fn read(fd: Fd, buffer: &mut [u8]) -> Result<&[u8], Error> {
    unsafe { read_uninit(fd, buffer.as_mut_ptr(), buffer.len()) }
}
/// Write a slice of bytes to a file.
/// The number of bytes successfully written is returned.
pub fn write(fd: Fd, buffer: &[u8]) -> Result<usize, Error> {
    let count;
    unsafe {
        syscall!{
            0x01(fd.raw(), buffer.as_ptr(), buffer.len()) -> count 
        }
    }
    Error::maybe_usize(count)
}

pub mod open {
    use super::Error;
    crate::c_flags!{
        pub Flags(u32) {
            ReadOnly = 0b00,
            WriteOnly = 0b01,
            ReadWrite = 0b10,
            Create = 0o100,
            Append = 0o2000,
            NonBlocking = 0o4000,
            CloseOnExec = 0o2000000,
            NoAccessTime = 0o1000000
        } _ => Err(Error::EINVAL)
    }
    #[derive(Clone, Copy, PartialEq, Eq)]
    #[repr(transparent)]
    pub struct Mode(pub u32);
    #[allow(non_upper_case_globals)]
    impl Mode {
        pub const None: Self = Self(0);
        pub const User: Self = Self(0o700);
        pub const Group: Self = Self(0o070);
        pub const Other: Self = Self(0o007);
        pub const Read: Self = Self(0b100);
        pub const Write: Self = Self(0b010);
        pub const Exec: Self = Self(0b001);
        pub const Setuid: Self = Self(0o4000);
        pub const Setgid: Self = Self(0o2000);
        pub const Restricted: Self = Self(0o1000);
        /// Returns true if any of the bits are set
        pub fn any(self, bits: Self) -> bool {
            self & bits != Self::None
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
                    if self.1.any(Mode::Read) { debug.field(&Read); }
                    if self.1.any(Mode::Write) { debug.field(&Write); }
                    if self.1.any(Mode::Exec) { debug.field(&Exec); }
                    if !self.1.any(Mode::Other) { debug.field(&None); }
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
            if self.any(Self::Setuid) { debug.field(&Setuid); }
            if self.any(Self::Setgid) { debug.field(&Setgid); }
            if self.any(Self::Restricted) { debug.field(&Restricted); }
            debug.field(&Domain("User", (*self & Self::User) >> 6))
                .field(&Domain("Group", (*self & Self::Group) >> 3))
                .field(&Domain("Other", *self & Self::Other));
            debug.finish()
        }
    }
}
pub unsafe fn open_unsafe(path: *const u8, open::Flags(flags): open::Flags, open::Mode(mode): open::Mode) -> Result<Fd, Error> {
    let fd: isize;
    syscall!{
        0x02(path, flags, mode) -> fd
    }
    fd.try_into()
}
#[cfg(feature = "std")]
pub fn open<P: AsRef<std::path::Path>>(path: P, flags: open::Flags, mode: open::Mode) -> Result<Fd, Error> {
    use std::os::unix::prelude::OsStrExt;
    let path = std::ffi::CString::new(path.as_ref().as_os_str().as_bytes()).map_err(|_| Error::EINVAL)?;
    unsafe { open_unsafe(path.as_ptr() as *const u8, flags, mode) }
}

pub fn close(fd: Fd) -> Result<(), Error> {
    let err;
    unsafe {
        syscall!{
            0x03(fd.raw()) -> err
        }
    }
    Error::maybe(err)
}
pub unsafe fn stat_unsafe(path: *const u8) -> Result<Stat, Error> {
    let mut stat = core::mem::MaybeUninit::uninit();
    let err;
    syscall!{
        0x04(path, stat.as_mut_ptr()) -> err
    }
    Error::maybe(err).map(|_| stat.assume_init())
}
#[cfg(feature = "std")]
pub fn stat<P: AsRef<std::path::Path>>(path: P) -> Result<Stat, Error> {
    use std::os::unix::prelude::OsStrExt;
    let path = std::ffi::CString::new(path.as_ref().as_os_str().as_bytes()).map_err(|_| Error::EINVAL)?;
    unsafe { stat_unsafe(path.as_ptr() as *const u8) }
}
pub fn fstat(fd: Fd) -> Result<Stat, Error> {
    let mut stat = core::mem::MaybeUninit::uninit();
    let err;
    unsafe {
        syscall!{
            5(fd.raw(), stat.as_mut_ptr()) -> err
        }
        Error::maybe(err).map(|_| stat.assume_init())
    }
}
pub unsafe fn lstat_unsafe(path: *const u8) -> Result<Stat, Error> {
    let mut stat = core::mem::MaybeUninit::uninit();
    let err;
    syscall!{
        6(path, stat.as_mut_ptr()) -> err
    }
    Error::maybe(err).map(|_| stat.assume_init())
}
#[cfg(feature = "std")]
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
pub fn mmap(address: usize, length: usize, protection: usize, flags: usize, fd: Fd, offset: usize) -> Result<*mut core::ffi::c_void, Error> {
    let ptr;
    unsafe {
        syscall!{
            9(address, length, protection, flags, fd.raw(), offset) -> ptr
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
pub unsafe fn ioctl<T>(fd: Fd, cmd: u32, arg: *mut T) -> Result<(), Error> {
    let err;
    syscall!{
        16(fd.raw(), cmd, arg) -> err
    }
    Error::maybe(err)
}

/// Terminate the process, returning a code to the parent process.
/// 
/// Linux will clean up used resources, however, language termination functions such as `Drop` will not be run.
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