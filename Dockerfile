FROM ubuntu:20.04

# needed for tz-data package so it does not ask for time zone interactively
ENV TZ=Europe/Warsaw
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

RUN apt-get update && apt-get install -y curl pkg-config libudev-dev libasound2-dev build-essential alsa libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev

WORKDIR /app

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rust_install.sh && chmod +x rust_install.sh && ./rust_install.sh -y 
ADD src/ src/
ADD assets/ assets/
ADD Cargo.toml Cargo.lock ./

RUN ls -la

# windows compilation deps
RUN apt-get install -y mingw-w64

RUN . "$HOME/.cargo/env" && rustup target add x86_64-pc-windows-gnu
RUN . "$HOME/.cargo/env" && cargo build --target x86_64-pc-windows-gnu

RUN . "$HOME/.cargo/env" && rustup target add x86_64-pc-windows-msvc
RUN . "$HOME/.cargo/env" && cargo build --target x86_64-pc-windows-msvc
