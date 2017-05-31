use super::super::super::objc;
use super::types::Id;

pub fn get_instance(s: &str) -> Id {
    let c = get_class!(s);
    let r: Id = unsafe { msg_send![c, alloc] };
    let r: Id = unsafe { msg_send![r, init] };
    return r;
}

pub fn set_ivar<T>(id: Id, name: &str, value: T) where T: objc::Encode {
    unsafe { (*id).set_ivar(name, value); }
}

pub fn alloc(s: &str) -> Id {
    let c = get_class!(s);
    unsafe { msg_send![c, alloc] }
}
