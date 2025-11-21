use crate::app::{AppState, Container};
use crate::config::{Config, ContainerPatch};

fn load_config() -> Config {
    Config::default()
        .with_addr("127.0.0.1")
        .with_port(8080)
        .with_log_output("console")
        .with_tls_cert("/etc/webhook/cert.pem")
        .with_tls_key("/etc/webhook/key.pem")
        .with_container_patch(ContainerPatch {
            name: "app-container".to_string(),
            port_name: "http".to_string(),
            port_number: 8080,
        })
}

#[tokio::test]
async fn test_app_state_build() {
    let config = load_config();
    let app_state = AppState::build(&config);

    assert_eq!(app_state.container_properties.name, "app-container");
    assert_eq!(app_state.container_properties.port_name, "http");
    assert_eq!(app_state.container_properties.port_number, 8080);
}

#[tokio::test]
async fn test_app_state_container_properties() {
    let config = load_config();
    let app_state = AppState::build(&config);

    assert_eq!(
        app_state.container_properties,
        Container {
            name: "app-container".to_string(),
            port_name: "http".to_string(),
            port_number: 8080,
        }
    );
}
