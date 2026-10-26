FROM messense/rust-musl-cross:x86_64-musl as builder
ENV SQLX_OFFLINE=true
WORKDIR /valkyrie
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl
FROM scratch
COPY --from=builder
/valkyrie/target/x86_64-unknown-linux-musl/release/valkyire /valkyrie
ENTRYPOINT ["/valkyrie"]
EXPOSE 3000
