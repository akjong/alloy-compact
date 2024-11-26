use alloy::{
    primitives::{Log, LogData},
    rpc::types::trace::geth::CallLogFrame,
};

pub fn call_log_frame_to_log(log_frame: CallLogFrame) -> Option<Log> {
    let address = log_frame.address?;
    let topics = log_frame.topics?;
    let data = log_frame.data?;

    LogData::new(topics, data).map(|data| Log { address, data })
}
