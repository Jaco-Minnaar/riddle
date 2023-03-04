FROM rust:1.67 as builder
WORKDIR /usr/src/riddle
COPY . .
RUN cargo install --path ./riddle_bin/

FROM debian:bullseye-slim
# RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/riddle_bin /usr/local/bin/riddle_bin
CMD ["riddle_bin"]
