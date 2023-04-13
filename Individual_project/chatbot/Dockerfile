FROM rust:latest as builder
ENV APP chatbot
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path .
RUN apt-get update && apt-get install -y libcurl4 libssl-dev --fix-missing && rm -rf /var/lib/apt/lists/*
ENV OPENAI_API_KEY=""

FROM debian:buster-slim
# Install necessary dependencies in the final image
RUN apt-get update && apt-get install -y libcurl4 libssl1.1 --fix-missing && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/$APP /usr/local/bin/$APP
CMD ["chatbot"]
