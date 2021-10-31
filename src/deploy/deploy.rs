pub trait Deploy {
    fn init(&self);
    fn deploy(&self) -> bool;
    fn release(&self) -> bool;
}
