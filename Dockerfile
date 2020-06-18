FROM ekidd/rust-musl-builder AS build

RUN USER=rust cargo init .
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

# Copy the source and build the application.
COPY src ./src
# Avoid main.rs cache
RUN rm -rf target/x86_64-unknown-linux-musl/release/deps/reports-*
# Build app deps
RUN cargo build --release

# Copy the statically-linked binary into a scratch container.
FROM alpine:latest
COPY ./src/templates/*.hbs ./src/templates/
COPY --from=build /home/rust/src/target/x86_64-unknown-linux-musl/release/reports .
USER 1000
CMD ["./reports"]