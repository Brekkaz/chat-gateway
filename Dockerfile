FROM rust:1.76.0 AS builder
RUN apt update && apt install -y cmake capnproto libsasl2-dev protobuf-compiler libprotobuf-dev
WORKDIR /usr/src/app

COPY . .
RUN cargo build --release

FROM rust:1.76.0
RUN apt update && apt install -y cmake capnproto libsasl2-dev protobuf-compiler libprotobuf-dev
WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/infrastructure .
CMD ["./infrastructure"]

#CMD ["tail", "-f", "/dev/null"]