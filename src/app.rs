use crate::{config::ToProperties, prelude::*, webhook::mutate};

// handy alias
type DynHandler = BoxEndpoint<'static, Response>;

// 3) Struktura popisující jednu routu
struct RouteDef {
    path: &'static str,
    handler: DynHandler,
}

#[derive(Clone)]
pub struct AppState {
    pub log: Arc<Logger>,
    pub container_properties: Container,
}

#[derive(Clone, Debug)]
pub struct Container {
    pub name: String,
    pub port_name: String,
    pub port_number: u16,
}

impl Container {
    pub fn new(name: &str, port_name: &str, port_number: u16) -> Self {
        Container {
            name: name.to_string(),
            port_name: port_name.to_string(),
            port_number,
        }
    }
}


impl ToProperties <Container> for Config {
    type Output = Container;

    fn to_properties(config: &Config) -> Self::Output {
        Container::new(
            &config.container_patch.name,
            &config.container_patch.port_name,
            config.container_patch.port_number,
        )
    }
}

impl AppState {
    pub fn build(config: &Config) -> Self {
        let log = Arc::new(Logger::build(&config.log_output));
        let container_properties = Config::to_properties(config);

        AppState { log, container_properties }
    }
}

pub async fn builder(config: &Config) -> AddDataEndpoint<Route, AppState> {
    
    let state = AppState::build(&config);
    let log = state.log.clone();



    let routes: Vec<RouteDef> = vec![
        RouteDef {
            path: "/healtz",
            handler: get(up).boxed(),
        },
        RouteDef {
            path: "/mutate",
            handler: post(mutate).boxed(),
        },
    ];

    let api =  routes
    .into_iter()
    .fold(Route::new(), |app, def| app.at(def.path, def.handler));


    let route=  Route::new()
        .nest("/", api)
        .data(state);

    log.info("Webhook initialized".into()).await;
    log.info(format!("Starting server on {}:{}", config.addr, config.port)).await;

    route
}