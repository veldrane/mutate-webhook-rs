//use std::os::unix::process;
use std::process;

use mutate_webhook_rs::{config::{ConfigLoader, FileConfigLoader}, prelude::*};

use poem::{
     Route,
     Server, 
     listener::{Listener, RustlsCertificate, RustlsConfig, TcpListener}};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {

    set_panic_hook();

    let config = build_config();
    let app: AddDataEndpoint<Route, AppState> = app::builder(&config).await;
    let lsnr= TcpListener::bind(format!("{}:{}", config.addr, config.port))
                                                                        .rustls(RustlsConfig::new()
                                                                            .fallback(RustlsCertificate::new()
                                                                                .key(config.tls_key)
                                                                                .cert(config.tls_cert)
                                                                            ));
    Server::new(lsnr).run(app).await
}


pub fn build_config() -> Config {

    let args = Args::new();
    let config = match args.config {
        Some(c) => {
            let loader = FileConfigLoader { path:  c.to_string()};
            loader.load()
        },
        None => Config::default(),
    };
    config
}

pub fn set_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        eprintln!("Fatal error: {}", info);
        process::exit(1);
    }));
}