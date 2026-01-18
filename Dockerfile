FROM rust:1.89-alpine AS builder


RUN apk add --no-cache build-base openssl-dev pkgconfig

RUN rustup target add wasm32-unknown-unknown

RUN cargo install trunk --locked
# permet de mettre wasm-bindgen en cache car trunk le retelecharge à chaque fois sinon
RUN cargo install wasm-bindgen-cli --version 0.2.106

WORKDIR /usr/src/app

# Copier les fichiers de dépendances
COPY Cargo.toml Cargo.lock ./

# Créer un build factice pour mettre en cache les dépendances
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    cargo build --release --target wasm32-unknown-unknown

# Nettoyer les artefacts du build factice
RUN rm -rf src target/release/deps/hangar_front* target/wasm32-unknown-unknown/release/deps/hangar_front*

COPY . .

RUN trunk build --release


FROM nginx:alpine AS runner

COPY --from=builder /usr/src/app/dist /usr/share/nginx/html

COPY nginx.conf /etc/nginx/conf.d/default.conf

EXPOSE 80

CMD ["nginx", "-g", "daemon off;"]