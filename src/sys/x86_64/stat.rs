#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Device(u64);
impl Device {
    pub fn new(major: u32, minor: u32) -> Self {
        let major = major as u64;
        let minor = minor as u64;
        Self(
            (major << 8) |
            ((minor & 0xFFFFFF00) << 32) |
            ((minor & 0x000000FF))
        )
    }
    pub fn major(&self) -> u32 {
        (self.0 >> 8) as u32
    }
    pub fn minor(&self) -> u32 {
        (
            ((self.0 & 0xFFFFFF00_0000000) >> 32) |
            self.0 & 0x00000000_000000FF
        ) as u32
    }
}
impl core::fmt::Debug for Device {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Device")
            .field("major", &self.major())
            .field("minor", &self.minor())
            .finish()
    }
}

// what the fuck?
#[derive(Clone)]
#[repr(C)]
pub struct Stat {
	pub device_id: u64,
	pub inode: u64,
	pub link_count: u64,
	pub mode: u32,
	pub uid: u32,
	pub gid: u32,
	__pad1: u32,
	pub device: Device,
	pub size: i64,
	pub block_size: i64,
	pub blocks: i64,
	pub access_time: u64,
	pub access_time_nsec: u64,
	pub modify_time: u64,
	pub modify_time_nsec: u64,
	pub status_change_time: u64,
	pub status_change_time_nsec: u64,
	_pad2: [u64; 3]
}
impl core::fmt::Debug for Stat {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Stat")
            .field("device_id", &self.device_id)
            .field("inode", &self.inode)
            .field("link_count", &self.link_count)
            .field("mode", &self.mode)
            .field("uid", &self.uid)
            .field("gid", &self.gid)
            .field("device", &self.device)
            .field("size", &self.size)
            .field("block_size", &self.block_size)
            .field("blocks", &self.blocks)
            .field("access_time", &self.access_time)
            .field("access_time_nsec", &self.access_time_nsec)
            .field("modify_time", &self.modify_time)
            .field("modify_time_nsec", &self.modify_time_nsec)
            .field("status_change_time", &self.status_change_time)
            .field("status_change_time_nsec", &self.status_change_time_nsec)
            .finish()
    }
}