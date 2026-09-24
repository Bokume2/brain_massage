# syntax=docker/dockerfile:1
FROM rust:1-trixie AS builder

RUN apt-get update && apt-get install -y lsb-release wget gnupg \
  && wget https://apt.llvm.org/llvm.sh && chmod +x llvm.sh \
  && ./llvm.sh 20 all

WORKDIR /brain_massage

COPY . .

RUN cargo build --release

FROM debian:trixie-slim AS bmsgc

RUN apt-get update && apt-get install -y libffi8 gcc

ARG UID=1000
ARG GID=1000

RUN groupadd -g ${GID} bmsg \
  && useradd -u ${UID} -g bmsg -s /bin/bash bmsg

USER bmsg
WORKDIR /brain_massage

COPY --from=builder /brain_massage/target/release/bmsgc /bin/

CMD ["/bin/bash"]
