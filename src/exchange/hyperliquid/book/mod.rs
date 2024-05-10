use crate::subscription::book::Level;

use self::l2::WsLevel;

pub mod l2;

impl From<WsLevel> for Level {
    fn from(level: WsLevel) -> Self {
        Self {
            price: level.px,
            amount: level.sz,
        }
    }
}
