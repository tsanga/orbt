use crate::prelude::*;
use crate::server::Config;
use console::{style, Style};
use dialoguer::{theme::ColorfulTheme, Input, Select};

impl Default for Config {
    fn default() -> Self {
        Config {
            host: "0.0.0.0".to_string(),
            hostname: None,
        }
    }
}

pub fn init() -> Result<Option<Config>> {
    let theme = ColorfulTheme {
        values_style: Style::new().magenta().dim(),
        prompt_prefix: style("?".to_owned()).magenta(),
        prompt_suffix: style("›".to_owned()).magenta().dim(),
        success_prefix: style("✔".to_owned()).magenta(),
        active_item_prefix: style("❯".to_owned()).magenta().dim(),
        active_item_style: Style::new().magenta(),
        ..ColorfulTheme::default()
    };

    let _room_name: String = Input::with_theme(&theme)
        .with_prompt("What should your room be called?")
        .default("Cool Room".parse()?)
        .interact()
        .unwrap();

    let configure = Select::with_theme(&theme)
        .with_prompt("Configure orbt internals?")
        .default(0)
        .item("no  (default configuration)")
        .item("yes (manual configuration)")
        .interact()?;

    // todo @alex: check for existence of config file, if so, present configure option to load from file

    if configure == 0 {
        return Ok(Some(Config::default()));
    }

    println!(
        "{}",
        style("This part isn't done yet, proceeding with default configuration. xd").red()
    );

    Ok(Some(Config::default()))
}
