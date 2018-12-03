macro_rules! ThrowIfFailed {
    ($e:expr) => {
        if !$crate::winapi::shared::winerror::SUCCEEDED(unsafe { $e }) {
            vxlogf!("Handle result failed!");
        }
    };
}
