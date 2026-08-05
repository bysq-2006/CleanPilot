use std::sync::Arc;

use tokio_util::sync::CancellationToken;

#[derive(Debug, Clone)]
pub enum AgentState {
    Idle,
    Chatting {
        cancellation_token: Arc<CancellationToken>,
    },
}

impl AgentState {
    /// 开始一轮新请求并返回这一轮共享的取消令牌。
    pub fn begin(&mut self) -> Result<Arc<CancellationToken>, String> {
        if self.is_chatting() {
            return Err("Agent 正在处理上一条消息".to_string());
        }

        let cancellation_token = Arc::new(CancellationToken::new());
        *self = Self::Chatting {
            cancellation_token: cancellation_token.clone(),
        };
        Ok(cancellation_token)
    }

    /// 取消当前请求并将状态切换回空闲。
    pub fn cancel(&mut self) -> bool {
        let Self::Chatting { cancellation_token } = self else {
            return false;
        };
        cancellation_token.cancel();
        *self = Self::Idle;
        true
    }

    /// 仅在令牌属于当前请求时结束这一轮请求。
    pub fn finish_if_current(&mut self, token: &Arc<CancellationToken>) {
        if matches!(self, Self::Chatting { cancellation_token } if Arc::ptr_eq(cancellation_token, token)) {
            *self = Self::Idle;
        }
    }

    /// 判断 Agent 当前是否正在处理请求。
    pub fn is_chatting(&self) -> bool {
        matches!(self, Self::Chatting { .. })
    }
}
