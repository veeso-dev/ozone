FROM rust:1.70.0-slim-bullseye

RUN apt update && \
  apt install -y libssl-dev pkg-config && \
  cargo install cargo-make

COPY . /app/ozone
WORKDIR /app/ozone

USER root

ENTRYPOINT ["./docker-entrypoint.sh"]
