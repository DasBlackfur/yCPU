pub trait Device {
    fn load(&mut self, addr: u8) -> u8;
    fn push(&mut self, addr: u8, data: u8);
    fn address(&self) -> u8;
}
