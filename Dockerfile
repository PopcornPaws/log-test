FROM rust:1.43

WORKDIR /usr/src/log-test

COPY . .

RUN cargo install --path .

CMD = ["log-test"]
