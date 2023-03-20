use std::{fs::File, path::Path};

use anyhow::{Context, Result};
use log::LevelFilter;
use simplelog::{
    ColorChoice, CombinedLogger, Config, ConfigBuilder, TermLogger, TerminalMode, WriteLogger,
};

pub fn create_logger(path: impl AsRef<Path>) -> Result<()> {
    let name = path.as_ref().display().to_string();
    let error_message = format!("Failed to create log file name: {name}");
    let config = ConfigBuilder::new().add_filter_allow_str("riddle").build();

    let file = File::create(path).context(error_message)?;
    CombinedLogger::init(vec![
        TermLogger::new(
            LevelFilter::Info,
            config.clone(),
            TerminalMode::Mixed,
            ColorChoice::Auto,
        ),
        WriteLogger::new(LevelFilter::max(), config, file),
    ])?;

    Ok(())
}
