/// Heap allocator for kernel memory allocation
/// 
/// This module provides memory allocation for kernel data structures.
/// Implements a simple bump allocator as a starting point.

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::NonNull;
use spinning_top::Mutex;

const HEAP_START: usize = 0x4444_4444_0000;
const HEAP_SIZE: usize = 100 * 1024; // 100 KiB

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
}

impl BumpAllocator {
    /// Create a new bump allocator
    pub const fn new(heap_start: usize, heap_size: usize) -> Self {
        BumpAllocator {
            heap_start,
            heap_end: heap_start + heap_size,
            next: heap_start,
        }
    }

    /// Allocate memory
    fn alloc(&mut self, layout: Layout) -> *mut u8 {
        let alloc_start = align_up(self.next, layout.align());
        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(end) => end,
            None => return core::ptr::null_mut(),
        };

        if alloc_end > self.heap_end {
            core::ptr::null_mut()
        } else {
            self.next = alloc_end;
            alloc_start as *mut u8
        }
    }

    /// Deallocate memory (no-op for bump allocator)
    fn dealloc(&mut self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator doesn't actually free memory
    }
}

unsafe impl GlobalAlloc for Mutex<BumpAllocator> {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.lock().alloc(layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        self.lock().dealloc(ptr, layout)
    }
}

/// Align up to the nearest multiple
fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

#[global_allocator]
static ALLOCATOR: Mutex<BumpAllocator> = Mutex::new(BumpAllocator::new(HEAP_START, HEAP_SIZE));

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}
