FROM rust:1.67-slim

RUN apt update -y && \
    apt-get install pkg-config -y && \
    apt-get install libssl-dev -y

EXPOSE 4000

WORKDIR /usr/src/mw

COPY . .

RUN cargo install --path .

CMD ["mw"]

