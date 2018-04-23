use std::collections::BTreeMap;
use std::sync::{Arc, RwLock, Weak};

pub trait Object: Drop {
    fn size(&self) -> isize;
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

struct ListNode {
    pub preceding: Option<Arc<RwLock<ListNode>>>,
    pub start: Weak<RwLock<Object>>,
    pub suceeding: Option<Arc<RwLock<ListNode>>>,
}

struct List {
    pub starting: Option<Arc<RwLock<ListNode>>>,
    pub ending: Option<Arc<RwLock<ListNode>>>,
}

pub struct Container {
    itself: Option<Weak<RwLock<Allocator>>>,
    front: isize,
    end: isize,
    size: isize,
    free_space: isize,
    tailing_space: isize,
    objects_count: isize,
    objects: BTreeMap<isize, BTreeMap<isize, Arc<RwLock<ListNode>>>>,
    starting_object: Option<Arc<RwLock<Object>>>,
    ending_object: Option<Arc<RwLock<Object>>>,
    container: Option<Arc<RwLock<Allocator>>>,
}

impl Container {
    pub fn new(front: isize, size: isize) -> Self {
        let starting_object: Arc<RwLock<Object>> = Arc::new(RwLock::new(Bag::new(front, 0)));
        let ending_object: Arc<RwLock<Object>> = Arc::new(RwLock::new(Bag::new(front + size, 0)));
        let range = Arc::new(RwLock::new(ListNode {
            preceding: None,
            start: Arc::downgrade(&starting_object),
            suceeding: None,
        }));
        let ranges = BTreeMap::new();
        ranges.insert(front, range);
        let objects = BTreeMap::new();
        objects.insert(size, ranges);
        Container {
            itself: None,
            front,
            end: front + size,
            size,
            free_space: size,
            tailing_space: size,
            objects_count: 0,
            objects,
            starting_object: Some(starting_object),
            ending_object: Some(ending_object),
            container: None,
        }
    }
}

impl Drop for Container {
    fn drop(&mut self) {}
}

impl Object for Container {
    fn size(&self) -> isize {
        self.size
    }

    fn place(
        &mut self,
        suceeding_range_size: isize,
        offset: isize,
        container: Arc<RwLock<Allocator>>,
    ) {
        vxunimplemented!();
    }
}

impl Allocator for Container {
    fn increase_size(&mut self, size: isize) {
        vxunimplemented!()
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
