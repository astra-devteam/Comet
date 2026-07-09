/// File System Module
/// Simple in-memory filesystem for Comet OS

pub mod inode;
pub mod vfs;

use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

/// File types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Regular,
    Directory,
    Symlink,
    Device,
}

/// File permissions (Unix-style)
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Permissions {
    pub owner_read: bool,
    pub owner_write: bool,
    pub owner_execute: bool,
    pub group_read: bool,
    pub group_write: bool,
    pub group_execute: bool,
    pub other_read: bool,
    pub other_write: bool,
    pub other_execute: bool,
}

impl Permissions {
    pub fn new(mode: u16) -> Self {
        Permissions {
            owner_read: (mode & 0o400) != 0,
            owner_write: (mode & 0o200) != 0,
            owner_execute: (mode & 0o100) != 0,
            group_read: (mode & 0o040) != 0,
            group_write: (mode & 0o020) != 0,
            group_execute: (mode & 0o010) != 0,
            other_read: (mode & 0o004) != 0,
            other_write: (mode & 0o002) != 0,
            other_execute: (mode & 0o001) != 0,
        }
    }

    pub fn to_octal(&self) -> u16 {
        let mut mode = 0u16;
        if self.owner_read { mode |= 0o400; }
        if self.owner_write { mode |= 0o200; }
        if self.owner_execute { mode |= 0o100; }
        if self.group_read { mode |= 0o040; }
        if self.group_write { mode |= 0o020; }
        if self.group_execute { mode |= 0o010; }
        if self.other_read { mode |= 0o004; }
        if self.other_write { mode |= 0o002; }
        if self.other_execute { mode |= 0o001; }
        mode
    }
}

/// File metadata
#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub inode: u64,
    pub file_type: FileType,
    pub size: u64,
    pub permissions: Permissions,
    pub owner_uid: u32,
    pub owner_gid: u32,
    pub created: u64,
    pub modified: u64,
    pub accessed: u64,
}

/// File handle
pub struct FileHandle {
    pub inode: u64,
    pub offset: u64,
    pub mode: FileMode,
}

#[derive(Debug, Clone, Copy)]
pub enum FileMode {
    Read,
    Write,
    ReadWrite,
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileType::Regular => write!(f, "file"),
            FileType::Directory => write!(f, "dir"),
            FileType::Symlink => write!(f, "link"),
            FileType::Device => write!(f, "device"),
        }
    }
}

/// Initialize filesystem
pub fn init() {
    // Initialize root filesystem
}
