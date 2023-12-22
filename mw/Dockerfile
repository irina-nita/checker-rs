FROM rust:1.67-slim

RUN apt update -y && \
    apt-get install pkg-config -y && \
    apt-get install libssl-dev -y

EXPOSE 4000

WORKDIR /usr/src/mw

COPY . .

RUN rustup default stable 

RUN rustup update 

RUN cargo install --path .

ENV BUCKET_NAME="acadnet"

CMD ["mw"]

