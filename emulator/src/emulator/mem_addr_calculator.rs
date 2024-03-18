pub struct MemAddressCalculator {

}

impl MemAddressCalculator {
    pub fn new() -> MemAddressCalculator {
        MemAddressCalculator {}
    }

    pub fn calculate(&self, base: u64, offset: u64, scaler: u64) -> u64 {
        base + offset * scaler
    }
}