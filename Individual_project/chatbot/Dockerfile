FROM rust:latest as builder
ENV APP chatbot
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path .
RUN apt-get update  && apt-get install -y libcurl4 && apt-get install -y libssl-dev --fix-missing && rm -rf /var/lib/apt/lists/*
ENV OPENAI_API_KEY = ""

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/$APP /usr/local/bin/$APP
CMD ["chatbot"]