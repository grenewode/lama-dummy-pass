FROM rust:alpine as builder

WORKDIR /usr/src/imposter-pass
COPY . .

RUN apk add --no-cache musl-dev

RUN cargo install --path .

# Now that we've built the binary, we can use it however we want!
FROM alpine:latest

COPY --from=builder /usr/local/cargo/bin/imposter-pass /usr/local/bin/imposter-pass

CMD ["imposter-pass"]
