mod app;
mod config;
mod daily_view;
mod detail_view;
mod hourly_view;
mod icon_cache;
mod key_bind;
mod localize;
mod location;
mod menu;
mod weather;

use cosmic::{
    app::Settings,
    cosmic_config::{self, CosmicConfigEntry},
    Application,
};

use crate::app::{App, Flags};
use crate::config::{Config, CONFIG_VERSION};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = Settings::default()
        .antialiasing(true)
        .client_decorations(true)
        .debug(false);

    let (config_handler, config) = match cosmic_config::Config::new(App::APP_ID, CONFIG_VERSION) {
        Ok(config_handler) => {
            let config = Config::get_entry(&config_handler).unwrap_or_else(|(errs, config)| {
                log::info!("errors loading config: {:?}", errs);
                config
            });
            (Some(config_handler), config)
        }
        Err(err) => {
            log::error!("failed to create config handler: {}", err);
            (None, Config::default())
        }
    };

    let flags = Flags {
        config_handler,
        config,
    };

    cosmic::app::run::<App>(settings, flags)?;

    Ok(())
}
