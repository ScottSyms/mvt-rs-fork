use crate::error::AppResult;
use clap::{Arg, Command};
use std::env;

#[derive(Debug)]
pub struct AppConfig {
    pub config_dir: String,
    pub cache_dir: String,
    pub map_assets_dir: String,
    pub host: String,
    pub port: String,
    pub redis_conn: String,
    pub jwt_secret: String,
    pub session_secret: String,
    pub config_cli: bool,
}

pub async fn parse_args() -> AppResult<AppConfig> {
    dotenvy::dotenv().ok();

    let matches = Command::new("mvt-server: a vector tiles server")
        .arg(
            Arg::new("configdir")
                .short('c')
                .long("config")
                .value_name("CONFIGDIR")
                .help("Directory where config file is placed"),
        )
        .arg(
            Arg::new("cachedir")
                .short('b')
                .long("cache")
                .value_name("CACHEDIR")
                .help("Directory where cache files are placed"),
        )
        .arg(
            Arg::new("mapassetsdir")
                .short('m')
                .long("mapassets")
                .value_name("MAPASSETS")
                .help("Directory where map_assets files are placed"),
        )
        .arg(
            Arg::new("host")
                .short('i')
                .long("host")
                .value_name("HOST")
                .help("Bind address"),
        )
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_name("PORT")
                .help("Bind port"),
        )
        .arg(
            Arg::new("redisconn")
                .short('r')
                .long("redisconn")
                .value_name("REDISCONN")
                .help("Redis connection"),
        )
        .arg(
            Arg::new("jwtsecret")
                .short('j')
                .long("jwtsecret")
                .value_name("JWTSECRET")
                .help("JWT secret key"),
        )
        .arg(
            Arg::new("sessionsecret")
                .short('s')
                .long("sessionsecret")
                .value_name("SESSIONSECRET")
                .help("Session secret key"),
        )
        .arg(
            Arg::new("config_cli")
                .long("config-cli")
                .short('C')
                .action(clap::ArgAction::SetTrue)
                .help("Enter to cli where you can set config values interactively"),
        )
        .get_matches();

    let config_cli = matches.get_flag("config_cli");

    let get_value = |key: &str, arg_name: &str, default: Option<&str>| -> String {
        matches
            .get_one::<String>(arg_name)
            .cloned()
            .or_else(|| env::var(key).ok())
            .or(default.map(String::from))
            .unwrap_or_else(|| {
                if !config_cli {
                    panic!("Missing required config value for '{key}'. Provide via CLI, env var, or default.")
                } else {
                    String::from("")
                }
            })
    };

    let config_dir = get_value("CONFIG", "configdir", Some("config"));
    let cache_dir = get_value("CACHE", "cachedir", Some("cache"));
    let map_assets_dir = get_value("MAPASSETS", "mapassetsdir", Some("map_assets"));
    let host = get_value("IPHOST", "host", Some("0.0.0.0"));
    let port = get_value("PORT", "port", Some("5800"));
    let redis_conn = get_value("REDISCONN", "redisconn", Some(""));
    let jwt_secret = get_value("JWTSECRET", "jwtsecret", None);
    let session_secret = get_value("SESSIONSECRET", "sessionsecret", None);

    Ok(AppConfig {
        config_dir,
        cache_dir,
        map_assets_dir,
        host,
        port,
        redis_conn,
        jwt_secret,
        session_secret,
        config_cli,
    })
}
