# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build

WORKDIR /usr/src/suedtirol1-tracker

COPY Cargo.toml Cargo.toml

RUN mkdir src/

RUN echo "fn main() {println!(\"if you see this, the build broke\")}" > src/main.rs

RUN cargo build --release

RUN rm -f target/release/deps/suedtirol1-tracker*

COPY . .

RUN cargo build --release

RUN cargo install --path .

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------

FROM scratch

COPY --from=cargo-build /usr/local/cargo/bin/suedtirol1-tracker /usr/local/bin/suedtirol1-tracker

CMD ["suedtirol1-tracker"]
