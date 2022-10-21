FROM rust:latest

RUN apt-get -y update && apt-get install -y

COPY . /usr/backend/

WORKDIR /usr/backend/

RUN cargo install diesel_cli

RUN cargo build --release

CMD ["./target/release/rest01"]