FROM rust:1.82 as builder

WORKDIR /usr/src/app
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release

FROM rust:1.82-slim
COPY --from=builder /usr/src/app/target/release/k8s-api /usr/local/bin/k8s-api
CMD ["k8s-api"]