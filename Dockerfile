# Compilation Layer
FROM rust:latest AS build
# Create a fresh project
RUN USER=root cargo new --bin web-service
WORKDIR /web-service
# Copy working configs
COPY ./migrations ./migrations
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./.sqlx ./.sqlx
# We need to make sure this is set to prevent SQLx from attempting to connect to a live database
# Instead, this forces it to rely on the migrations in the .sqlx directory
ARG SQLX_OFFLINE=true
# Build the dependencies for caching then nuke the original dependencies
RUN cargo build --release
RUN rm ./src/*.rs
RUN rm ./target/release/deps/web_service*
# Copy in the original application source files
COPY ./src ./src
# Run a release build for the app
RUN cargo build --release && rm -r src/

# Deployment Layer
FROM debian:trixie
# Expose the apps web server port
EXPOSE 3000
# Copy the release executable to the root, copy in the docker environment
COPY --from=build /web-service/target/release/web-service .
COPY ./.env.docker ./.env
# Start the app
CMD ["./web-service"]