FROM docker.io/rust:bookworm
MAINTAINER Xuejie Xiao <xxuejie@gmail.com>

RUN rustup target add riscv64imac-unknown-none-elf

RUN apt-get update && apt install -y lsb-release wget software-properties-common gnupg
RUN wget https://apt.llvm.org/llvm.sh && chmod +x llvm.sh && ./llvm.sh 16 && rm llvm.sh

RUN set -eux; \
  rustup --version; \
  cargo --version; \
  rustc --version; \
  clang-16 --version
