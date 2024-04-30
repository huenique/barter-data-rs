use serde::Deserialize;
use serde::Serialize;

use super::message::TimeOfDay;
use super::message::ZonedDatetime;

/// See: <https://docs.api.power.trade/#perpetual_future>
#[derive(Debug, Deserialize, Serialize)]
pub struct PerpetualFuture {
    pub funding_period: TimeOfDay,
    pub funding_reference_time: ZonedDatetime,
    pub funding_spec_id: u64,
    pub underlying_deliverable_id: u64,
    pub contract_size_deliverable_id: u64,
    pub contract_size: u64,
    pub settlement_deliverable_id: u64,
    pub utc_creation_time: u64,
    pub creation_source_id: u64,
    pub margin_spec_id: u64,
}

impl ToString for PerpetualFuture {
    fn to_string(&self) -> String {
        "PerpetualFuture".to_string()
    }
}
