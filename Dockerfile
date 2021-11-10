FROM node as frontend-builder

WORKDIR /app

COPY ["frontend/package.json", "frontend/package-lock.json*", "./"]

RUN npm install

COPY frontend .

RUN npm run build

FROM rust:latest as backend-builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=backend-builder /app/target/release/electer /
COPY --from=frontend-builder /app/dist /static
ENV DIST_DIR="/static"
CMD ["./electer"]

