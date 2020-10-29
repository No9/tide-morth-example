# Docker file to build an image with just the executable in.
FROM rust:1.47.0-alpine as builder

COPY . /app-build

WORKDIR "/app-build"

RUN \
  apk add --no-cache musl-dev && \
  cargo build --release \
 && echo "#!/bin/bash" > run.sh \
 && bin=$(find ./target/release -maxdepth 1 -perm -111 -type f| head -n 1) \
 && echo ./${bin##*/} >> run.sh \
 && chmod 755 run.sh

FROM alpine

RUN useradd rust

WORKDIR "/app"

# get files and built binary from previous image
COPY --from=builder --chown=rust /app-build/run.sh /app-build/Cargo.toml /app-build/target/release/ ./

USER rust

ENV PORT 8000

EXPOSE 8000

CMD ["./run.sh"]