# まず backend (rust axum) のビルドを行う
FROM rust:1.80 as backend_builder

WORKDIR /app

COPY backend .
RUN cargo build --release \
    && mv ./target/release/kani-life /app/kani-life-backend

# frontend (vite react) のビルドを行う
FROM node:20 as frontend_builder

WORKDIR /app

COPY frontend .
RUN npm install \
    && npm run build \
    && mv ./dist /app/kani-life-frontend

# 本番環境用のイメージを作成する
# scratch だとなぜか /app/kani-life-backend がないと言われるので ubuntu にしている
# rust:1.80-slim とかだとイメージサイズが 1GB を超えるが、 ubuntu だと 100MB くらい
FROM ubuntu
WORKDIR /app
# backend binary
COPY --from=backend_builder /app/kani-life-backend /app/kani-life-backend
# frontend static files
COPY --from=frontend_builder /app/kani-life-frontend /app/static
CMD ["/app/kani-life-backend"]
