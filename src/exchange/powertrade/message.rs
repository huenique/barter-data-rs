use barter_integration::model::Side;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize)]
pub struct DisplayOrderAdded {
    pub timestamp: String,
    pub tradeable_entity_id: String,
    pub market_id: String,
    pub side: Side,
    pub display_order_id: String,
    pub display_price: String,
    pub display_quantity: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DisplayOrderDeleted {
    pub timestamp: String,
    pub tradeable_entity_id: String,
    pub market_id: String,
    pub side: Side,
    pub display_order: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DisplayOrderUpdated {
    pub timestamp: String,
    pub tradeable_entity_id: String,
    pub market_id: String,
    pub side: Side,
    pub old_display_order_id: String,
    pub new_display_order_id: String,
    pub display_price: String,
    pub display_quantity: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum SubscriptionResult {
    Subscribed(Subscribed),
    Error(SubscribeError),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Subscribed {
    pub tradeable_entity_id: String,
    pub symbol: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SubscribeError {
    pub message: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Deliverable<AliasedDeliverable> {
    pub deliverable_id: String,
    pub symbol: String,
    pub tags: Vec<String>,
    pub decimal_places: String,
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
    pub tradeable_entity_id: String,
    pub symbol: String,
    pub tags: Vec<String>,
    pub price_deliverable_id: String,
    pub price_decimal_places: String,
    pub quantity_deliverable_id: String,
    pub quantity_decimal_places: String,
    pub buy_trading_limit_deliverable_id: String,
    pub sell_trading_limit_deliverable_id: String,
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
        market_id: String,
        settlement_event: ZonedDatetime,
    },
    MultiMarket {
        market_ids: Vec<String>,
    },
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ZonedDatetime {
    pub datetime: Datetime,
    pub timezone: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Datetime {
    date: Date,
    time: TimeOfDay,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Date {
    year: String,
    month: String,
    day: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TimeOfDay {
    hours: String,
    minutes: String,
    seconds: String,
    nanoseconds: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct TopOfBook {
    pub timestamp: String,
    pub tradeable_entity_id: String,
    pub market_id: String,
    pub buy_price: String,
    pub buy_quantity: String,
    pub sell_price: String,
    pub sell_quantity: String,
}
