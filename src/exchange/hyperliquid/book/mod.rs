use crate::exchange::hyperliquid::book::l2::WsLevel;
use crate::subscription::book::Level;

pub mod l2;

impl From<WsLevel> for Level {
    fn from(level: WsLevel) -> Self {
        Self {
            price: level.px,
            amount: level.sz,
        }
    }
}
