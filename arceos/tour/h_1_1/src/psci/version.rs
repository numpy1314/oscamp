use super::PsciReturn;
#[derive(Clone, Copy, Debug)]
pub struct VersionFunction;
impl VersionFunction {
    pub fn handle(&self) -> PsciReturn {
        PsciReturn::success(0x00010000)
    }
}
