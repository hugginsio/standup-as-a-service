# BUILD
FROM rust:1.83.0-alpine AS build

COPY . /app
WORKDIR /app

RUN apk add --no-cache musl-dev
RUN cargo build --release

# DEPLOY
FROM alpine:3.21.0 AS deploy

RUN apk add --no-cache catatonit
COPY --from=build /app/target/release/feature-flags /app/feature-flags

USER nobody:nogroup
ENTRYPOINT ["/usr/bin/catatonit", "--", "/app/feature-flags"]
CMD ["/bin/sh"]
