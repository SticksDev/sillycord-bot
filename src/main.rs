mod commands;
mod events;

use events::event_handler;
use serenity::all::{ClientBuilder, GatewayIntents};
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
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
    sqlx_pool: MySqlPool,
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

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

    info!("initializing SQLx");
    let pool = init_sqlx().await;

    info!("initializing bot");
    let intents = GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions::<Data, Error> {
            commands: vec![commands::ping::ping()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    // Initialize user data here
                    sqlx_pool: pool,
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
