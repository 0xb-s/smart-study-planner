FROM lukemathwalker/cargo-chef:latest-rust-1 as builder


ENV CARGO_INCREMENTAL=0 \
    CARGO_TERM_COLOR=always \
    RUSTFLAGS="-C target-feature=+crt-static"

RUN apt-get update && apt-get -y upgrade && apt-get install -y libclang-dev pkg-config


RUN USER=root cargo new --bin smart-study-planner-backend
WORKDIR /smart-study-planner-backend


COPY Cargo.toml Cargo.lock ./


RUN cargo build --release
RUN rm src/*.rs


COPY src ./src


RUN cargo build --release


FROM gcr.io/distroless/cc-debian10


COPY --from=builder /smart-study-planner-backend/target/release/smart-study-planner-backend /usr/local/bin/smart-study-planner-backend


EXPOSE 3000


CMD ["smart-study-planner-backend"]
