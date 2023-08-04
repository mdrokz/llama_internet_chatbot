# generate rust docker file
FROM rust:1.71.0

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

# CMD ["diesel migration run && ./target/release/llama_internet_chatbot"]
CMD diesel migration run && ./target/release/llama_internet_chatbot

# EXPOSE 8080
