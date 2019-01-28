use super::debug::Debug;
use super::types::Id;
use std::collections::BTreeMap;
use std::sync::{Arc, Weak};

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Storage<T>
where
    T: Debug + ?Sized,
{
    data: Vec<Arc<T>>,
    id_index: BTreeMap<Id, usize>,
    name_index: BTreeMap<String, usize>,
}

impl<T> Storage<T>
where
    T: Debug + ?Sized,
{
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            id_index: BTreeMap::new(),
            name_index: BTreeMap::new(),
        }
    }

    #[inline]
    pub fn load<F>(&mut self, id: Id, f: F) -> Arc<T>
    where
        T: Clone,
        F: Fn() -> Arc<T>,
    {
        if let Some(i) = self.id_index.get(&id) {
            return self.data[*i].clone();
        }
        let i = self.data.len();
        let t = f();
        self.data.push(t.clone());
        self.id_index.insert(id, i);
        return t;
    }

    #[inline]
    pub fn insert(&mut self, t: Arc<T>, id: Id, name: Option<String>) {
        let i = self.data.len();
        self.data.push(t);
        self.id_index.insert(id, i);
        if let Some(name) = name {
            self.name_index.insert(name, i);
        }
    }

    #[inline]
    pub fn get_with_index(&self, i: usize) -> Arc<T> {
        return self.data[i].clone();
    }

    #[inline]
    pub fn get_with_id(&self, id: Id) -> Option<Arc<T>> {
        if let Some(i) = self.id_index.get(&id) {
            return Some(self.data[*i].clone());
        }
        return None;
    }

    #[inline]
    pub fn get_with_name(&mut self, name: &str) -> Option<Arc<T>> {
        if let Some(i) = self.name_index.get(name) {
            return Some(self.data[*i].clone());
        }
        return None;
    }
}

#[cfg_attr(debug_mode, derive(Debug))]
pub struct WeakStorage<T>
where
    T: Debug + ?Sized,
{
    data: Vec<Weak<T>>,
    id_index: BTreeMap<Id, usize>,
    name_index: BTreeMap<String, usize>,
}

impl<T> WeakStorage<T>
where
    T: Debug + ?Sized,
{
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            id_index: BTreeMap::new(),
            name_index: BTreeMap::new(),
        }
    }

    #[inline]
    pub fn load<F>(&mut self, id: Id, f: F) -> Arc<T>
    where
        T: Clone,
        F: Fn() -> Arc<T>,
    {
        if let Some(i) = self.id_index.get(&id) {
            if let Some(t) = self.data[*i].upgrade() {
                return t;
            }
        }
        let i = self.data.len();
        let t = f();
        self.data.push(Arc::downgrade(&t));
        self.id_index.insert(id, i);
        return t;
    }

    #[inline]
    pub fn insert(&mut self, t: Weak<T>, id: Id, name: Option<String>) {
        let i = self.data.len();
        self.data.push(t);
        self.id_index.insert(id, i);
        if let Some(name) = name {
            self.name_index.insert(name, i);
        }
    }

    #[inline]
    pub fn get_with_index(&self, i: usize) -> Option<Arc<T>> {
        return self.data[i].upgrade();
    }

    #[inline]
    pub fn get_with_id(&self, id: Id) -> Option<Arc<T>> {
        if let Some(i) = self.id_index.get(&id) {
            return self.data[*i].upgrade();
        }
        return None;
    }

    #[inline]
    pub fn get_with_name(&self, name: &str) -> Option<Arc<T>> {
        if let Some(i) = self.name_index.get(name) {
            return self.data[*i].upgrade();
        }
        return None;
    }
}
