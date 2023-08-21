FROM rust:1.67-slim

SHELL ["/bin/bash", "-O", "extglob", "-c"]

EXPOSE 3000

RUN apt update -y && \
    apt-get install pkg-config -y && \
    apt-get install libssl-dev -y

# Create sandbox dirs.
RUN mkdir -p /restricted/home/sandbox && \
    mkdir -p /restricted/home/service/app && \
    mkdir -p /restricted/home/sandbox/tests && \
    mkdir -p /restricted/home/sandbox/tests/in && \
    mkdir -p /restricted/home/sandbox/tests/ref && \
    mkdir -p /restricted/home/sandbox/src && \
    mkdir -p /restricted/home/sandbox/tests/out && \
    touch /restricted/home/sandbox/config.json

# Create new users.
RUN groupadd -r restricted && \
    useradd -r -g restricted sandbox && \
    useradd -r -g restricted service

# TODO: Restrict access to almost all

RUN cargo install acadchecker

WORKDIR /restricted/home/service/core

COPY . .

ENV HOST="0.0.0.0"
ENV PORT=3000

ENV __PROBLEM_INS="/restricted/home/sandbox/tests/in"
ENV __PROBLEM_REFS="/restricted/home/sandbox/tests/ref"
ENV __PROBLEM_OUT="/restricted/home/sandbox/tests/out"
ENV __PROBLEM_SRC="/restricted/home/sandbox/src"
ENV __PROBLEM_CFG="/restricted/home/sandbox/config.json"

RUN cargo install --path .

CMD ["core"]
