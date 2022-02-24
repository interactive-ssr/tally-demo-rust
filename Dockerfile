FROM rust:1.58.1

WORKDIR /app
COPY . .

RUN cargo build

CMD ["cargo", "run"]