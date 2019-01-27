FROM rust:latest

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM alpine:latest

RUN apk add libc6-compat libgcc

COPY --from=0 /usr/src/app/target/release/envpopulate /usr/src/app/envpopulate

CMD ["/usr/src/app/envpopulate"]
