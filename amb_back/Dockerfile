FROM rust:1.47 as builder

RUN USER=root cargo new --bin amb_back
WORKDIR ./amb_back
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release \
    && rm src/*.rs target/release/deps/amb_back*

ADD . ./

RUN cargo build --release


FROM debian:buster-slim

ARG APP=/usr/src/app

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8000
EXPOSE 80

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /amb_back/target/release/amb_back ${APP}/amb_back

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./amb_back"]
