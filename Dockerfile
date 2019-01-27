FROM rust:latest

WORKDIR /usr/src/app
COPY . .

RUN cargo build --release

FROM alpine:latest

COPY --from=0 /usr/src/app/target/release/envpopulate /usr/src/app

CMD ["/usr/src/app/envpopulate"]
