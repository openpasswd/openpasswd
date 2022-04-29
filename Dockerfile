FROM rust:1.60 as build
WORKDIR /app
COPY . .
RUN cargo build --release -p openpasswd-server

FROM debian:buster-slim
RUN apt-get update && \
    apt-get upgrade -y && \
    apt-get install -y libpq5 && \
    rm -rf /var/lib/apt/lists/* && \
    useradd -ms /bin/bash openpasswd
USER openpasswd
WORKDIR /app
COPY --chown=openpasswd:openpasswd --from=build /app/target/release/openpasswd-server /app/openpasswd-server
CMD ["/app/openpasswd-server"]
