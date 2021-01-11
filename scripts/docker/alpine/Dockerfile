FROM alpine:edge AS builder

# show backtraces
ENV RUST_BACKTRACE 1

RUN apk add --no-cache \
  build-base \
  cargo \
  cmake \
  eudev-dev \
  linux-headers \
  perl \
  rust

WORKDIR /vast
COPY . /vast
RUN cargo build --release --target x86_64-alpine-linux-musl --verbose
RUN strip target/x86_64-alpine-linux-musl/release/vast


FROM alpine:edge

# show backtraces
ENV RUST_BACKTRACE 1

RUN apk add --no-cache \
  libstdc++ \
  eudev-libs \
  libgcc

RUN addgroup -g 1000 vast \
  && adduser -u 1000 -G vast -s /bin/sh -D vast

USER vast

EXPOSE 8080 8545 8180

WORKDIR /home/vast

RUN mkdir -p /home/vast/.local/share/io.vast.vast/
COPY --chown=vast:vast --from=builder /vast/target/x86_64-alpine-linux-musl/release/vast ./

ENTRYPOINT ["./vast"]
