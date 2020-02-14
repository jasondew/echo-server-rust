FROM messense/rust-musl-cross:x86_64-musl AS builder

WORKDIR /app
COPY . /app/

RUN cargo build --release --locked --target x86_64-unknown-linux-musl


FROM alpine:3.11

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/echo_server /app/

CMD ["/app/echo_server"]
