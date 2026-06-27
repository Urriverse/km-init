use super::Ke;

struct GA;

unsafe impl core::alloc::GlobalAlloc for GA {
    unsafe fn alloc(&self, layout: core::alloc::Layout) -> *mut u8 {
        Ke!(alloc layout)
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: core::alloc::Layout) {
        Ke!(free ptr, layout)
    }
}

#[global_allocator]
static GAI: GA = GA;
