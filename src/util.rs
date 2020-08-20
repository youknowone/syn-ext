pub trait UnwrapOrDefault {
    type Unwrapped;
    fn unwrap_or_default(self) -> Self::Unwrapped;
}
