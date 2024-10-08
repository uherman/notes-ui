FROM rust:1-slim-buster as builder
WORKDIR /app
COPY . .
RUN cargo install --path .

FROM debian:buster-slim as runner
COPY --from=builder /usr/local/cargo/bin/notes-md-api /usr/local/bin/notes-md-api
EXPOSE 5000
CMD ["notes-md-api"]