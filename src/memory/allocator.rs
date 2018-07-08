// The compiler needs to be instructed that this crate is an allocator in order
// to realize that when this is linked in another allocator like jemalloc should
// not be linked in
use alloc::alloc::{Alloc, AllocErr, Layout};
use core::ptr::NonNull;
use core::alloc::GlobalAlloc;
use core::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
pub struct GlobalBumpAllocator {
    heap_start: usize,
    heap_end: usize,
}

static NEXT_HEAP: AtomicUsize = AtomicUsize::new(0o_000_001_000_000_0000);

impl GlobalBumpAllocator {
    pub const fn new(heap_start: usize, heap_end: usize) -> Self {
        Self { heap_start, heap_end }
    }
}

pub fn align_down(addr: usize, align: usize) -> usize {
    if align.is_power_of_two() {
        addr & !(align - 1)
    } else if align == 0 {
        addr
    } else {
        panic!("`align` must be a power of 2");
    }
}

/// Align upwards. Returns the smallest x with alignment `align`
/// so that x >= addr. The alignment must be a power of 2.
pub fn align_up(addr: usize, align: usize) -> usize {
    align_down(addr + align - 1, align)
}

unsafe impl GlobalAlloc for GlobalBumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let current_next = NEXT_HEAP.load(Ordering::Relaxed);
        let alloc_start = align_up(current_next, layout.align());
        let alloc_end = alloc_start.saturating_add(layout.size());
        if alloc_end <= current_next {
            let next_now = NEXT_HEAP.compare_and_swap(current_next, alloc_end,
                                                      Ordering::Relaxed);
        }
        return alloc_start as *mut u8;
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unimplemented!()
    }
}