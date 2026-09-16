pub mod live;
pub mod simulator;

use common::models::TrafficEvent;

pub trait PacketSource: Send + Sync {
    fn next_event(&mut self) -> impl std::future::Future<Output = Option<TrafficEvent>> + Send;
}
