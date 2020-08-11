FROM rust:latest as builder
WORKDIR /user/src/myapp
COPY . .
RUN cargo build
CMD cargo run 1000 192.168.1.141