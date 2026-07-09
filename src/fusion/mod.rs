/// Fusion: Object and Handle Management
/// Core abstraction layer for resource management

use core::sync::atomic::{AtomicU64, Ordering};

static HANDLE_COUNTER: AtomicU64 = AtomicU64::new(1);

/// Object handle - lightweight identifier for kernel objects
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectHandle {
    pub id: u64,
}

impl ObjectHandle {
    /// Create a new unique object handle
    pub fn new() -> Self {
        let id = HANDLE_COUNTER.fetch_add(1, Ordering::SeqCst);
        ObjectHandle { id }
    }

    /// Get the raw handle ID
    pub fn as_u64(&self) -> u64 {
        self.id
    }
}

/// Object types supported by Fusion
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectType {
    Thread,
    Process,
    File,
    Socket,
    Event,
    Semaphore,
}

/// Object Manager - manages all kernel objects
pub struct ObjectManager;

impl ObjectManager {
    /// Create a new kernel object
    pub fn create(object_type: ObjectType) -> ObjectHandle {
        ObjectHandle::new()
    }

    /// Close an object handle
    pub fn close(_handle: ObjectHandle) {
        // Future: implement cleanup logic
    }
}

use crate::vga::writer::VgaWriter;

pub fn init(vga: &mut VgaWriter) {
    vga.write_str("[*] Initializing Fusion (Object/Handle Management)...\n");
    vga.write_str("[✓] Object Manager initialized\n");
    vga.write_str("[✓] Fusion ready\n\n");
}
