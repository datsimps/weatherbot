use crate::Weather;

use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;


pub fn run(_options: &[ResolvedOption], date: Weather) -> String {
    let forecast: String = format!("{}:\n{}", date.name, date.forecast).to_string();
    forecast
}

pub fn register() -> CreateCommand {
    CreateCommand::new("forecast").description("Get the forecast")
}
