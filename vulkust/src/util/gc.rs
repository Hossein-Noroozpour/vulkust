use super::{List, ListNode};

pub trait GcObject {
    fn get_size(&self) -> usize;
    fn allocate(&mut self, offset: usize);
} 

struct MemInfo {
    front: usize,
    end: usize,
    size: usize,
    id: usize,
    pointer: Weak<RefCell<GcObject>>,
}

pub struct Gc {
    size: usize,
    last_offset: usize,
    filled: usize,
    last_id: usize,
    last_checked: Option<&mut ListNode<MemInfo>>,
    objects: List<MemInfo>,
}

impl Gc {
    pub fn new(size: usize) -> Self {
        Gc {
            size: size,
            last_offset: 0,
            filled: 0,
            last_id: 0,
            last_checked: None,
            objects: List::new(),
        }
    }

    pub fn allocate(&mut self, object: &Arc<RefCell<GcObject>>) {
        let obj_size = object.borrow().get_size();
        if self.size < obj_size {
            logf!("The Object you want to allocate is bigger than GC memory!");
        }
        if self.size - self.last_offset >= obj_size {
            object.borrow_mut().allocate(self.last_offset);
            self.objects.add_end(
                MeshInfo {
                    front: self.last_offset,
                    end: self.last_offset + obj_size,
                    size: obj_size,
                    id: self.last_id,
                    pointer: Arc::downgrade(object),
                }
            );
            self.last_offset += obj_size;
            self.filled += obj_size;
            self.last_id += 1;
            return;
        }
        if self.size - self.filled >= obj_size {
            self.last_checked = if self.last_checked.is_none() {
                self.objects.get_first_node().unwrap()
            };
            let mut offset_free = self.last_checked.as_ref().unwrap().end();
            let starting_id = node_obj.id;
            loop {
                let obj = node_obj.data.1.upgrade();
                self.last_checked = match obj {
                    Some(o) => {
                        let offset_free_end = node_obj.data.0.front;
                        if offset_free_end - offset_free >= obj_size {
                            object.borrow_mut().allocate(offset_free);
                            node_obj.add_parent(
                                MeshInfo {
                                    front: offset_free,
                                    end: offset_free + obj_size,
                                    size: obj_size,
                                    id: self.last_id,
                                    pointer: Arc::downgrade(object),
                                }
                            );
                            self.last_checked = node_obj.get_child();
                            self.filled += obj_size;
                            self.last_id += 1;
                            return;
                        }
                        offset_free = node_buffer.data.0.end;
                        node_obj.get_child()
                    },
                    None => {
                        self.filled -= node_obj.data.0.size;
                        node_obj.remove()
                    },
                };
                node_obj = match self.last_checked {
                    Some(n) => n,
                    None => {
                        if self.size - offset_free >= obj_size {
                            object.borrow_mut().allocate(offset_free);
                            node_obj.add_parent(
                                MeshInfo {
                                    front: offset_free,
                                    end: offset_free + obj_size,
                                    size: obj_size,
                                    id: self.last_id,
                                    pointer: Arc::downgrade(object),
                                }
                            );
                            self.filled += obj_size;
                            self.last_offset += obj_size;
                            self.last_id += 1;
                            return;
                        } else {
                            loge!("Performance warning!");
                            self.collocate_meshes();
                            if self.meshes_region_size - self.meshes_region_last_offset < mesh_size {
                                logf!("Out of buffer memory!");
                            }
                            let buff = Arc::new(RefCell::new(MeshBuffer {
                                need_refresh: true,
                                offset: self.meshes_region_last_offset,
                                index_offset: self.meshes_region_last_offset + vertices_size,
                                size: mesh_size,
                                vertex_size: vertex_size,
                                indices_count: indices_count,
                                address: unsafe { self.address.offset(self.meshes_region_last_offset) },
                                best_alignment: self.best_alignment,
                                main_buffer: self.main_buffer,
                                main_memory: self.main_memory,
                                staging_buffer: self.staging_buffer,
                                staging_memory: self.staging_memory,
                            }));
                            self.mesh_buffers.add_end((
                                MeshInfo {
                                    front: self.meshes_region_last_offset,
                                    end: self.meshes_region_last_offset + mesh_size,
                                }, 
                                Arc::downgrade(&buff)));
                            self.meshes_region_last_offset += mesh_size;
                            self.meshes_region_filled += mesh_size;
                            return buff;
                        }
                    },
                };
            }
        }
    }
}