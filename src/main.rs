mod commands;
mod events;
mod handlers;
mod structs;
mod utils;

use std::sync::Arc;

use events::event_handler;
use handlers::db::DatabaseController;
use serde::{Deserialize, Serialize};
use serenity::{
    all::{ClientBuilder, GatewayIntents},
    futures::lock::Mutex,
};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use structs::vouch::Vouch;
use tracing::{event, info, info_span, Level};

fn check_required_env_vars() {
    let env_span = info_span!("check_required_env_vars");

    let required_vars = vec!["DATABASE_URL", "BOT_TOKEN"];
    // Enter into the span then check the required environment variables
    let _enter = env_span.enter();
    for var in required_vars {
        info!("checking {}", var);
        if std::env::var(var).is_err() {
            event!(
                Level::ERROR,
                "required environment variable {} is not set",
                var
            );
            panic!(
                "required environment variable {} is not set, cannot continue",
                var
            );
        }
    }

    info!("all required environment variables are set");

    // Exit the span
    drop(_enter);
}

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("./migrations");

async fn init_sqlx() -> MySqlPool {
    let sqlx_span = info_span!("init_sqlx");

    // Enter into the span then initialize SQLx
    let _enter = sqlx_span.enter();

    // Create a pooled connection to the database
    info!("creating a database connection pool");
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .expect("failed to create a database connection pool");

    info!("database connection pool created");

    // Ensure database schema is up to date
    info!("running database migrations");
    MIGRATOR
        .run(&pool)
        .await
        .expect("Migrations did not succeed");

    info!("database migrations completed");
    info!("SQLx initialized");

    // Exit the span
    drop(_enter);
    pool
}

struct Data {
    database_controller: DatabaseController,
    owners: Vec<u64>,
    uptime: std::time::Instant,
    config: Config,
    vouch_store: Mutex<Vec<Vouch>>,
} // User data, which is stored and accessible in all command invocations

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Deserialize, Serialize)]
struct Config {
    main_guild_id: u64,
    channels: Channels,
    roles: Roles,
}

#[derive(Deserialize, Serialize)]
struct Channels {
    welcome: u64,
    main: u64,
    logs_public: u64,
    logs_mod: u64,
    starboard: u64,
}

#[derive(Deserialize, Serialize)]
struct Roles {
    admin: u64,
    silly_role: u64,
}

#[tokio::main]
async fn main() {
    let start_time = std::time::Instant::now();

    tracing_subscriber::fmt::init();
    let init_span = info_span!("init");
    let main_span = info_span!("main");

    let _enter = init_span.enter();
    info!("loading dotenv");
    dotenv::dotenv().ok();

    info!("checking required environment variables");
    check_required_env_vars();

    info!("loading config");
    // Do we have a config.toml file? If we do, load it
    // If we don't, create it with the default values, then exit the program and tell the user to fill it out
    let config: Config = match std::fs::read_to_string("config.toml") {
        Ok(config) => toml::from_str(&config).expect("failed to parse config.toml"),
        Err(_) => {
            let default_config = Config {
                main_guild_id: 0,
                channels: Channels {
                    welcome: 0,
                    main: 0,
                    logs_public: 0,
                    logs_mod: 0,
                    starboard: 0,
                },
                roles: Roles {
                    admin: 0,
                    silly_role: 0,
                },
            };
            let default_config_toml = toml::to_string_pretty(&default_config).unwrap();
            std::fs::write("config.toml", default_config_toml)
                .expect("failed to write config.toml");
            event!(Level::WARN, "config.toml not found, created a default one");
            event!(
                Level::ERROR,
                "please fill out config.toml and restart the bot"
            );

            panic!("config.toml not found, created a default one, please fill it out and restart the bot");
        }
    };

    info!("initializing SQLx");
    let pool = init_sqlx().await;

    info!("initializing bot");
    let intents = GatewayIntents::privileged()
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_MESSAGES;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions::<Data, Error> {
            commands: vec![
                commands::ping::ping(),
                commands::vouch::vouch(),
                commands::profile::profiles(),
                commands::dog::dog(),
                commands::cta::cta(),
                commands::action::use_action_hug(),
                commands::action::use_action_kiss(),
                commands::action::use_action_pat(),
                commands::eval::eval(),
                commands::quote::quote_action(),
                commands::quote::random_quote(),
                commands::quote::user_quotes(),
            ],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            prefix_options: poise::PrefixFrameworkOptions {
                prefix: Some("~".into()),
                edit_tracker: Some(Arc::new(poise::EditTracker::for_timespan(
                    std::time::Duration::from_secs(3600),
                ))),
                case_insensitive_commands: true,
                ..Default::default()
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    // Initialize user data here
                    database_controller: DatabaseController::new(pool.clone()),
                    uptime: std::time::Instant::now(),
                    config,
                    vouch_store: Mutex::new(Vec::new()),
                    // Sticks, Emi, Katie
                    owners: vec![1017196087276220447, 272871217256726531, 1033331958291369984],
                })
            })
        })
        .build();

    let mut client = ClientBuilder::new(std::env::var("BOT_TOKEN").unwrap(), intents)
        .framework(framework)
        .await
        .expect("Error creating client");

    info!("bot initialized");
    drop(_enter);

    let _enter = main_span.enter();

    info!("init done in {:?}", start_time.elapsed());
    info!("starting bot");
    if let Err(why) = client.start().await {
        event!(Level::ERROR, "Client error: {:?}", why);
    }
}
