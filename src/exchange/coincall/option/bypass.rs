use crossbeam::thread;
use tracing::error;

use crate::exchange::coincall::auth::token::fetch_token_from_url;
use crate::exchange::coincall::Coincall;
use crate::exchange::ExchangeServer;
use crate::ExchangeId;

pub const COINCALL_URL: &str = "https://www.coincall.com/";

pub type CoincallOptionBypass = Coincall<CoincallServerBypass>;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CoincallServerBypass;

impl ExchangeServer for CoincallServerBypass {
    const ID: ExchangeId = ExchangeId::CoincallOption;

    /// Returns the WebSocket URL as a static string slice with a `'static`
    /// lifetime.
    ///
    /// # Implementation Details
    ///
    /// This function generates the WebSocket URL required for connecting to the
    /// Coincall server. The URL is built using a token obtained from the
    /// authentication server.
    ///
    /// ## Static Initialization
    ///
    /// The URL is stored in a `static mut` variable `WEBSOCKET_URL`, which is
    /// initially `None`. The first time `websocket_url()` is called, the
    /// URL is generated, stored in this static variable, and subsequently
    /// returned as a `&'static str`. Subsequent calls to this function will
    /// return the already stored URL, ensuring that the URL is only generated
    /// once during the program's execution.
    ///
    /// ## Memory Safety and String Leaking
    ///
    /// This implementation uses `String::leak()` to convert the generated
    /// `String` into a `&'static str`. While this allows for a simple
    /// solution that provides a `'static` lifetime, it also causes the
    /// memory allocated for the `String` to be leaked, as it will never be
    /// deallocated. This is an acceptable trade-off in scenarios where the
    /// URL does not change frequently and the memory leak is minimal.
    /// However, in environments where the URL might be regenerated multiple
    /// times during the program's execution, this approach could lead to
    /// significant memory usage.
    ///
    /// ## Thread Safety Considerations
    ///
    /// The function uses `unsafe` blocks because `static mut` is inherently
    /// unsafe due to potential data races in a multi-threaded environment.
    /// We mitigate this by ensuring that the URL is generated within a
    /// scoped thread using `crossbeam::thread::scope`, which helps guarantee
    /// that the URL is safely initialized before any other threads attempt to
    /// access it.
    ///
    /// # Returns
    ///
    /// - A `&'static str` representing the WebSocket URL.
    fn websocket_url() -> &'static str {
        // A static mutable variable to store the WebSocket URL. Initially set to None.
        static mut WEBSOCKET_URL: Option<String> = None;

        // Unsafe block required to access and modify static mut variable
        unsafe {
            // Check if the WebSocket URL has already been initialized
            if WEBSOCKET_URL.is_none() {
                // Use crossbeam's scoped threads to safely generate and store the URL
                thread::scope(|s| {
                    s.spawn(|_| {
                        // Attempt to get the authentication token
                        let token = match get_token() {
                            Ok(token) => token,
                            Err(e) => {
                                error!("Error getting token: {}", e);
                                std::process::exit(1);
                            }
                        };

                        // Generate the WebSocket URL using the token
                        let url = generate_wss_url(&token);

                        // Store the generated URL in the static mutable variable
                        WEBSOCKET_URL = Some(url);
                    })
                    .join()
                    .unwrap();
                })
                .unwrap();
            }

            // Return the WebSocket URL as a static string slice
            WEBSOCKET_URL.as_ref().unwrap().clone().leak()
        }
    }
}

pub fn get_token() -> Result<String, Box<dyn std::error::Error>> {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let token = runtime.block_on(fetch_token_from_url(COINCALL_URL))?;
    match token {
        Some(token) => Ok(token),
        None => Err("Token not found".into()),
    }
}

fn generate_wss_url(token: &str) -> String {
    format!(
        "wss://ws.coincall.com/options?code=10&Authorization=Bearer%20{}",
        token
    )
}
