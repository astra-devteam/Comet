/// Virtual File System
use alloc::string::String;
use alloc::vec::Vec;
use super::{FileHandle, FileMode, FileType, Permissions};

/// Directory entry
#[derive(Debug, Clone)]
pub struct DirEntry {
    pub name: String,
    pub inode: u64,
    pub file_type: FileType,
}

/// Virtual filesystem implementation
pub struct VFS {
    entries: Vec<DirEntry>,
}

impl VFS {
    pub fn new() -> Self {
        VFS {
            entries: Vec::new(),
        }
    }

    pub fn create_file(&mut self, name: &str, file_type: FileType) -> u64 {
        let inode = self.entries.len() as u64;
        self.entries.push(DirEntry {
            name: String::from(name),
            inode,
            file_type,
        });
        inode
    }

    pub fn list_directory(&self, path: &str) -> Option<Vec<DirEntry>> {
        let mut result = Vec::new();
        for entry in &self.entries {
            result.push(entry.clone());
        }
        Some(result)
    }

    pub fn find_file(&self, name: &str) -> Option<u64> {
        for entry in &self.entries {
            if entry.name == name {
                return Some(entry.inode);
            }
        }
        None
    }
}
