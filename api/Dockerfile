FROM rust:latest

RUN cargo install cargo-watch

# todo alex: switch to alpine image & install build deps

WORKDIR /app

# copy deps info first
COPY Cargo.lock .
COPY Cargo.toml .
RUN mkdir src

# add fake main rs to get cargo to build
RUN echo "fn main(){println!(\"dummy\");}" > src/main.rs

# get deps
RUN cargo build
ADD . .
RUN touch src/main.rs
RUN cargo build

ENTRYPOINT [ "cargo" ]
CMD ["watch", "-w", "src", "-x", "run"]