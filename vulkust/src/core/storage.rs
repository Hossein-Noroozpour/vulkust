use super::object::Object;
use super::types::Id;
use std::collections::BTreeMap;

#[cfg_attr(debug_mode, derive(Debug))]
pub struct Storage<T: Object> {
    data: Vec<T>,
    id_index: BTreeMap<Id, usize>,
    name_index: BTreeMap<String, usize>,
}

impl<T> Storage<T>
where
    T: Object,
{
    pub fn new() -> Self {
        Self {
            data: Vec::new(),
            id_index: BTreeMap::new(),
            name_index: BTreeMap::new(),
        }
    }

    // pub fn load(&mut self, id: Id, f: F) -> T
    // where
    //     T: Clone,
    //     F: Fn() -> T,
    // {
    //     if let Some(i) = self.id_index.get(id) {
    //         return self.data[i];
    //     }
    //     let t = F();
    // }
}
