FROM rust:1.81-slim-bookworm
WORKDIR /usr/app/
RUN rustup component add clippy rustfmt
RUN cargo install cargo-clean-all
RUN apt-get update && \
    apt-get install -yq wget && \
    apt-get clean 

COPY ./ ./

ENTRYPOINT ["bash", "-c"]
CMD ["bash"]
