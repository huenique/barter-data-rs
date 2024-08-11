use std::error::Error;
use std::future::Future;
use std::time::Duration;

use playwright::api::Browser;
use playwright::api::Cookie;
use playwright::api::Page;
use playwright::Playwright;
use tokio::time::sleep;
use tokio::time::timeout;
use tracing::debug;
use tracing::error;
use tracing::warn;

const MAX_ATTEMPTS: u32 = 10;
const RETRY_DELAY_MS: u64 = 500;
const MAX_RETRY_DELAY_MS: u64 = 5000; // Maximum backoff delay
const TOTAL_TIMEOUT_SECS: u64 = 30; // Total timeout for all attempts
const CIRCUIT_BREAKER_THRESHOLD: u32 = 5; // Number of failures before breaking
const CIRCUIT_BREAKER_COOLDOWN_SECS: u64 = 10; // Cooldown period before retrying after circuit is broken

pub async fn fetch_token_from_url(url: &str) -> Result<Option<String>, Box<dyn Error>> {
    debug!("Fetching token from URL: {}", url);
    let playwright = initialize_playwright().await?;
    let browser = launch_browser(&playwright).await?;
    let page = open_page(&browser, url).await?;

    let token = timeout(Duration::from_secs(TOTAL_TIMEOUT_SECS), async {
        retry_with_delay(MAX_ATTEMPTS, RETRY_DELAY_MS, || async {
            extract_token_cookie(&page).await
        })
        .await
    })
    .await??;

    close_browser(browser).await?;

    Ok(token)
}

async fn initialize_playwright() -> Result<Playwright, Box<dyn Error>> {
    debug!("Initializing Playwright and installing WebKit");
    let playwright = Playwright::initialize().await?;
    playwright.install_webkit()?;
    Ok(playwright)
}

async fn launch_browser(playwright: &Playwright) -> Result<Browser, Box<dyn Error>> {
    debug!("Launching WebKit browser");
    playwright
        .webkit()
        .launcher()
        .headless(true)
        .launch()
        .await
        .map_err(Into::into)
}

async fn open_page(browser: &Browser, url: &str) -> Result<Page, Box<dyn Error>> {
    debug!("Opening page at {}", url);
    let context = browser.context_builder().build().await?;
    let page = context.new_page().await?;
    page.goto_builder(url).goto().await?;
    Ok(page)
}

async fn extract_token_cookie(page: &Page) -> Result<Option<String>, Box<dyn Error>> {
    let cookies: Vec<Cookie> = page.context().cookies(&[]).await?;
    let token = cookies
        .iter()
        .find(|&cookie| cookie.name == "token")
        .map(|cookie| cookie.value.clone());
    if let Some(ref token_value) = token {
        debug!("Token cookie found: {}", token_value);
    }
    Ok(token)
}

async fn close_browser(browser: Browser) -> Result<(), Box<dyn Error>> {
    debug!("Closing WebKit browser");
    browser.close().await?;
    Ok(())
}

async fn retry_with_delay<F, Fut, T>(
    max_attempts: u32,
    initial_delay_ms: u64,
    mut operation: F,
) -> Result<Option<T>, Box<dyn Error>>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<Option<T>, Box<dyn Error>>>,
{
    let mut delay = initial_delay_ms;
    let mut failures = 0;

    for attempt in 1..=max_attempts {
        if failures >= CIRCUIT_BREAKER_THRESHOLD {
            warn!(
                "Circuit breaker tripped after {} failures. Cooling down for {} seconds...",
                CIRCUIT_BREAKER_THRESHOLD, CIRCUIT_BREAKER_COOLDOWN_SECS
            );
            sleep(Duration::from_secs(CIRCUIT_BREAKER_COOLDOWN_SECS)).await;
            failures = 0; // Reset failures after cooldown
        }

        match operation().await? {
            Some(result) => return Ok(Some(result)),
            None => {
                failures += 1;
                if attempt < max_attempts {
                    warn!("Attempt {} failed. Retrying in {} ms...", attempt, delay);
                    sleep(Duration::from_millis(delay)).await;
                    // Exponential backoff with max cap
                    delay = (delay * 2).min(MAX_RETRY_DELAY_MS);
                }
            }
        }
    }

    error!("Operation failed after {} attempts", max_attempts);
    Ok(None)
}
