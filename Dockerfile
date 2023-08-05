# generate rust docker file
FROM rust:1.71.0 as builder

# set working directory
WORKDIR /app

# install dependencies
RUN apt-get update && apt-get install -y \
    clang \
    libclang-dev

RUN cargo install diesel_cli --no-default-features --features postgres

# copy project
COPY . /app

# run server
RUN cargo build --release


FROM debian:buster-slim

WORKDIR /app

COPY --from=builder /app/target/release/llama_internet_chatbot /app/llama_internet_chatbot
COPY --from=builder /app/migrations /app/migrations
COPY --from=builder /app/.env /app/.env
COPY --from=builder /app/Rocket.toml /app/Rocket.toml
COPY --from=builder /usr/local/cargo/bin/diesel /app/diesel

# install dependencies
RUN apt-get update && apt-get install -y \
    libpq-dev \
    libssl-dev \
    curl \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 8080

CMD ./diesel migration run && ./llama_internet_chatbot

