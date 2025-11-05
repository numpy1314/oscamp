#[derive(Copy, Clone, Debug)]
pub enum DebugConsoleFunction {
    PutString {
        len: u64,
        addr: u64,
    },
}
