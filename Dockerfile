#FROM ekidd/rust-musl-builder:nightly-2021-12-23 as builder
# The version below updates every day. Ideally we'd want to tag
# a version frozen in time, like above, but currently all other available
# nightly versions are too old to compile this project.
FROM rustlang/rust:nightly-alpine as builder

RUN apk add build-base # Parent image doesn't contain dependencies necessary for linking to musl libc for some reason

RUN USER=root cargo new --lib wasmthello
WORKDIR ./wasmthello
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN USER=root cargo new --bin web
COPY ./web/Cargo.lock ./web/Cargo.lock
COPY ./web/Cargo.toml ./web/Cargo.toml

WORKDIR ./web

# Why build, remove, then build again?
# It's a docker image build time optimisation only.
# This first build we have only added the list of dependencies
# but not our source code. So it will compile all the deps,
# and then when we change the source code it doesn't need to recompile
# them when building the image, because no files copied so far have changed.
RUN cargo build --release

# Removes both wasmthello* and wasmthello_web* so they are recompiled
RUN rm ./target/release/deps/wasmthello*
RUN rm -r ./target/release/.fingerprint/wasmthello*

WORKDIR ..
ADD . ./
WORKDIR ./web

# Now we've added the source code and the real binary is built
RUN cargo build --release

# New container, containing only the binary (no dev dependencies)

FROM alpine:latest

ARG APP=/usr/src/app

EXPOSE 3000

ENV APP_USER=appuser

RUN addgroup -S $APP_USER \
    && adduser -S -g $APP_USER $APP_USER

COPY --from=builder /wasmthello/web/target/release/wasmthello-web ${APP}/web

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./web"]
