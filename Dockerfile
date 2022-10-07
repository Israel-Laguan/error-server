## Adapted from https://dev.to/karanpratapsingh/seeding-postgres-with-docker-19n7
FROM postgres:14-alpine as db
WORKDIR /app
COPY ./db_init.sh /docker-entrypoint-initdb.d
COPY ./schema.sql ./scripts/db/dump.sql

####################################################################################################
## Builder
####################################################################################################
FROM rust:1-slim AS builder

RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev libpq-dev wget
RUN update-ca-certificates

# Install sccache to greatly speedup builds in the CI
RUN wget https://github.com/mozilla/sccache/releases/download/v0.2.15/sccache-v0.2.15-x86_64-unknown-linux-musl.tar.gz \
    && tar xzf sccache-v0.2.15-x86_64-unknown-linux-musl.tar.gz \
    && mv sccache-v0.2.15-x86_64-unknown-linux-musl/sccache /usr/local/bin/sccache \
    && chmod +x /usr/local/bin/sccache


ENV RUSTC_WRAPPER=/usr/local/bin/sccache
ENV SCCACHE_CACHE_SIZE=2G

# Create appuser
ENV USER=error-server
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /error-server

COPY ./ .

RUN cargo build --target x86_64-unknown-linux-musl --release

RUN strip -s /error-server/target/release/error-server

####################################################################################################
## Final image
####################################################################################################
FROM scratch

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /error-server

# Copy our build
COPY --from=builder /error-server/target/x86_64-unknown-linux-musl/release/error-server ./

# Use an unprivileged user.
USER error-server:error-server

CMD ["/error-server/error-server"]
