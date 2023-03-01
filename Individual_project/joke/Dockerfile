FROM rust:latest as builder
ENV APP joke
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update  && apt-get install -y libcurl4 && apt-get install -y libssl-dev --fix-missing && rm -rf /var/lib/apt/lists/*
RUN ldconfig
COPY --from=builder /usr/local/cargo/bin/$APP /usr/local/bin/$APP
EXPOSE 8080
CMD ["joke"]