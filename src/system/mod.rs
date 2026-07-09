/// System Distribution Layer
/// Dispatcher for routing system requests to appropriate kernel components

use crate::fusion::{ObjectHandle, ObjectType, ObjectManager};
use crate::vga::writer::VgaWriter;

/// System requests that can be dispatched
#[derive(Debug, Clone, Copy)]
pub enum Request {
    CreateThread,
    CreateProcess,
    OpenFile,
    CreateObject(ObjectType),
    CloseHandle(ObjectHandle),
}

/// Request dispatcher - routes requests to appropriate kernel components
pub struct Dispatcher;

impl Dispatcher {
    /// Dispatch a system request to the appropriate handler
    pub fn dispatch(request: Request) -> Result<ObjectHandle, DispatchError> {
        match request {
            Request::CreateThread => {
                // Kernel A: Thread creation
                let handle = ObjectManager::create(ObjectType::Thread);
                Ok(handle)
            }
            Request::CreateProcess => {
                // Kernel A: Process creation
                let handle = ObjectManager::create(ObjectType::Process);
                Ok(handle)
            }
            Request::OpenFile => {
                // Kernel B: File operations
                let handle = ObjectManager::create(ObjectType::File);
                Ok(handle)
            }
            Request::CreateObject(obj_type) => {
                // Fusion: Generic object creation
                let handle = ObjectManager::create(obj_type);
                Ok(handle)
            }
            Request::CloseHandle(handle) => {
                // Fusion: Handle cleanup
                ObjectManager::close(handle);
                Ok(handle)
            }
        }
    }
}

/// Dispatcher error types
#[derive(Debug)]
pub enum DispatchError {
    InvalidRequest,
    ResourceExhausted,
    PermissionDenied,
}

use core::fmt;
impl fmt::Display for DispatchError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DispatchError::InvalidRequest => write!(f, "Invalid request"),
            DispatchError::ResourceExhausted => write!(f, "Resource exhausted"),
            DispatchError::PermissionDenied => write!(f, "Permission denied"),
        }
    }
}

pub fn init(vga: &mut VgaWriter) {
    vga.write_str("[*] Initializing System Distribution Layer...\n");
    vga.write_str("[✓] Request Dispatcher initialized\n");
    vga.write_str("[✓] System Distribution ready\n\n");
}
