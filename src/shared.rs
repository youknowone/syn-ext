pub trait SharedEmpty {
    fn empty_ref() -> &'static Self;
}

/// # Safety
/// The value must not be mutable at any point of its lifecycle.
/// The local key must not be destroyed.
pub(crate) unsafe fn thread_local_ref<T>(local: &'static std::thread::LocalKey<T>) -> &'static T {
    let ptr = local.with(|nested| nested as *const _);
    unsafe { &*ptr }
}
