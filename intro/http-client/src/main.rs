use anyhow::{bail, Result};
use core::str;
use embedded_svc::{
    http::{client::Client, Method},
    io::Read,
};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::prelude::Peripherals,
    http::client::{Configuration, EspHttpConnection},
};
use wifi::wifi;

#[toml_cfg::toml_config]
pub struct Config {
    #[default("")]
    wifi_ssid: &'static str,
    #[default("")]
    wifi_psk: &'static str,
}

fn main() -> Result<()> {
    esp_idf_svc::sys::link_patches();
    esp_idf_svc::log::EspLogger::initialize_default();

    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take()?;

    // The constant `CONFIG` is auto-generated by `toml_config`.
    let app_config = CONFIG;

    // Connect to the Wi-Fi network
    let _wifi = wifi(
        app_config.wifi_ssid,
        app_config.wifi_psk,
        peripherals.modem,
        sysloop,
    )?;

    get("http://neverssl.com/")?;

    Ok(())
}

fn get(url: impl AsRef<str>) -> Result<()> {
    // 1. Create a new EspHttpConnection with default Configuration. (Check documentation)

    // 2. Get a client using the embedded_svc Client::wrap method. (Check documentation)

    // 3. Open a GET request to `url`
    let headers = [("accept", "text/plain")];

    // 4. Submit the request and check the status code of the response.
    // let response = request...;
    // let status = ...;
    // println!("Response code: {}\n", status);
    // match status {
    // Successful http status codes are in the 200..=299 range.

    // 5. If the status is OK, read response data chunk by chunk into a buffer and print it until done.

    // 6. Try converting the bytes into a Rust (UTF-8) string and print it.
    // }

    Ok(())
}
