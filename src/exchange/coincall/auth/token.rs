use std::error::Error;

use playwright::api::Cookie;
use playwright::Playwright;
use tracing::debug;

pub async fn fetch_token_from_url(url: &str) -> Result<Option<String>, Box<dyn Error>> {
    debug!("Fetching token from URL: {}", url);
    let playwright = initialize_playwright().await?;
    let browser = launch_browser(&playwright).await?;
    let page = open_page(&browser, url).await?;

    let token = extract_token_cookie(&page).await?;

    close_browser(browser).await?;

    Ok(token)
}

async fn initialize_playwright() -> Result<Playwright, Box<dyn Error>> {
    debug!("Initializing Playwright and installing WebKit");
    let playwright = Playwright::initialize().await?;
    playwright.install_webkit()?;
    Ok(playwright)
}

async fn launch_browser(
    playwright: &Playwright,
) -> Result<playwright::api::Browser, Box<dyn Error>> {
    debug!("Launching WebKit browser");
    let browser = playwright
        .webkit()
        .launcher()
        .headless(true)
        .launch()
        .await?;
    Ok(browser)
}

async fn open_page(
    browser: &playwright::api::Browser,
    url: &str,
) -> Result<playwright::api::Page, Box<dyn Error>> {
    debug!("Opening page at {}", url);
    let context = browser.context_builder().build().await?;
    let page = context.new_page().await?;
    page.goto_builder(url).goto().await?;
    Ok(page)
}

async fn extract_token_cookie(
    page: &playwright::api::Page,
) -> Result<Option<String>, Box<dyn Error>> {
    debug!("Extracting token cookie");
    let cookies: Vec<Cookie> = page.context().cookies(&[]).await?;
    let token = cookies
        .iter()
        .find(|&cookie| cookie.name == "token")
        .map(|cookie| cookie.value.clone());

    Ok(token)
}

async fn close_browser(browser: playwright::api::Browser) -> Result<(), Box<dyn Error>> {
    debug!("Closing WebKit browser");
    browser.close().await?;
    Ok(())
}
