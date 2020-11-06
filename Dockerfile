# Docker file to build an image with just the executable in.
FROM rust:1.47.0-buster as builder

COPY . /app-build

WORKDIR "/app-build"

RUN \
  cargo build --release \
 && echo "#!/bin/bash" > run.sh \
 && bin=$(find ./target/release -maxdepth 1 -perm -111 -type f| head -n 1) \
 && echo ./${bin##*/} >> run.sh \
 && chmod 755 run.sh

FROM debian:buster-slim

RUN useradd rust

WORKDIR "/app"

# get files and built binary from previous image
COPY --from=builder --chown=rust /app-build/run.sh /app-build/public/ /app-build/Cargo.toml /app-build/target/release/tide-morth-example ./

WORKDIR "/app/public"

COPY --from=builder --chown=rust /app-build/run.sh /app-build/public/  ./

WORKDIR "/app"

USER rust

ENV PORT 8080

EXPOSE 8080

CMD ["./run.sh"]