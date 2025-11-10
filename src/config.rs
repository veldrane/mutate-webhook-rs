use serde_yaml::Value;

#[derive(Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub addr: String,
    pub log_output: String,
    pub container_ports: ContainerPorts,
}

#[derive(Clone, Debug)]
pub struct ContainerPorts {
    pub name: String,
    pub port: i32,
}

impl Default for ContainerPorts {
    fn default() -> Self {
        ContainerPorts {
            name: "metrics".to_string(),
            port: 9200,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        Config::default()
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    pub fn with_addr(mut self, addr: &str) -> Self {
        self.addr = addr.into();
        self
    }
    pub fn with_log_output(mut self, output: &str) -> Self {
        self.log_output = output.into();
        self
    }

}

impl Default for Config {
    fn default() -> Self {
        Config {
            port: 8443,
            addr: String::from("0.0.0.0"),
            log_output: String::from("console"),
            container_ports: ContainerPorts::default(),
        }
    }
}

pub trait ConfigLoader {
    fn load(&self) -> Config;
}

pub struct FileConfigLoader {
    pub path: String,
}

impl ConfigLoader for FileConfigLoader {
    fn load(&self) -> Config {

        let r = std::fs::File::open(&self.path).unwrap();
        let file_value: Value = serde_yaml::from_reader(r).unwrap();
        let mut config = Config::default();

        if let Value::Mapping(map) = file_value {
            for (k, v) in map {
                match v {
                    Value::String(_) => {
                        match k {
                            Value::String(s) if s == "addr" => {
                                config = config.with_addr(v.as_str().unwrap());
                            },
                            Value::String(s) if s == "log" => {
                                config = config.with_log_output(v.as_str().unwrap());
                            },
                            _ => continue,
                        }
                    },
                    Value::Number(_) => {
                        match k {
                            Value::String(s) if s == "port" => {
                                config = config.with_port(v.as_u64().unwrap() as u16);
                            },
                            _ => continue,
                        }
                    }

                    _ => continue,
                    }
                }
            }
            config
    }
}

