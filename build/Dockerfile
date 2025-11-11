FROM ustclug/rocky:9-minimal

LABEL MANTAINER "-veldrane"

COPY ../target/release/mutate-webhook-rs /mutate-webhook-rs

RUN microdnf install -y net-tools iproute iputils bash openssl \
    && chmod +x /mutate-webhook-rs 

ENTRYPOINT ["/mutate-webhook-rs"]
