pub mod config;
pub mod entries;
pub mod output;
pub mod search;

use std::io;

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::parse_args()
        .map_err(|msg| io::Error::new(io::ErrorKind::InvalidInput, msg))?;

    let entries = entries::read_entries(&config.path)?;
    let matched_entries = search::filter_entries(entries, &config);

    output::print_entries(matched_entries)?;

    Ok(())
}