FROM clux/muslrust:1.83.0-stable AS chef
WORKDIR /app
RUN cargo install cargo-chef

FROM chef AS prepare
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM prepare AS cook
COPY --from=prepare /app/recipe.json recipe.json
# x86_64-unknown-linux-musl
RUN cargo chef cook --release --target aarch64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target aarch64-unknown-linux-musl

FROM alpine:3.21.0 AS runtime
RUN apk add --no-cache catatonit
COPY --from=cook /app/target/aarch64-unknown-linux-musl/release/standup-api /usr/local/bin/standup-api
USER nobody:nogroup
ENTRYPOINT ["/usr/bin/catatonit", "--", "/usr/local/bin/standup-api"]
