mod command_service;
use crate::{CommandResponse, Storage};

/// 对Command的处理抽象
pub trait CommandService {
    /// 处理 Command，返回 Response
    fn execute(self, store: &impl Storage) -> CommandResponse;
}
