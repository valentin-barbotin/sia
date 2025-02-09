FROM rust:1.84 as builder

ARG WATCH=""
RUN if [ -n "$WATCH" ]; then cargo install cargo-watch; fi;

RUN cargo new --bin app
WORKDIR /app

ARG PROFILE="dev"

RUN apt-get update && apt-get install protobuf-compiler -y

COPY Cargo.* .

RUN cargo build --profile $PROFILE && rm src/*.rs

RUN ln -s debug target/dev

COPY ./src ./src
COPY ./proto ./proto
COPY ./build.rs ./build.rs
COPY ./tests ./tests

RUN cargo build --profile $PROFILE

FROM debian:bullseye-slim as final

WORKDIR /app

ARG PROFILE="dev"
COPY --from=builder /app/target/$PROFILE/main .

RUN useradd -M -s /bin/bash -u 1001 svc

RUN chown -R svc:svc /app

USER svc

CMD [ "./main" ]

