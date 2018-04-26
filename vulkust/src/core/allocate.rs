use std::collections::BTreeMap;
use std::sync::{Arc, RwLock, Weak};

use super::list::{List, Node};

pub trait Object: Drop {
    fn size(&self) -> isize;
    fn suceeding_size(&self) -> isize;
    fn set_suceeding_size(&mut self, isize);
    fn place(
        &mut self,
        suceeding_range_size: isize,
        offset: isize,
        allocator: Arc<RwLock<Allocator>>,
    );
}

pub trait Allocator {
    fn increase_size(&mut self, size: isize);
    fn initialize(&mut self, itself: Weak<RwLock<Allocator>>);
    fn allocate(&mut self, bag: Arc<RwLock<Object>>);
    fn deallocate(&mut self, suceeding_range_size: isize, end_offset: isize);
    fn coallocate(&mut self);
}

pub struct Bag {
    front: isize,
    end: isize,
    size: isize,
    suceeding_range_size: isize,
    allocator: Option<Arc<RwLock<Allocator>>>,
}

impl Bag {
    pub fn new(front: isize, size: isize) -> Self {
        Bag {
            front,
            end: front + size,
            size,
            suceeding_range_size: 0,
            allocator: None,
        }
    }
}

impl Drop for Bag {
    fn drop(&mut self) {
        vxunwrap!(vxunwrap!(self.allocator).read()).deallocate(self.suceeding_range_size, self.end);
    }
}

impl Object for Bag {
    fn size(&self) -> isize {
        self.size
    }

    fn suceeding_size(&self) -> isize {
        self.suceeding_range_size
    }

    fn set_suceeding_size(&mut self, size: isize) {
        self.suceeding_range_size = size;
    }

    fn place(
        &mut self,
        suceeding_range_size: isize,
        offset: isize,
        allocator: Arc<RwLock<Allocator>>,
    ) {
        self.suceeding_range_size = suceeding_range_size;
        self.front = offset;
        self.end = self.size + offset;
        self.allocator = Some(allocator);
    }
}

struct FakeBag {}

impl FakeBag {
    pub fn new() -> Self {
        FakeBag {}
    }
}

impl Drop for FakeBag {
    fn drop(&mut self) {}
}

impl Object for FakeBag {
    fn size(&self) -> isize {
        0
    }

    fn suceeding_size(&self) -> isize {
        0
    }

    fn set_suceeding_size(&mut self, _size: isize) {}

    fn place(
        &mut self,
        _suceeding_range_size: isize,
        _offset: isize,
        _allocator: Arc<RwLock<Allocator>>,
    ) {
    }
}

pub struct Container {
    itself: Option<Weak<RwLock<Allocator>>>,
    front: isize,
    end: isize,
    size: isize,
    free_space: isize,
    tailing_space: isize,
    suceeding_range_size: isize,
    objects: List<Weak<RwLock<Object>>>,
    space_offset_node: BTreeMap<isize, BTreeMap<isize, Arc<RwLock<Node<Weak<RwLock<Object>>>>>>>,
    starting_object: Option<Arc<RwLock<Object>>>,
    ending_object: Option<Arc<RwLock<Object>>>,
    allocator: Option<Arc<RwLock<Allocator>>>,
}

impl Container {
    pub fn new(front: isize, size: isize) -> Self {
        let starting_object: Arc<RwLock<Object>> = Arc::new(RwLock::new(FakeBag::new()));
        let ending_object: Arc<RwLock<Object>> = Arc::new(RwLock::new(FakeBag::new()));
        let mut objects = List::new();
        objects.append(Arc::downgrade(&starting_object));
        objects.append(Arc::downgrade(&ending_object));
        let offsets = BTreeMap::new();
        offsets.insert(front, vxunwrap!(objects.front()));
        let space_offset_node = BTreeMap::new();
        space_offset_node.insert(size, offsets);
        Container {
            itself: None,
            front,
            end: front + size,
            size,
            free_space: size,
            tailing_space: size,
            suceeding_range_size: 0,
            objects,
            space_offset_node,
            starting_object: Some(starting_object),
            ending_object: Some(ending_object),
            allocator: None,
        }
    }
}

impl Drop for Container {
    fn drop(&mut self) {
        self.space_offset_node.clear();
        self.objects.clear();
        self.starting_object = None;
        self.ending_object = None;
        self.itself = None;
        match self.allocator {
            Some(allocator) => {
                vxunwrap!(allocator.read()).deallocate(self.suceeding_range_size, self.end)
            }
            None => (),
        }
    }
}

impl Object for Container {
    fn size(&self) -> isize {
        self.size
    }

    fn suceeding_size(&self) -> isize {
        self.suceeding_range_size
    }

    fn set_suceeding_size(&mut self, size: isize) {
        self.suceeding_range_size = size;
    }

    fn place(
        &mut self,
        suceeding_range_size: isize,
        offset: isize,
        allocator: Arc<RwLock<Allocator>>,
    ) {
        self.suceeding_range_size = suceeding_range_size;
        let translate = offset - self.front;
        self.front = offset;
        let mut space_offset_node = BTreeMap::new();
        for (space, offset_node) in &self.space_offset_node {
            let mut tree = BTreeMap::new();
            for (offset, node) in offset_node {
                let offset = offset + translate;
                vxunwrap!(vxunwrap!(vxunwrap!(node.read()).value.upgrade()).write()).place(
                    space,
                    offset,
                    vxunwrap!(self.itself),
                );
                tree.insert(offset, *node);
            }
            space_offset_node.insert(*space, tree);
        }
        self.space_offset_node = space_offset_node;
    }
}

impl Allocator for Container {
    // possibility of dead lock in here! if happened I gonna manage it
    fn increase_size(&mut self, size: isize) {
        let n = vxunwrap!(vxunwrap!(self.objects.back()).read());
        let n = vxunwrap!(vxunwrap!(n.previous()).read());
        let space = vxunwrap!(n.value.read()).suceeding_size();
        let new_space = space + size;
        vxunwrap!(n.value.write()).set_suceeding_size(new_space);
        self.space_offset_node.insert(
            new_space, vxunwrap!(self.space_offset_node.remove(space)));
        self.end += size;
        self.size += size;
        self.free_space += size;
        self.tailing_space += size;
    }

    fn initialize(&mut self, itself: Weak<RwLock<Allocator>>) {
        self.itself = Some(itself);
    }

    fn allocate(&mut self, bag: Arc<RwLock<Object>>) {
        let bag_size = vxunwrap!(bag.read()).size();
        if bag_size > self.free_space {
            vxlogf!("Out of space, you probably forget to increase the size.");
        }
        match self.objects.range_mut(bag_size..).next() {
            Some((_, ranges)) => vxunimplemented!(), // find requested space
            None => {
                self.coallocate();
                self.allocate(bag);
            }
        }
    }

    fn deallocate(&mut self, suceeding_range_size: isize, end_offset: isize) {
        vxunimplemented!();
    }

    fn coallocate(&mut self) {
        vxunimplemented!();
    }
}
