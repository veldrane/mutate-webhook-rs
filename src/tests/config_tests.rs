use crate::config::Config;
use std::fs;

fn load_config() -> Config {
    Config::default()
        .with_addr("127.0.0.1")
        .with_port(8080)
        .with_log_output("json")
        .with_tls_cert("/etc/webhook/cert.pem")
        .with_tls_key("/etc/webhook/key.pem")
}

#[test]

fn test_config_address() {
    let config = load_config();
    assert_eq!(config.addr, "127.0.0.1");
}

#[test]
fn test_config_port() {
    let config = load_config();
    assert_eq!(config.port, 8080);
}

#[test]
fn test_config_log_output() {
    let config = load_config();
    assert_eq!(config.log_output, "json");
}

#[test]
fn test_config_tls_cert() {
    let config = load_config();
    assert_eq!(config.cert_path, "/etc/webhook/cert.pem");
}

#[test]
fn test_config_tls_key() {
    let config = load_config();
    assert_eq!(config.key_path, "/etc/webhook/key.pem");
}
