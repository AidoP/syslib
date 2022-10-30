use crate::{enumeration, c_flags, Error};

pub const MAX_CONNECTIONS: u32 = 4096;

enumeration!{
    pub struct Domain(u32) {
        #["Unspecified protocol family"]
        UNSPECIFIED = 0,
        #["Unix Domain Socket"]
        UNIX = 1,
        #["IPv4"]
        INET = 2,
        #["IPv6"]
        INET6 = 10
    }
}

enumeration!{
    pub struct Type(u32) {
        #["Sequenced, reliable, connection-based byte streams"]
        STREAM = 1,
        #["Connectionless, unreliable datagrams of fixed maximum length"]
        DATAGRAM = 2
    }
}
enumeration!{
    pub struct TypeFlags(u32) {
        #["Atomically set close-on-exec flag for the new descriptor(s)"]
        CLOSE_ON_EXEC = 0o2000000,
        #["Atomically mark descriptor(s) as non-blocking"]
        NON_BLOCKING = 0o0004000
    }
}
impl std::ops::BitOr<TypeFlags> for Type {
    type Output = Self;
    fn bitor(self, rhs: TypeFlags) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

enumeration!{
    pub struct Protocol(u32) {
        #["Unspecified socket level"]
        UNSPECIFIED = 0,
        #["Raw socket"]
        RAW = 255
    }
}

enumeration!{
    pub struct Level(u32) {
        #["Unspecified socket level"]
        UNSPECIFIED = 0,
        #["The socket itself"]
        SOCKET = 0xffff
    }
}
enumeration!{
    pub struct AncillaryType(u32) {
        #["No ancillary data"]
        NONE = 0,
        #["Transfer file descriptors"]
        RIGHTS = 1
    }
}

c_flags!{
    pub Flags(u32) {
        OOB = 0x01,
        PEEK = 0x02,
        DONT_ROUTE = 0x04,
        NON_BLOCKING = 0x40,
        CLOSE_ON_EXEC = 0x4000_0000
    } _ => Err(crate::Error::EINVAL)
}


#[repr(transparent)]
pub struct Address<'a>(pub(crate) &'a [u8]);
/// The address of a Unix Domain socket.
#[repr(C)]
pub struct UnixAddress {
    // Always 1
    family: u16,
    path: [u8; 108]
}
impl UnixAddress {
    pub fn new(path: &[u8]) -> Result<Self, Error> {
        if path.len() > 107 {
            return Err(Error::EINVAL)
        }

        let mut p = [0; 108];
        p[..path.len()].copy_from_slice(path);

        Ok(Self {
            family: 1,
            path: p
        })
    }
    pub fn address<'a>(&'a self) -> Address<'a> {
        Address(unsafe { std::slice::from_raw_parts(self as *const _ as *const u8, std::mem::size_of::<Self>()) })
    }
}

/// Data that may be associated with a eocket message.
/// 
/// Data is wrapped in `MaybeUninit` as arbitrary bytes are read in, and not all types can be an arbitrary bit pattern.
/// Further, not all items may be written to and you may not want to initialise the whole buffer.
#[repr(C, align(8))]
struct AncillaryData<T, const N: usize>([std::mem::MaybeUninit<T>; N]);
#[repr(C)]
pub struct Ancillary<T, const N: usize> {
    len: usize,
    level: Level,
    ty: AncillaryType,
    data: AncillaryData<T, N>
}
impl<T: Copy, const N: usize> Ancillary<T, N> {
    pub fn new() -> Self {
        Self {
            len: 0,
            level: Level::UNSPECIFIED,
            ty: AncillaryType::NONE,
            data: AncillaryData([std::mem::MaybeUninit::uninit(); N])
        }
    }
    /// Get the items from the ancillary data. All items are initialised, but contain an arbitrary bit pattern the may not be
    /// valid for `T`.
    pub fn items(&self) -> &[std::mem::MaybeUninit<T>] {
        &self.data.0[..(self.len - std::mem::size_of::<Ancillary<T, 0>>()) / std::mem::size_of::<std::mem::MaybeUninit<T>>()]
    }
    pub fn level(&self) -> Level {
        self.level
    }
    pub fn ty(&self) -> AncillaryType {
        self.ty
    }
}