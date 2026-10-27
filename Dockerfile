# VALKYRIE by Alyx Shang.
# Licensed under the FSL v1.

FROM messense/rust-musl-cross:x86_64-musl AS builder
ENV SQLX_OFFLINE=true
WORKDIR /valkyrie
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl
FROM scratch
COPY --from=builder /valkyrie/target/x86_64-unknown-linux-musl/release/valkyrie /valkyrie
ENTRYPOINT ["/valkyrie"]
EXPOSE 3000
