use serde_yaml::Value;
use std::fs::File;
use std::io::Read;

const CERT: &str = r#"
-----BEGIN CERTIFICATE-----
MIIDUTCCAjmgAwIBAgIUeq20D4nOVjme2/whTbakViG+ng8wDQYJKoZIhvcNAQEL
BQAwJjEkMCIGA1UEAwwbYnVpbGQudnhsYW5kLnN5c2NhbGx4ODYuY29tMB4XDTI1
MTEwNzEzNDAxMVoXDTM1MTEwNTEzNDAxMVowJjEkMCIGA1UEAwwbYnVpbGQudnhs
YW5kLnN5c2NhbGx4ODYuY29tMIIBIjANBgkqhkiG9w0BAQEFAAOCAQ8AMIIBCgKC
AQEA5OuOKP20rch2wkZ8YysnXeL3monJiZNfJCf7jsSY9v5icQLGu/ANYFgUGD2q
e/3sCkLjxzJ5Eis81XKU2kAq08rgw7kVd9ps8vjC5hGCk2VxKykrcuOaNUPoWczL
V2sArWAKwHsyOvS4T2Tpq48q+YIJhiek3CCgcUYCMJyPenM3hCoeiXVfcbNFvFYz
kCO+iv0HiFKsNcGUwWXgprCiKgfvX1vLQQWgJpIGaD2qvt1++j4Fya6yEltejsCR
tn8g9VXCpF+/LJNPRFwu5027WRyENbS8Xc+O+xfSjUjxlHkigvzuqshBjK0a5xfY
AZbc5sbY1ghT3Li8mk7+6lnc1QIDAQABo3cwdTAdBgNVHQ4EFgQUQOxaDB3wJS97
Ju37fYeBBiPeEt8wHwYDVR0jBBgwFoAUQOxaDB3wJS97Ju37fYeBBiPeEt8wDwYD
VR0TAQH/BAUwAwEB/zAiBgNVHREEGzAZghcqLnZ4bGFuZC5zeXNjYWxseDg2LmNv
bTANBgkqhkiG9w0BAQsFAAOCAQEAO+g5WfyhIFiGwafxoQa5vjjqvbnI8tISMoQA
EU0rEhbjEnT9ZwocJGXjxgXrSTrBLiy8Z2PDtkMJawvSrtO/swKo59RWu1e2475G
B3LpZdFTFu3qmTnnYIzN78pygNgoiF/xlBXZ7MeXaNw1VGzuQ3rad4sleW8XfgWK
cWXnkGYBpDl2mqiMVWMtTes3hqS6cI4JYRsSCItI4CazsnSM8qVNdFBlCNnYDG8L
cNKDpFauNi5KulBfNTgm82G5wkMbltG8xVAin1cb06nazGJyHdGXJ52VvLVt80QU
3bgUYkhlKwohtFX4i35CXjLrXDRhXjJxvy8BPIwezZwpoUCgYg==
-----END CERTIFICATE-----
"#;

const KEY: &str = r#"
-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQDk644o/bStyHbC
RnxjKydd4veaicmJk18kJ/uOxJj2/mJxAsa78A1gWBQYPap7/ewKQuPHMnkSKzzV
cpTaQCrTyuDDuRV32mzy+MLmEYKTZXErKSty45o1Q+hZzMtXawCtYArAezI69LhP
ZOmrjyr5ggmGJ6TcIKBxRgIwnI96czeEKh6JdV9xs0W8VjOQI76K/QeIUqw1wZTB
ZeCmsKIqB+9fW8tBBaAmkgZoPaq+3X76PgXJrrISW16OwJG2fyD1VcKkX78sk09E
XC7nTbtZHIQ1tLxdz477F9KNSPGUeSKC/O6qyEGMrRrnF9gBltzmxtjWCFPcuLya
Tv7qWdzVAgMBAAECggEAAX5QRWF1+DMsr4c6Xz0RSX89eEhaBIIztSXbzxX9zAVA
pSeMe3KqSAnN+Hua3nwHItGjQd2DP2pHKN2uxOp1Vz/UCBip1I3hyJ6MeSIgp9sl
pzgoLK35cGL5THBQCuyXJP7F2+k9eaZCgwLPy83oHlHbfd3shjoE/ZJeiAPtdMlO
1rPTmvpBU6/da485RMlcie0M+NtqB90NM34hUhMJD96TSot/PMSxKB0viWjQ5x+7
y/OpiFKty6ocsURopxa8j+KkLnptgCS1g5hoIJKQ5JXu6IeRCCXV6ko/zva/JcCI
sxlBlE6UylnJixSf/TgjZVLyJbcdi18sBwKL+ARZvQKBgQD97/A4HNEusC/GJfcf
iV/DH4DYm4zIzYezIBNNdWlRzCBp4+ffowuj9exSyluFMS+VpgD0PO+pIYMlt2xx
GIJR5Gu7+p5gqUL0YvC5kaXqs0hkS3YtCskfAbMrTkyu7WFT9ltVif+yg7ceYMAp
R70OiNy0n5Zb7GXdN3WXLh3uewKBgQDmx5gMe1AgWgenH8G0mT/XRG4XdwUnEJf1
OhT1mCmC/gvzM8g0Zh2hIV0+As8uIqnScA0mPkpCRW9VPvaorLk/lqTbC3q6N75o
FsHmu02O2x00fGK/nUwTCSKDI1PU1Y1UgxPULXqpJjoJgS3x4ZC3AqSeui4LIlq+
giGYtt8o7wKBgFGMGgKLDooFvnHNg1y3Su9oUII/AakCh894P3qid93yxX372Fyz
CWvv76JupZFSSEuwcaH9Z6FSx8D29HYlPsR5rDdDtlSoRn4gv9l38mY6iMbcjOjO
C+RPMXB4xptuU9EYzh/cHyb+sXGp9EvNY+MlBGtkzG5criGqS7Lu9mG9AoGBAJaE
Z278nm37SjJ+S+B2c2T7hLJZNkscT3/pufHUpH6DI1gj1dgXIgwNrrrbKjV09j4C
RxFCXzyJ/OskfcvVm50Vq8AR5KG/6bgJ372VPCiIUKwC1IUVGqDvvEq7p9mQzvTn
6O7iGrZn8EJytnykGbbDSosb8xvf9D98sFziKPqRAoGAdS/umLML0pSmf8OlViJf
hzOYJMNTsQkiAmdyg4d24+0uIjn2elEqU8qkMkuJ4iCrPj6upkYJN0K8oAx1E+px
CGpkTg61Co2hw/N4BgMUoI8CiMnT5+VuxwZvLmcmJsnbm4SW3MVH9JE3aQP5eIyF
o7R7oq6hcPDqe3fd78WSCWw=
-----END PRIVATE KEY-----
"#;

#[derive(Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub addr: String,
    pub log_output: String,
    pub container_ports: ContainerPorts,
    pub container_name: String,
    pub tls_cert:  String,
    pub tls_key: String,
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
    pub fn with_tls_cert(mut self, cert: &str) -> Self {

        let mut f = File::open(cert).expect("Unable to open TLS cert file");
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).expect("Unable to read TLS cert file");

        self.tls_cert = buffer;
        self
    }
    pub fn with_tls_key(mut self, key: &str) -> Self {

        let mut f = File::open(key).expect("Unable to open TLS key file");
        let mut buffer = String::new();
        f.read_to_string(&mut buffer).expect("Unable to read TLS key file");

        self.tls_key = buffer;
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
            container_name: "simple-api".to_string(),
            tls_cert: CERT.to_string(),
            tls_key: KEY.to_string(), 
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
                            Value::String(s) if s == "tls_cert" => {
                                config = config.with_tls_cert(v.as_str().unwrap());
                            },
                            Value::String(s) if s == "tls_key" => {
                                config = config.with_tls_key(v.as_str().unwrap());
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

