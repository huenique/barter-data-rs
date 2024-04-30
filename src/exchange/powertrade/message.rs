use barter_integration::model::Side;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct DisplayOrderAdded {
    pub timestamp: String,
    pub tradeable_entity_id: u64,
    pub market_id: u64,
    pub side: Side,
    pub display_order_id: String,
    pub display_price: String,
    pub display_quantity: String,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct DisplayOrderDeleted {
    pub timestamp: u64,
    pub tradeable_entity_id: u64,
    pub market_id: u32,
    pub side: Side,
    pub display_order: u64,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct DisplayOrderUpdated {
    pub timestamp: u64,
    pub tradeable_entity_id: u64,
    pub market_id: u32,
    pub side: Side,
    pub old_display_order_id: u64,
    pub new_display_order_id: u64,
    pub display_price: i64,
    pub display_quantity: u64,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SubscriptionResult {
    Subscribed(Subscribed),
    Error(SubscribeError),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Subscribed {
    pub tradeable_entity_id: u64,
    pub symbol: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscribeError {
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Deliverable<AliasedDeliverable> {
    pub deliverable_id: u64,
    pub symbol: String,
    pub tags: Vec<String>,
    pub decimal_places: u32,
    pub listing_status: ListingStatus,
    pub details: AliasedDeliverable,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum ListingStatus {
    Active,
    Delisted,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TradeableEntity {
    pub tradeable_entity_id: u64,
    pub symbol: String,
    pub tags: Vec<String>,
    pub price_deliverable_id: u64,
    pub price_decimal_places: u32,
    pub quantity_deliverable_id: u64,
    pub quantity_decimal_places: u32,
    pub buy_trading_limit_deliverable_id: u64,
    pub sell_trading_limit_deliverable_id: u64,
    pub tradeability: Tradeability,
    pub details: TradeableEntityDetails,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tradeability {
    Tradable,
    DisplayOnly,
    NotTradable,
    Delisted,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TradeableEntityDetails {
    SimpleMarket {
        market_id: u32,
        settlement_event: ZonedDatetime,
    },
    MultiMarket {
        market_ids: Vec<u32>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ZonedDatetime {
    pub datetime: Datetime,
    pub timezone: String,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct Datetime {
    date: Date,
    time: TimeOfDay,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct Date {
    year: u16,
    month: u32,
    day: u32,
}

#[derive(Copy, Clone, Debug, Deserialize, Serialize)]
pub struct TimeOfDay {
    hours: u32,
    minutes: u32,
    seconds: u32,
    nanoseconds: u64,
}

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct TopOfBook {
    pub timestamp: u64,
    pub tradeable_entity_id: u64,
    pub market_id: u32,
    pub buy_price: i64,
    pub buy_quantity: u64,
    pub sell_price: i64,
    pub sell_quantity: u64,
}
