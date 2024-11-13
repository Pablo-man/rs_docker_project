FROM rust:1.72 as builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bookworm-slim

COPY --from=builder /app/target/release/rust /usr/local/bin/rust

EXPOSE 9000

CMD ["rust"]
