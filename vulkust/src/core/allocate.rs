use super::debug::Debug;
use std::sync::{Arc, RwLock, Weak};
use std::mem::size_of;

pub fn align(size: isize, alignment: isize, mask: isize, not_mask: isize) -> isize {
    #[cfg(debug_mode)]
    {
        check_power_of_two(alignment);
        if alignment -1 != mask {
            vxunexpected!();
        }
        if mask != !not_mask {
            vxunexpected!();
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
        vxunexpected!();        
    }
    loop {
        if v & 1 == 1 {
            v >>= 1;
            if v != 0 {
                vxunexpected!();
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
    fn get_size(&self) -> isize;
    fn get_offset(&self) -> isize;
    fn get_offset_alignment(&self) -> isize;
    fn get_offset_alignment_mask(&self) -> isize;
    fn get_offset_alignment_not_mask(&self) -> isize;
    fn place(&mut self, offset: isize);
}

pub trait Allocator: Debug {
    fn increase_size(&mut self, size: isize);
    fn allocate(&mut self, obj: &Arc<RwLock<Object>>);
    fn clean(&mut self);
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Memory {
    offset: isize,
    end: isize,
    size: isize,
    offset_alignment: isize,
    offset_alignment_mask: isize,
    offset_alignment_not_mask: isize,
}

impl Memory {
    pub(crate) fn new(size: isize, offset_alignment: isize) -> Self {
        let offset_alignment_mask = offset_alignment - 1;
        let offset_alignment_not_mask = !offset_alignment_mask;
        #[cfg(debug_mode)]
        {
            check_power_of_two(offset_alignment);
            if size != align(size, offset_alignment, offset_alignment_mask, offset_alignment_not_mask) {
                vxunexpected!();
            }
        }
        Memory {
            offset: 0,
            end: size,
            size,
            offset_alignment,
            offset_alignment_mask,
            offset_alignment_not_mask,
        }
    }
}

impl Object for Memory {
    fn get_size(&self) -> isize {
        self.size
    }

    fn get_offset(&self) -> isize {
        self.offset
    }

    fn get_offset_alignment(&self) -> isize {
        self.offset_alignment
    }

    fn get_offset_alignment_mask(&self) -> isize {
        self.offset_alignment_mask
    }

    fn get_offset_alignment_not_mask(&self) -> isize {
        self.offset_alignment_not_mask
    }

    fn place(&mut self, offset: isize) {
        #[cfg(debug_mode)]
        {
            if offset != align(offset, self.offset_alignment, self.offset_alignment_mask, self.offset_alignment_not_mask) {
                vxunexpected!();
            }
        }
        self.offset = offset;
        self.end = self.size + offset;
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Container {
    base: Memory,
    free_offset: isize,
    objects: Vec<Weak<RwLock<Object>>>,
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
    fn get_size(&self) -> isize {
        return self.base.get_size();
    }

    fn get_offset(&self) -> isize {
        return self.base.get_offset();
    }

    fn get_offset_alignment(&self) -> isize {
        return self.base.get_offset_alignment();
    }

    fn get_offset_alignment_mask(&self) -> isize {
        return self.base.get_offset_alignment_mask();
    }

    fn get_offset_alignment_not_mask(&self) -> isize {
        return self.base.get_offset_alignment_not_mask();
    }

    fn place(&mut self, offset: isize) {
        self.base.place(offset);
        self.clean();
    }
}

impl Allocator for Container {
    fn increase_size(&mut self, size: isize) {
        self.base.end += size;
        self.base.size += size;
    }

    fn allocate(&mut self, obj: &Arc<RwLock<Object>>) {
        let mut mobj = vxresult!(obj.write());
        let obj_size = mobj.get_size();
        let offset_alignment = mobj.get_offset_alignment();
        let offset_alignment_mask = mobj.get_offset_alignment_mask();
        let offset_alignment_not_mask = mobj.get_offset_alignment_not_mask();
        let offset = align(self.free_offset, offset_alignment, offset_alignment_mask, offset_alignment_not_mask);
        let next_free_offset = obj_size + offset;
        if next_free_offset > self.base.end {
            vxlogf!(
                "Out of space, you probably forget to increase \
                the size or cleaning the allocator, \
                offset_alignment: {}, \
                next_free_offset: {}, \
                aligned_free_offset: {}, \
                free_offset: {}, \
                size: {}, \
                offset: {}, \
                obj_size: {}",
                offset_alignment,
                next_free_offset,
                offset,
                self.free_offset,
                self.base.size,
                self.base.offset,
                obj_size
            );
        }
        mobj.place(offset);
        self.objects.push(Arc::downgrade(obj));
        self.free_offset = next_free_offset;
    }

    fn clean(&mut self) {
        let mut objects = Vec::new();
        self.free_offset = self.base.offset;
        for obj in &self.objects {
            if let Some(obj) = obj.upgrade() {
                let mut objm = vxresult!(obj.write());
                let size = objm.get_size();
                let offset = objm.get_offset();
                let offset_alignment = objm.get_offset_alignment();
                let offset_alignment_mask = objm.get_offset_alignment_mask();
                let offset_alignment_not_mask = objm.get_offset_alignment_not_mask();
                let aligned_offset = align(self.free_offset, offset_alignment, offset_alignment_mask, offset_alignment_not_mask);
                if aligned_offset != offset {
                    objm.place(aligned_offset);
                }
                objects.push(Arc::downgrade(&obj));
                self.free_offset = aligned_offset + size;
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
}