use crate::{Frame, FrameAllocator, PhysAddr};

pub struct AreaFrameAllocator<const PAGE_SIZE: usize, MemoryArea, MemoryAreaIter, BannedRegions>
where
    MemoryArea: crate::MemoryArea,
    MemoryAreaIter: Iterator<Item = MemoryArea>,
    BannedRegions: Iterator<Item = BannedFrames<PAGE_SIZE>> + Clone,
{
    next_free_frame: Frame<PAGE_SIZE>,
    current_area: Option<MemoryArea>,
    areas: MemoryAreaIter,
    banned_areas: Option<BannedRegions>,
}

impl<const PAGE_SIZE: usize, MemoryArea, MemoryAreaIter, BannedRegions>
    AreaFrameAllocator<PAGE_SIZE, MemoryArea, MemoryAreaIter, BannedRegions>
where
    MemoryArea: crate::MemoryArea,
    MemoryAreaIter: Iterator<Item = MemoryArea> + Clone,
    BannedRegions: Iterator<Item = BannedFrames<PAGE_SIZE>> + Clone,
{
    pub fn new(areas: MemoryAreaIter, banned_areas: Option<BannedRegions>) -> Self {
        let mut allocator = Self {
            next_free_frame: Frame::from_addr(PhysAddr(PAGE_SIZE)),
            current_area: None,
            areas,
            banned_areas,
        };
        allocator.choose_next_area();
        allocator
    }

    fn choose_next_area(&mut self) {
        self.current_area = self
            .areas
            .clone()
            .filter(|area| {
                let address = area.start_address() + area.len() - 1;
                Frame::<PAGE_SIZE>::from_addr(PhysAddr(address)) >= self.next_free_frame
            })
            .min_by_key(|area| area.start_address());

        if let Some(area) = &self.current_area {
            let start_frame = Frame::<PAGE_SIZE>::from_addr(PhysAddr(area.start_address()));
            if self.next_free_frame < start_frame {
                self.next_free_frame = start_frame;
            }
        }
    }
}

impl<const PAGE_SIZE: usize, MemoryArea, MemoryAreaIter, BannedRegions> FrameAllocator<PAGE_SIZE>
    for AreaFrameAllocator<PAGE_SIZE, MemoryArea, MemoryAreaIter, BannedRegions>
where
    MemoryArea: crate::MemoryArea,
    MemoryAreaIter: Iterator<Item = MemoryArea> + Clone,
    BannedRegions: Iterator<Item = BannedFrames<PAGE_SIZE>> + Clone,
{
    fn allocate_frame(&mut self) -> Option<Frame<PAGE_SIZE>> {
        let area;
        if let Some(memory_area) = &self.current_area {
            area = memory_area;
        } else {
            return None;
        }

        let frame: Frame<PAGE_SIZE> = self.next_free_frame;

        let current_area_last_frame = {
            let address = area.start_address() + area.len() - 1;
            Frame::<PAGE_SIZE>::from_addr(PhysAddr(address))
        };

        let is_banned = if let Some(banned_areas) = &mut self.banned_areas.clone() {
            banned_areas.any(|banned| banned.contains(&frame))
        } else {
            false
        };

        if is_banned {
            self.next_free_frame = Frame {
                number: frame.number + 1,
            };
        } else if frame > current_area_last_frame {
            self.choose_next_area();
        } else {
            self.next_free_frame.number += 1;
            return Some(frame);
        }

        self.allocate_frame()
    }

    fn deallocate_frame(&mut self, _frame: Frame<PAGE_SIZE>) {}
}

#[derive(Debug)]
pub struct BannedFrames<const PAGE_SIZE: usize> {
    start: Frame<PAGE_SIZE>,
    end: Frame<PAGE_SIZE>,
}

impl<const PAGE_SIZE: usize> BannedFrames<PAGE_SIZE> {
    pub fn new(start: Frame<PAGE_SIZE>, end: Frame<PAGE_SIZE>) -> Self {
        Self { start, end }
    }

    pub fn contains(&self, frame: &Frame<PAGE_SIZE>) -> bool {
        frame >= &self.start && frame < &self.end
    }
}
