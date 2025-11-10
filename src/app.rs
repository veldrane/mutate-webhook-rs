use crate::{prelude::*, webhook::mutate};

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
    pub container_ports: ContainerPorts,
}

#[derive(Clone, Debug)]
pub struct ContainerPorts {
    pub name: String,
    pub port: i32,
}

impl ContainerPorts {
    pub fn new(name: &str, port: i32) -> Self {
        ContainerPorts {
            name: name.to_string(),
            port,
        }
    }
}

impl AppState {
    pub fn build(config: &Config) -> Self {
        let log = Arc::new(Logger::build(&config.log_output));
        let container_ports = ContainerPorts::new(&config.container_ports.name, config.container_ports.port);

        AppState { log, container_ports }
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

    log.info("Application initialized".into()).await;
    log.info(format!("Starting server on {}:{}", config.addr, config.port)).await;

    route
}