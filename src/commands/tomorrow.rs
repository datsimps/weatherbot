use crate::Weather;

use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;

pub fn run(_options: &[ResolvedOption], date: Weather) -> String {
    let tomorrow: String = "Tomorrow".to_string();
    let forecast: String = format!("{:?}:\n{}", tomorrow, date.tomorrow_forecast).to_string();
    forecast
}

pub fn register() -> CreateCommand {
    CreateCommand::new("tomorrow").description("Get tomorrow's forecast")
}
