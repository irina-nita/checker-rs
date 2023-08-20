FROM rust:1.67-slim

EXPOSE 4000

WORKDIR /usr/src/mw
COPY . .

ENV PORT=4000 
ENV HOST="0.0.0.0"
ENV SANDBOX_CONFIG="sandbox.json"

RUN cargo install --path .

CMD ["mw"]

