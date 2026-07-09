/// Inode management
use alloc::vec::Vec;
use super::{FileType, Permissions};

/// Inode structure
#[derive(Debug, Clone)]
pub struct Inode {
    pub number: u64,
    pub file_type: FileType,
    pub size: u64,
    pub permissions: Permissions,
    pub owner_uid: u32,
    pub owner_gid: u32,
    pub link_count: u32,
    pub created_time: u64,
    pub modified_time: u64,
    pub accessed_time: u64,
    pub blocks: Vec<u64>,
}

impl Inode {
    pub fn new(number: u64, file_type: FileType) -> Self {
        Inode {
            number,
            file_type,
            size: 0,
            permissions: Permissions {
                owner_read: true,
                owner_write: true,
                owner_execute: false,
                group_read: true,
                group_write: false,
                group_execute: false,
                other_read: true,
                other_write: false,
                other_execute: false,
            },
            owner_uid: 0,
            owner_gid: 0,
            link_count: 1,
            created_time: 0,
            modified_time: 0,
            accessed_time: 0,
            blocks: Vec::new(),
        }
    }
}
