FROM rust:1.81-slim-bookworm
WORKDIR /usr/app/
RUN rustup component add clippy rustfmt
RUN cargo install cargo-clean-all
RUN apt-get update && \
    apt-get install -yq wget python3 python3-pip --no-install-recommends && \
    apt-get clean
RUN pip3 install maturin --no-cache-dir --break-system-packages
COPY ./ ./

ENTRYPOINT ["bash", "-c"]
CMD ["bash"]
