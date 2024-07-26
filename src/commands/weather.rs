use crate::Weather;

use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;


pub fn run(_options: &[ResolvedOption], date: Weather) -> String {
    let forecast: String = format!("{}:\nWind speed: {}\nTemperature: {}\nChance of rain: {}\n",
                                    date.name, date.wind_speed, date.temperature, date.rain_chance).to_string();
    forecast
}

pub fn register() -> CreateCommand {
    CreateCommand::new("weather").description("Get forecast details")
}
