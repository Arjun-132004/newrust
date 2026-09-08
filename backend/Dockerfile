# syntax=docker/dockerfile:1
# --- Build stage ---
FROM rust:1-bookworm AS builder
WORKDIR /src
ENV SQLX_OFFLINE=true
COPY . .
RUN cargo build --release
RUN set -eu; \
    mkdir -p /out; \
    want='backend'; \
    if [ -n "$want" ] && [ -f "target/release/$want" ]; then \
      cp "target/release/$want" /out/app; \
    else \
      set -- $(find target/release -maxdepth 1 -type f -executable \
                 ! -name '*.d' ! -name '*.so' ! -name '*.rlib' ! -name '*.a' 2>/dev/null); \
      if [ "$#" -eq 0 ]; then \
        echo "cargo produced no executable in target/release." >&2; \
        echo "A library-only crate has nothing to run — add src/main.rs or a [[bin]] target." >&2; \
        exit 1; \
      fi; \
      if [ "$#" -gt 1 ]; then \
        echo "cargo produced more than one executable: $*" >&2; \
        echo "Name the one this image should run with a single [[bin]] in Cargo.toml." >&2; \
        exit 1; \
      fi; \
      cp "$1" /out/app; \
    fi; \
    chmod +x /out/app

# --- Runtime ---
FROM gcr.io/distroless/cc-debian12:nonroot
WORKDIR /app
COPY --from=builder /out/app /app/app
# Read from disk at run time — see the note in rustDockerfile.
COPY --from=builder /src/migrations /app/migrations
USER 65532
ENV PORT=8080
EXPOSE 8080
ENTRYPOINT ["/app/app"]
