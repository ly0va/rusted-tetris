FROM rust:1.69

WORKDIR /tetris
COPY . .

RUN cargo install --path .

ENTRYPOINT ["rusted-tetris"]
