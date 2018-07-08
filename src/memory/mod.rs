use multiboot2::BootInformation;
use multiboot2::MemoryMapTag;
use self::paging::PhysicalAddress;
pub use self::paging::remap_the_kernel;
pub use self::paging::test_paging;

#[derive(Debug, PartialEq, PartialOrd, Ord)]
pub struct Frame {
    pub number: usize,
}

pub const PAGE_SIZE: usize = 4096;

pub trait FrameAllocator {
    fn allocate_frame(&mut self) -> Option<Frame>;
    fn deallocate_frame(&mut self, frame: Frame);
}

impl Frame {
    pub fn containing_address(address: usize) -> Frame {
        Frame { number: address / PAGE_SIZE }
    }

    fn start_address(&self) -> PhysicalAddress {
        self.number * PAGE_SIZE
    }
    fn clone(&self) -> Frame {
        Frame { number: self.number }
    }
    fn range_inclusive(start: Frame, end: Frame) -> FrameIter {
        FrameIter {
            start: start,
            end: end,
        }
    }
}

impl Eq for Frame {}

struct FrameIter {
    start: Frame,
    end: Frame,
}

impl Iterator for FrameIter {
    type Item = Frame;

    fn next(&mut self) -> Option<Frame> {
        if self.start <= self.end {
            let frame = self.start.clone();
            self.start.number += 1;
            Some(frame)
        } else {
            None
        }
    }
}

pub fn init(boot_info: &BootInformation) {
    let memory_map_tag = boot_info.memory_map_tag()
        .expect("Memory map tag required");

    println!("Kernel sections found:");
    let elf_sections = boot_info.elf_sections_tag().expect("Did not find elf section");
    for section in elf_sections.sections() {
        println!("    addr: {:#x}, size: {:#x}, flags: {:#x}",
                 section.addr, section.size, section.flags);
    }
    let kernel_start = elf_sections.sections().filter(|s| s.is_allocated()).map(|s| s.addr)
        .min().unwrap();
    let kernel_end = elf_sections.sections().filter(|s| s.is_allocated()).map(|s| s.addr + s.size)
        .max().unwrap();
    let multiboot_start = boot_info.start_address();
    let multiboot_end = boot_info.end_address();
    println!("Kernel start: {:#x}, kernel end: {:#x}", kernel_start, kernel_end);
    println!("Multiboot start: {:#x}, multiboot  end: {:#x}", multiboot_start, multiboot_end);
    let mut frame_allocator = self::area_frame_allocator::AreaFrameAllocator::new(
        kernel_start as usize, kernel_end as usize, multiboot_start,
        multiboot_end, memory_map_tag.memory_areas());
    let mut active_table = paging::remap_the_kernel(&mut frame_allocator,
                                                    boot_info);

    println!("Memory areas found:");
    for area in memory_map_tag.memory_areas() {
        println!("    start: {:#x}, length: {:#x}",
                 area.base_addr, area.length);
    }
    use self::paging::Page;
    use {HEAP_START, HEAP_SIZE};

    let heap_start_page = Page::containing_address(HEAP_START);
    let heap_end_page = Page::containing_address(HEAP_START + HEAP_SIZE-1);

    for page in Page::range_inclusive(heap_start_page, heap_end_page) {
        active_table.map(page, paging::WRITABLE, &mut frame_allocator);
    }
}

pub mod area_frame_allocator;
pub mod paging;
pub mod allocator;
