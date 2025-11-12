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
    pub container_ports: ContainerProperties,
}

#[derive(Clone, Debug)]
pub struct ContainerPorts {
    pub name: String,
    pub port: i32,
}

#[derive(Clone, Debug)]
pub struct  ContainerProperties {
    pub name: String,
    pub container_ports: ContainerPorts,
}

impl ContainerProperties {
    pub fn new(name: &str, container_ports: ContainerPorts) -> Self {
        ContainerProperties {
            name: name.to_string(),
            container_ports,
        }
    }
}


impl AppState {
    pub fn build(config: &Config) -> Self {
        let log = Arc::new(Logger::build(&config.log_output));
        let container_properties = ContainerProperties::new(&config.container_name
            ,ContainerPorts { name: config.container_ports.name.clone(), 
                              port: config.container_ports.port 
                            }
        );

        AppState { log, container_ports: container_properties }
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