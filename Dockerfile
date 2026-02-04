ARG version=1.90
FROM rust:${version}
ARG version

RUN dpkg --add-architecture arm64
RUN apt-get update && apt-get install -y \
    cmake \
    libclang-dev \
    libxkbcommon-dev \
    libxkbcommon-dev:arm64 \
    libssl-dev \
    libssl-dev:arm64 \
    libudev-dev \
    libudev-dev:arm64 \
    libpipewire-0.3-dev \
    libpipewire-0.3-dev:arm64 \
    libinput-dev \
    libinput-dev:arm64

RUN apt-get install -y \
    g++-aarch64-linux-gnu \
    libc6-dev-arm64-cross

# Taskfile support
RUN curl -1sLf 'https://dl.cloudsmith.io/public/task/task/setup.deb.sh' | bash
RUN apt-get install -y task

# RPM support
RUN apt-get install -y rpm librpmbuild10 elfutils

RUN rustup toolchain install 1.90.0 --profile minimal --component clippy,rustfmt
RUN rustup target add aarch64-unknown-linux-gnu --toolchain 1.90.0
RUN chmod -R 777 /usr/local/rustup

ENV CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc
ENV CC_aarch64_unknown_linux_gnu=aarch64-linux-gnu-gcc
ENV CXX_aarch64_unknown_linux_gnu=aarch64-linux-gnu-g++

# Pipewire/spa headers for bindgen and cc-rs
ENV BINDGEN_EXTRA_CLANG_ARGS="-I/usr/include/pipewire-0.3 -I/usr/include/spa-0.2"
ENV C_INCLUDE_PATH="/usr/include/pipewire-0.3:/usr/include/spa-0.2"