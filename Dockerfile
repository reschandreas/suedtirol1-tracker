#https://github.com/bjornmolin/rust-minimal-docker
FROM clux/muslrust:stable as build-stage

RUN groupadd -g 10001 -r dockergrp && useradd -r -g dockergrp -u 10001 dockeruser

# Build the project with target x86_64-unknown-linux-musl

# Build dummy main with the project's Cargo lock and toml
# This is a docker trick in order to avoid downloading and building 
# dependencies when lock and toml not is modified.
COPY Cargo.lock .
COPY Cargo.toml .

RUN mkdir src && \
    echo "fn main() {print!(\"Dummy main\");} // dummy file" > src/main.rs

RUN set -x && cargo build --target x86_64-unknown-linux-musl --release

RUN set -x && rm target/x86_64-unknown-linux-musl/release/deps/suedtirol*
RUN touch /var/tmp/suedtirol1 && chmod 777 /var/tmp/suedtirol1

# Now add the rest of the project and build the real main
COPY src ./src
RUN set -x && cargo build --target x86_64-unknown-linux-musl --release
RUN mkdir -p /build-out
RUN set -x && cp target/x86_64-unknown-linux-musl/release/suedtirol1-tracker /build-out/

# Create a minimal docker image 
FROM scratch as runtime-stage

COPY --from=builder /etc/passwd /etc/passwd
USER dockeruser

ENV RUST_LOG="error,suedtirol1-tracker=info"
COPY --from=build-stage /build-out/suedtirol1-tracker /
COPY --from=build-stage /var/tmp/suedtirol1 /var/tmp/suedtirol1
CMD ["/suedtirol1-tracker"]
