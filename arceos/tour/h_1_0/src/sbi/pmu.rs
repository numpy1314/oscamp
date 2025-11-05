use axerrno::AxResult;
#[derive(Clone, Copy, Debug)]
pub enum PmuFunction {
    GetNumCounters,
    GetCounterInfo(u64),
    StopCounter {
        counter_index: u64,
        counter_mask: u64,
        stop_flags: u64,
    },
}
impl PmuFunction {
    pub(crate) fn from_regs(args: &[usize]) -> AxResult<Self> {
        match args[6] {
            0 => Ok(Self::GetNumCounters),
            1 => Ok(Self::GetCounterInfo(args[0] as u64)),
            4 => Ok(Self::StopCounter {
                counter_index: args[0] as u64,
                counter_mask: args[1] as u64,
                stop_flags: args[2] as u64,
            }),
            _ => panic!("Unsupported yet!"),
        }
    }
}
