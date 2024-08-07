pub mod book;
pub mod channel;
pub mod market;
pub mod message;
pub mod subscription;
pub mod ticker;

use crate::exchange::powertrade::book::l3::PowerTradeOrderBookL3;
use crate::exchange::powertrade::channel::PowerTradeChannel;
use crate::exchange::powertrade::market::PowerTradeMarket;
use crate::exchange::powertrade::subscription::PowerTradeSubResponse;
use crate::exchange::powertrade::ticker::PowerTradeTicker;
use crate::exchange::Connector;
use crate::exchange::ExchangeId;
use crate::exchange::ExchangeSub;
use crate::exchange::StreamSelector;
use crate::subscriber::validator::WebSocketSubValidator;
use crate::subscriber::WebSocketSubscriber;
use crate::subscription::book::OrderBooksL3;
use crate::subscription::ticker::Tickers;
use crate::transformer::stateless::StatelessTransformer;
use crate::transformer::ticker::MultiTickerTransformer;
use crate::ExchangeWsStream;

use barter_integration::error::SocketError;
use barter_integration::protocol::websocket::WsMessage;
use barter_macro::DeExchange;
use barter_macro::SerExchange;
use url::Url;

/// <https://power-trade.github.io/api-docs-source/ws_feeds.html#Market_Feeds_Connection_Parameters>
pub const BASE_URL_POWERTRADE: &str = "wss://api.wss.prod.power.trade/v1/feeds/market_data?type[]=all_rte,deliverable,funding_rate,last_trade_price,risk&mbp_period=1&mbo_period=0&snapshot_depth=100";

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Default, DeExchange, SerExchange)]
pub struct PowerTrade {
    pub connection_params: Vec<(&'static str, &'static str)>,
}

impl Connector for PowerTrade {
    const ID: ExchangeId = ExchangeId::PowerTrade;
    type Channel = PowerTradeChannel;
    type Market = PowerTradeMarket;
    type Subscriber = WebSocketSubscriber;
    type SubValidator = WebSocketSubValidator;
    type SubResponse = PowerTradeSubResponse;

    fn url() -> Result<Url, SocketError> {
        Url::parse(BASE_URL_POWERTRADE).map_err(SocketError::UrlParse)
    }

    fn requests(exchange_subs: Vec<ExchangeSub<Self::Channel, Self::Market>>) -> Vec<WsMessage> {
        exchange_subs
            .into_iter()
            .map(|sub| {
                WsMessage::Text(
                    serde_json::json!({
                        "subscribe": {
                            "symbol": sub.market.as_ref(),
                        },
                    })
                    .to_string(),
                )
            })
            .collect()
    }
}

impl StreamSelector<OrderBooksL3> for PowerTrade {
    type Stream = ExchangeWsStream<StatelessTransformer<Self, OrderBooksL3, PowerTradeOrderBookL3>>;
}

impl StreamSelector<Tickers> for PowerTrade {
    type Stream = ExchangeWsStream<MultiTickerTransformer<Self, Tickers, PowerTradeTicker>>;
}
