# Multi-stage build for tui-framework with notcurses
FROM ubuntu:25.10 AS builder

# Install dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    cmake \
    pkg-config \
    libncurses-dev \
    libunistring-dev \
    libavformat-dev \
    libavutil-dev \
    libswscale-dev \
    libqrcodegen-dev \
    git \
    curl \
    && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Build notcurses 3.0.11
WORKDIR /tmp
RUN git clone https://github.com/dankamongmen/notcurses.git && \
    cd notcurses && \
    git checkout v3.0.11 && \
    mkdir build && \
    cd build && \
    cmake .. -DCMAKE_BUILD_TYPE=Release && \
    make -j$(nproc) && \
    make install && \
    ldconfig

# Runtime stage
FROM ubuntu:25.10

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libncurses6 \
    libunistring2 \
    libavformat60 \
    libavutil58 \
    libswscale7 \
    libqrcodegen1 \
    && rm -rf /var/lib/apt/lists/*

# Copy notcurses from builder
COPY --from=builder /usr/local/lib/libnotcurses* /usr/local/lib/
COPY --from=builder /usr/local/include/notcurses /usr/local/include/notcurses
COPY --from=builder /usr/local/lib/pkgconfig/notcurses*.pc /usr/local/lib/pkgconfig/
RUN ldconfig

# Install Rust in runtime
RUN apt-get update && apt-get install -y curl && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    rm -rf /var/lib/apt/lists/*
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /workspace
COPY . .

# Test the installation
RUN cargo run --example backend_test --features notcurses

CMD ["bash"]
