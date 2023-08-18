FROM rust:1.67-slim

SHELL ["/bin/bash", "-O", "extglob", "-c"]

RUN apt update && \
    apt-get install pkg-config && \
    apt-get install libssl-dev && \
    apt-get install setfacl

# Create sandbox dirs.
RUN mkdir -p /restricted/home/sandbox && \
    mkdir -p /restricted/home/service/app && \
    mkdir -p /restricted/home/sandbox/tests && \
    mkdir -p /restricted/home/sandbox/tests/in && \
    mkdir -p /restricted/home/sandbox/tests/ref && \
    mkdir -p /restricted/home/sandbox/src && \
    mkdir -p /restricted/home/sandbox/tests/out

# Create new users.
RUN groupadd -r restricted && \
    useradd -r -g restricted sandbox && \
    useradd -r -g restricted service

# Restrict access to almost all
RUN chmod 755 !(*sys*|*proc*) && \
    setfacl -RM sandbox:r /restricted/home/sandbox/tests/in && \
    setfacl -RM sandbox:w /restricted/home/sandbox/tests/out && \
    setfacl -RM sandbox:r /restricted/home/sandbox/tests/ref && \
    setfacl -RM sandbox:rx /restricted/home/sandbox/src && \
    chown -R service:restricted /restricted/home/service && \
    chmod 740 /restricted/home/service

RUN cargo install acadchecker

WORKDIR /restricted/home/service/app

COPY . .

RUN cargo build --release

USER service

CMD ["chroot", "/restricted", "./target/release/core"]
