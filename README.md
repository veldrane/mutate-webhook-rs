# mutate-webhook-rs

A simple **Kubernetes Mutating Admission Webhook** written in Rust.
It automatically injects an additional port definition into container specs based on annotation.

## Overview

This webhook watches for **Pod creation events** and, if the Pod contains the annotation:

```yaml
syscallx86.com/container-port-injector: "true"
```

it adds a container port (for example a metrics port) into the Pod specification.

This is useful when you need to expose internal ports (e.g., injected Envoy sidecars in Consul service mesh) without modifying deployment manifests directly.

## Usage

```bash
mutate-webhook-rs [OPTIONS]

Options:
  -c, --config <CONFIG>  Path to configuration file
  -h, --help             Print help
  -V, --version          Print version
```

Example:

```bash
mutate-webhook-rs -c contrib/config.yaml
```

## Configuration

Example config file (`contrib/config.yaml`):

```yaml
addr: "0.0.0.0"
port: 8443
log: "console"
tls_cert: "/tmp/cert.pem"
tls_key: "/tmp/cert.key"

container_patch:
  name: "simple-api"
  port_name: "metrics"
  port_number: 9101
```

### Fields

- **addr / port** – address and port where the webhook listens  
- **log** – logging backend (`console`)  
- **tls_cert / tls_key** – filesystem paths to the TLS certificate and key  
- **container_patch**
  - `name`: name of the container to mutate
  - `port_name`: name of the injected port
  - `port_number`: container port number to inject

## Annotations

Mutation happens only when the Pod includes the annotation:

```yaml
metadata:
  annotations:
    syscallx86.com/container-port-injector: "true"
```

## TLS and Deployment

In the `contrib/` directory:

- `config.yaml` – sample configuration  
- `self-signed.sh` – script to generate a self‑signed TLS certificate

In the `deploy/` directory:

- `mutatingwebhook.yaml` – example `MutatingWebhookConfiguration`

Deploy the webhook Pod and apply the manifest to register it with the Kubernetes API server.

## Build

```bash
make build
```

## Authors

[-veldrane](https://github.com/veldrane)

## Version History

* 0.5.4
    * Initial public release

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

Built with:
* [Rust](https://www.rust-lang.org/)
* [Tokio](https://tokio.rs/)
* [Hyper](https://hyper.rs/)
* [Poem](https://github.com/poem-web/poem)