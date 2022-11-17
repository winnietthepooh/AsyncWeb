FROM rust:1.65.0 as builder

WORKDIR /usr/src/AsyncWeb

COPY src ./src
COPY Cargo.toml .

RUN cargo install --path .

FROM debian:bullseye-slim as runner
RUN apt-get update
RUN apt-get upgrade
RUN apt-get -y install ca-certificates
RUN apt-get -y install libssl-dev
COPY --from=builder /usr/local/cargo/bin/AsyncWeb /usr/local/bin/AsyncWeb
CMD ["AsyncWeb"]
