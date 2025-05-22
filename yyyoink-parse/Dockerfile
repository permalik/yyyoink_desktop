# Nix builder
FROM nixos/nix:latest AS builder

# Copy source; Init working directory
COPY . /tmp/build
WORKDIR /tmp/build

# Build Nix environment
RUN nix \
        --extra-experimental-features "nix-command flakes" \
        --option filter-syscalls false \
        build

# Copy Nix store closure into directory
RUN mkdir /tmp/nix-store-closure
RUN cp -R $(nix-store -qR result/) /tmp/nix-store-closure

# Final image based on scratch
FROM scratch

WORKDIR /bin

# Copy /nix/store
COPY --from=builder /tmp/nix-store-closure /nix/store
COPY --from=builder /tmp/build/result /yyyoink-parse
CMD ["/yyyoink-parse/bin/yyyoink-parse"]
