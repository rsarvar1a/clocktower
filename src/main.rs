mod factory;
mod models;
mod services;
mod utils;

use actix_session::{config::BrowserSession, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie, middleware, web, App, HttpServer};
use anyhow::{Context, Error, Result};
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use sqlx::postgres::PgPoolOptions;

use factory::{app, cli, config};

#[actix_web::main]
async fn main() -> Result<()>
{
    let cli_options = cli::parse();

    config::load(&cli_options.config_path).context("failed to load config")?;
    let cfg = config::get()?;

    // TODO: roll Postgres handle into a proper AppState, plug that into App.app_data() instead...
    // (along with the Clocktower-online relevant appstate as we decide on what that'll look like)
    let pool = PgPoolOptions::new().max_connections(cfg.db.max_connections).connect(&cfg.db.postgres_url).await?;

    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::NormalizePath::trim())
            .wrap(middleware::Logger::default())
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), cookie::Key::from(&[0; 64]))
                    .session_lifecycle(BrowserSession::default())
                    .build(),
            )
            .service(web::scope(&cfg.server.root).configure(app::routes))
    });

    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder.set_private_key_file(cfg.ssl.key_file, SslFiletype::PEM).context("failed to set key")?;
    builder.set_certificate_chain_file(cfg.ssl.cert_file).context("failed to set cert")?;

    let server_addr = format!("{}:{}", cfg.server.host, cfg.server.port);
    server.bind_openssl(server_addr, builder)?.run().await.map_err(Error::from)
}
