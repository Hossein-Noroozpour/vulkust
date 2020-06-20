use super::debug::Debug;
use std::mem::size_of;
use std::sync::{Arc, RwLock, Weak};

pub fn align(size: isize, alignment: isize, mask: isize, not_mask: isize) -> isize {
    #[cfg(debug_mode)]
    {
        check_power_of_two(alignment);
        if alignment - 1 != mask {
            vx_unexpected!();
        }
        if mask != !not_mask {
            vx_unexpected!();
        }
    }
    let tmp = size & mask;
    if tmp == 0 {
        return size;
    }
    return alignment + (size & not_mask);
}

#[cfg(debug_mode)]
pub fn check_power_of_two(mut v: isize) {
    if v <= 0 {
        vx_unexpected!();
    }
    loop {
        if v & 1 == 1 {
            v >>= 1;
            if v != 0 {
                vx_unexpected!();
            }
            return;
        }
        v >>= 1;
    }
}

pub fn round_to_power_of_two(mut v: isize) -> isize {
    v -= 1;
    let bc = size_of::<isize>();
    let mut i = 1;
    loop {
        if i >= bc {
            break;
        }
        v |= v >> i;
        i <<= 1;
    }
    v += 1;
    return v;
}

pub trait Object: Debug {
    fn get_allocated_memory(&self) -> &Memory;
    fn place(&mut self, offset: isize);
}

pub trait Allocator: Debug {
    fn allocate(&mut self, obj: &Arc<RwLock<dyn Object>>);
    fn clean(&mut self);
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Memory {
    offset: isize,
    end: isize,
    size: isize,
    aligned_size: isize,
    offset_alignment: isize,
    offset_alignment_mask: isize,
    offset_alignment_not_mask: isize,
}

impl Memory {
    pub(crate) fn new(size: isize, offset_alignment: isize) -> Self {
        let offset_alignment_mask = offset_alignment - 1;
        let offset_alignment_not_mask = !offset_alignment_mask;
        let aligned_size = align(
            size,
            offset_alignment,
            offset_alignment_mask,
            offset_alignment_not_mask,
        );
        Memory {
            offset: 0,
            end: aligned_size,
            size,
            aligned_size,
            offset_alignment,
            offset_alignment_mask,
            offset_alignment_not_mask,
        }
    }

    pub(crate) fn get_size(&self) -> isize {
        return self.size;
    }

    // pub(crate) fn get_aligned_size(&self) -> isize {
    //     return self.aligned_size;
    // }

    // pub(crate) fn increase_size(&mut self, size: isize) {
    //     self.size += size;
    //     self.aligned_size = self.align(self.size);
    //     self.end = self.offset + self.aligned_size;
    // }

    pub(crate) fn align(&self, size: isize) -> isize {
        return align(
            size,
            self.offset_alignment,
            self.offset_alignment_mask,
            self.offset_alignment_not_mask,
        );
    }

    pub(crate) fn get_offset(&self) -> isize {
        return self.offset;
    }

    pub(crate) fn get_end(&self) -> isize {
        return self.end;
    }

    pub(crate) fn get_offset_alignment(&self) -> isize {
        return self.offset_alignment;
    }

    // pub(crate) fn get_offset_alignment_mask(&self) -> isize {
    //     return self.offset_alignment_mask;
    // }

    // pub(crate) fn get_offset_alignment_not_mask(&self) -> isize {
    //     return self.offset_alignment_not_mask;
    // }
}

impl Object for Memory {
    fn get_allocated_memory(&self) -> &Memory {
        return self;
    }

    fn place(&mut self, offset: isize) {
        #[cfg(debug_mode)]
        {
            if offset
                != align(
                    offset,
                    self.offset_alignment,
                    self.offset_alignment_mask,
                    self.offset_alignment_not_mask,
                )
            {
                vx_unexpected!();
            }
        }
        self.offset = offset;
        self.end = self.aligned_size + self.offset;
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Container {
    base: Memory,
    free_offset: isize,
    objects: Vec<Weak<RwLock<dyn Object>>>,
}

impl Container {
    pub(crate) fn new(size: isize, offset_alignment: isize) -> Self {
        Container {
            base: Memory::new(size, offset_alignment),
            free_offset: 0,
            objects: Vec::new(),
        }
    }
}

impl Object for Container {
    fn get_allocated_memory(&self) -> &Memory {
        return &self.base;
    }

    fn place(&mut self, offset: isize) {
        self.base.place(offset);
        self.clean();
    }
}

impl Allocator for Container {
    fn allocate(&mut self, obj: &Arc<RwLock<dyn Object>>) {
        let aligned_offset = vx_result!(obj.read())
            .get_allocated_memory()
            .align(self.free_offset);
        vx_result!(obj.write()).place(aligned_offset);
        let mobj = vx_result!(obj.read());
        let aobj = mobj.get_allocated_memory();
        self.free_offset = aobj.get_end();
        if self.free_offset > self.base.end {
            vx_log_f!(
                "Out of space, you probably forget to increase \
                 the size or cleaning the allocator, \
                 offset_alignment: {}, \
                 next_free_offset: {}, \
                 aligned_free_offset: {}, \
                 free_offset: {}, \
                 size: {}, \
                 offset: {}, \
                 obj_size: {}",
                aobj.get_offset_alignment(),
                self.free_offset,
                aligned_offset,
                self.free_offset,
                self.base.size,
                self.base.offset,
                aobj.get_size(),
            );
        }
        self.objects.push(Arc::downgrade(obj));
    }

    fn clean(&mut self) {
        let mut objects = Vec::with_capacity(self.objects.len());
        self.free_offset = self.base.offset;
        for obj in &self.objects {
            if let Some(obj) = obj.upgrade() {
                let mut mobj = vx_result!(obj.write());
                let aligned_offset = mobj.get_allocated_memory().align(self.free_offset);
                if aligned_offset != self.free_offset {
                    mobj.place(aligned_offset);
                }
                objects.push(Arc::downgrade(&obj));
                self.free_offset = mobj.get_allocated_memory().end;
            }
        }
        self.objects = objects;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[should_panic]
    fn check_power_of_two_test1() {
        check_power_of_two(3 * 1024);
    }

    #[test]
    #[should_panic]
    fn check_power_of_two_test2() {
        check_power_of_two(1024 + 1024 * 1024);
    }

    #[test]
    fn check_power_of_two_test3() {
        check_power_of_two(4 * 1024 * 1024 * 1024);
        check_power_of_two(4 * 1024 * 1024);
        check_power_of_two(4 * 1024);
        check_power_of_two(1024 * 1024 * 1024);
    }

    #[test]
    fn align_test1() {
        assert_eq!(4864, align(4851, 256, 255, !255));
        assert_eq!(19268608, align(19267623, 1024, 1023, !1023));
        assert_eq!(19269120, align(19268864, 512, 511, !511));
        assert_eq!(512, align(512, 512, 511, !511));
        assert_eq!(4851, align(4851, 1, 0, !0));
    }

    #[test]
    #[should_panic]
    fn align_test2() {
        align(4851, 256, 259, !259);
    }

    #[test]
    #[should_panic]
    fn align_test3() {
        align(4851, 256, 255, 25005);
    }

    #[test]
    #[should_panic]
    fn align_test4() {
        align(4851, 255, 254, !254);
    }

    #[test]
    #[should_panic]
    fn align_test5() {
        align(4851, 251, 255, 25005);
    }

    #[test]
    #[should_panic]
    fn align_test6() {
        align(4851, 200, 199, !199);
    }

    #[test]
    fn round_to_power_of_two_test1() {
        assert_eq!(2, round_to_power_of_two(2));
        assert_eq!(1, round_to_power_of_two(1));
        assert_eq!(4, round_to_power_of_two(3));
        assert_eq!(1024 * 1024, round_to_power_of_two(1024 * 1024 - 1));
        assert_eq!(1024 * 1024, round_to_power_of_two(1024 * 1024));
        assert_eq!(1024 * 1024, round_to_power_of_two(1024 * 1024 - 1024));
        assert_eq!(8, round_to_power_of_two(5));
        assert_eq!(8, round_to_power_of_two(6));
        assert_eq!(8, round_to_power_of_two(7));
    }

    #[test]
    fn container_test1() {}
}
