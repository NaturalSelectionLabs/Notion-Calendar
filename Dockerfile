FROM rust:alpine as builder

# Set the working directory inside the container
WORKDIR /usr/src/app
RUN apk --no-cache add  \
    openssl-dev \
    musl-dev \
    musl-tools
COPY . .
RUN cargo build --release

FROM alpine

RUN apk --no-cache add ca-certificates openssl tini

COPY --from=builder /usr/src/app/target/release/notion_calendar /usr/local/bin/notion_calendar
ENTRYPOINT ["/sbin/tini", "--"]
CMD ["/usr/local/bin/notion_calendar"]
