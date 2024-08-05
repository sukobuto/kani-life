# まず backend (rust axum) のビルドを行う
FROM --platform=$BUILDPLATFORM messense/rust-musl-cross:x86_64-musl as backend_builder

WORKDIR /app

COPY backend .
RUN cargo build --release \
    && mv ./target/x86_64-unknown-linux-musl/release/kani-life /app/kani-life-backend

# frontend (vite react) のビルドを行う
FROM node:20 as frontend_builder

WORKDIR /app

COPY frontend .
RUN npm install \
    && npm run build \
    && mv ./dist /app/kani-life-frontend

# 本番環境用のイメージを作成する
FROM scratch
WORKDIR /app
# backend binary
COPY --from=backend_builder /app/kani-life-backend /app/kani-life-backend
# frontend static files
COPY --from=frontend_builder /app/kani-life-frontend /app/static
CMD ["/app/kani-life-backend"]
