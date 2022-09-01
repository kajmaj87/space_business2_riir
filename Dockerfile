FROM ubuntu:20.04

# needed for tz-data package so it does not ask for time zone interactively
ENV TZ=Europe/Warsaw
RUN ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone

RUN apt-get update && apt-get install -y curl pkg-config libudev-dev libasound2-dev build-essential alsa libxcb-render0-dev libxcb-shape0-dev libxcb-xfixes0-dev
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs -o rust_install.sh && chmod +x rust_install.sh && ./rust_install.sh -y 

WORKDIR /app

ADD src/ src/
ADD assets/ assets/
ADD Cargo.toml Cargo.lock ./

RUN ls -la

RUN . "$HOME/.cargo/env" && cargo check