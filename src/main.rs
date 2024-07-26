mod commands;

use serenity::builder::CreateInteractionResponse;
use std::env;
use serenity::{async_trait, builder::CreateInteractionResponseMessage};
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::model::id::GuildId;
use serenity::model::application::Interaction;
use tracing::{error, info};


use serde_json;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Weather {
    pub name: String,
    pub wind_speed: String,
    pub temperature: String,
    pub rain_chance: String,
    pub forecast: String,
    pub tomorrow_forecast: String,
}

impl Weather {
    pub fn build(input: serde_json::Value) -> Weather {
        let name = input["properties"]["periods"][0]["name"].to_string();
        let wind_speed = input["properties"]["periods"][0]["windSpeed"].to_string();
        let temperature = input["properties"]["periods"][0]["temperature"].to_string();
        // if rain chance returns "null" then it means "0" percent chance
        let rain_chance = input["properties"]["periods"][0]["probabilityOfPrecipitation"]["value"].to_string();
        let forecast = input["properties"]["periods"][0]["detailedForecast"].to_string();
        let tomorrow_forecast = input["properties"]["periods"][1]["detailedForecast"].to_string();

        Weather {
            name,
            wind_speed,
            temperature,
            rain_chance,
            forecast,
            tomorrow_forecast,
        }
    }
}

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!hello" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "worldw!").await {
                error!("Error sending message: {:?}", e);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    
        let guild_id = GuildId::new(
            env::var("GUILD_ID")
            .expect("Expected GUILD_ID in environment")
            .parse()
            .expect("GUILD_ID must be an integer"),
        );

        let commands = guild_id
            .set_commands(&ctx.http, vec![
                commands::forecast::register(),
                commands::weather::register(),
                commands::tomorrow::register(),
            ])
            .await;

        println!("I now have these commands: {commands:#?}")
    }

    async fn interaction_create(&self, ctx:Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
  
    let app_user_agent: &str = concat!(
        "Daniel Simpson",
        "/",
        "daniel.t.simpson16@gmail.com",
    );

    let url = "https://api.weather.gov/gridpoints/IND/61,66/forecast";

    let weather_api = reqwest::Client::builder()
        .user_agent(app_user_agent)
        .build()
        .unwrap();

    let date = match weather_api.get(url).send().await {
        Ok(resp) => {
            let json: serde_json::Value = resp.json().await.unwrap();
            let date = Weather::build(json);
            date
        }
        Err(err) => {
            println!("Reqwest error: {}", err);
            Weather {
                name: "error struct".to_string(),
                wind_speed: "error struct".to_string(),
                temperature: "error struct".to_string(),
                rain_chance: "error struct".to_string(),
                forecast: "error struct".to_string(),
                tomorrow_forecast: "error struct".to_string(),

            }
        }
    };

    let content = match command.data.name.as_str() {
        "forecast" => Some(commands::forecast::run(&command.data.options(), date)),
        "weather" => Some(commands::weather::run(&command.data.options(), date)),
        "tomorrow" => Some(commands::tomorrow::run(&command.data.options(), date)),
        command => unreachable!("Unknown command: {}", command),
    };

    if let Some(content) = content {
        let data = CreateInteractionResponseMessage::new().content(content);
        let builder = CreateInteractionResponse::Message(data);
        if let Err(error) = command.create_response(&ctx.http, builder).await {
            println!("Cannot respond to slash command: {error}");
        }
    }

        }
    }
}

#[shuttle_runtime::main]
async fn serenity(
) -> shuttle_serenity::ShuttleSerenity {
    // Get the discord token set in environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(&token, GatewayIntents::empty())
        .event_handler(Bot)
        .await
        .expect("Error creating client");

    if let Err(error) = client.start().await {
        println!("Client error: {error:?}");
    }

    Ok(shuttle_serenity::SerenityService(client))
}
