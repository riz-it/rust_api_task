# Tahap 1: Build dengan Rust di atas Alpine
FROM rust:1.87-alpine AS builder

# Install dependencies build termasuk static OpenSSL
RUN apk add --no-cache \
    musl-dev \
    openssl-dev \
    openssl-libs-static \
    pkgconf \
    build-base \
    perl \
    libgcc \
    libstdc++ \
    linux-headers

# Buat direktori kerja
WORKDIR /app

# PERBAIKAN: Salin semua file proyek dari root (bukan src/.)
COPY src/. .

# Set environment variables untuk static linking
ARG DATABASE_URL
ENV DATABASE_URL=${DATABASE_URL}
ENV OPENSSL_STATIC=1
ENV OPENSSL_DIR=/usr

# Build release
RUN cargo build --release

# Tahap 2: Image minimal untuk menjalankan binary
FROM alpine:3.20

# Install runtime dependencies
RUN apk add --no-cache ca-certificates

# Buat direktori kerja
WORKDIR /app

# Salin hasil build dari tahap sebelumnya
COPY --from=builder /app/target/release/crud_axum .

COPY startScript.sh /app/startScript.sh

# Beri permission executable
RUN chmod +x /app/startScript.sh

# Ekspose port aplikasi
EXPOSE 8787

# PERBAIKAN: Konsisten dengan nama file (startScript.sh bukan startscript.sh)
CMD ["/app/startScript.sh"]