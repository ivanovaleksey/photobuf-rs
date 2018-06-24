FROM rust:latest

WORKDIR /app
COPY . .

RUN set -xe \
    && apt-get update \
    && apt-get install -y \
        mingw-w64

RUN mkdir .cargo
ADD docker/cargo_config .cargo/config

RUN rustup target add x86_64-pc-windows-gnu

CMD cargo build --release --target=x86_64-pc-windows-gnu
