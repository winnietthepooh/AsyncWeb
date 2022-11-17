FROM rust:1.65.0 as builder

WORKDIR /usr/src/AsyncWeb

COPY . .

RUN cargo install --path .

FROM debian:buster-slim as runner
RUN apt-get install -y extra-runtime-dependencies  \
RUN apt-get update
RUN apt-get upgrade
COPY --from=builder /usr/local/cargo/bin/AsyncWeb /usr/local/bin/AsyncWeb
CMD ["AsyncWeb"]
