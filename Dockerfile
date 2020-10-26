FROM rust:1.47.0-buster as builder

RUN git clone https://github.com/No9/tide-morth-example.git app-build

WORKDIR "/app-build"

RUN cargo build --release \
 && echo "#!/bin/bash" > run.sh \
 && bin=$(find ./target/release -maxdepth 1 -perm -111 -type f| head -n 1) \
 && echo ./${bin##*/} >> run.sh \
 && chmod 755 run.sh

FROM debian:buster-slim

RUN useradd rust

WORKDIR "/app"

# get files and built binary from previous image
COPY --from=builder /app-build/run.sh /app-build/Cargo.toml /app-build/bin/target/release/ ./

USER rust

ENV PORT 8000

EXPOSE 8000

CMD ["./run.sh"]