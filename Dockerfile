FROM rust as builder

# Set the working directory inside the container
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:buster-slim

ENV TINI_VERSION=v0.19.0
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    ca-certificates \
    curl \
    && rm -rf /var/lib/apt/lists/*
RUN curl -sSL -o /usr/local/bin/tini "https://github.com/krallin/tini/releases/download/${TINI_VERSION}/tini-amd64" && \
    chmod +x /usr/local/bin/tini

COPY --from=builder /usr/src/app/target/release/notion_calendar /usr/local/bin/notion_calendar
ENTRYPOINT ["/sbin/tini", "--"]
CMD ["/usr/local/bin/notion_calendar"]
