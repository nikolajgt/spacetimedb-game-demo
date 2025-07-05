
FROM rust:1.86
WORKDIR /client
COPY . .

# Install ALSA dev libs
RUN apt-get update && \
    apt-get install -y libasound2-dev pkg-config && \
    apt-get clean

RUN cargo build --release

CMD ["./target/release/client"]