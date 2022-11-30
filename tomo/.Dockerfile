FROM btfs/wasm-pack

WORKDIR .
COPY . .

RUN rustup target add wasm32-unknown-unknown

CMD ["wasm-pack build --release"]
