macro_rules! ThrowIfFailed {
    ($e:expr) => {
        if !$crate::winapi::shared::winerror::SUCCEEDED(unsafe { $e }) {
            vx_log_f!("Handle result failed!");
        }
    };
}
