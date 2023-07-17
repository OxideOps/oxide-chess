FROM catthehacker/ubuntu:rust-latest
LABEL org.opencontainers.image.source=https://github.com/Event-Horizon-Technologies/chess
WORKDIR /root
RUN apt-get update
RUN apt-get upgrade -y
RUN apt-get install -y libjavascriptcoregtk-4.1-dev libgtk-3-dev libsoup-3.0-dev libwebkit2gtk-4.1-dev
RUN rustup target add wasm32-unknown-unknown
RUN rustup component add rustfmt
RUN cargo install --locked trunk
