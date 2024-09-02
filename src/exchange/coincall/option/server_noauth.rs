use std::mem;
use std::mem::ManuallyDrop;
use std::sync::Mutex;

use lazy_static::lazy_static;
use tracing::debug;
use tracing::error;
use tracing::info;

use crate::exchange::coincall::auth::get_cc_token;
use crate::exchange::coincall::Coincall;
use crate::exchange::ExchangeServer;
use crate::ExchangeId;

lazy_static! {
    static ref WEBSOCKET_URL: Mutex<ManuallyDrop<Option<Box<str>>>> =
        Mutex::new(ManuallyDrop::new(None));
}

pub type CoincallServerOptionNoAuth = Coincall<CoincallServerOption>;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CoincallServerOption;

impl ExchangeServer for CoincallServerOption {
    const ID: ExchangeId = ExchangeId::CoincallOption;

    /// Returns the WebSocket URL as a `&'static str`.
    ///
    /// This function generates and returns the WebSocket URL needed to connect
    /// to the Coincall server. It handles memory carefully to ensure that old
    /// URLs are properly cleaned up before a new one is stored and leaked.
    ///
    /// # How It Works
    ///
    /// 1. **Thread-Safe Access**: We start by locking `WEBSOCKET_URL` to ensure
    ///    that only one thread can modify or access the URL at a time.
    ///
    /// 2. **Cleaning Up the Old URL**: If there's an old URL already stored, we
    ///    clear it out using `mem::replace`. This step is crucial because it
    ///    frees the memory associated with the old URL before we store a new
    ///    one. Without this, we'd risk leaking memory with each new URL.
    ///
    /// 3. **Generating and Storing the New URL**: We fetch a new token and
    ///    generate the corresponding WebSocket URL. This new URL is then
    ///    converted into a `Box<str>` and stored in our `url_storage`. By using
    ///    `ManuallyDrop`, we prevent Rust from automatically dropping the URL
    ///    later on, because we're going to leak it intentionally.
    ///
    /// 4. **Leaking the New URL**: Now, here's where we do something a bit
    ///    unusual: We leak the new URL. We grab a mutable reference to the
    ///    stored URL, then use `std::mem::transmute` to convert it into a
    ///    `&'static str`. This effectively makes the URL live for the duration
    ///    of the program. The old URL is gone (freed), and only the latest one
    ///    persists.
    ///
    /// # What's Happening with Memory?
    ///
    /// - **Old URLs**: Every time we generate a new URL, the old one is
    ///   properly cleaned up using `mem::replace`. This avoids memory leaks
    ///   from old URLs sticking around.
    ///
    /// - **New URL**: The new URL is intentionally leaked. This means it’s
    ///   removed from Rust’s usual memory management, allowing it to persist
    ///   until the program ends. The `&'static str` we return stays valid for
    ///   as long as the program runs.
    ///
    /// # Safety
    ///
    /// We use `unsafe` with `std::mem::transmute` to extend the lifetime of the
    /// URL reference to `'static`. This is safe in our specific case
    /// because we ensure that the memory backing the URL won’t be freed
    /// while the program is running. The combination of careful memory
    /// management with `mem::replace` and the controlled leak ensures this
    /// approach works without causing memory issues.
    fn websocket_url() -> &'static str {
        // Acquire a lock on the global URL storage to ensure thread-safe access
        let mut url_storage = WEBSOCKET_URL.lock().unwrap();

        // Check for existing URL. The url_storage.is_some() check should return false
        // on the first run. Clear out the old URL if there is one. The old URL is
        // cleared by replacing the stored value with None.
        if url_storage.is_some() {
            let _ = mem::replace(&mut *url_storage, ManuallyDrop::new(None));
        }

        // Get a new token and generate the URL
        let token = match get_token() {
            Ok(token) => {
                info!("Got token from Coincall: {}", token);
                token
            }
            Err(e) => {
                error!("Exiting. Couldn't get token from Coincall: {}", e);
                std::process::exit(1);
            }
        };
        let generated_url = generate_wss_url(&token);

        // Store the new URL in a Box and prevent it from dropping. The newly generated
        // URL is stored in url_storage, replacing the old one.
        *url_storage = ManuallyDrop::new(Some(generated_url.into_boxed_str()));

        // Get a mutable reference and transmute it to 'static
        let mutable_url_ref: &mut str = url_storage.as_deref_mut().unwrap();

        // Transmute to extend the lifetime to 'static. The transmuted reference is
        // effectively leaked, meaning it stays in memory for the program's
        // entire lifetime.
        //
        // `std::mem::transmute` is necessary here because
        // Rust's strict lifetime system normally wouldn't allow us to return a
        // reference with a `'static` lifetime. But since we control the memory
        // and know it won't be dropped unexpectedly, `transmute` is safe in
        // this context.
        unsafe { std::mem::transmute(mutable_url_ref) }
    }
}

pub fn get_token() -> Result<String, Box<dyn std::error::Error>> {
    debug!("Getting token from Coincall");

    tokio::task::block_in_place(|| {
        let rt = tokio::runtime::Runtime::new()?;
        rt.block_on(get_cc_token())
    })
}

fn generate_wss_url(token: &str) -> String {
    format!(
        "wss://ws.coincall.com/options?code=10&Authorization=Bearer%20{}",
        token
    )
}
