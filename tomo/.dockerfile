FROM rust:latest

WORKDIR .
COPY . .

CMD ["wasm-pack build --release"]
