FROM rust:1.81-slim-bookworm
WORKDIR /usr/app/
RUN rustup component add clippy
RUN cargo install cargo-clean-all
COPY ./ ./

ENTRYPOINT ["bash", "-c"]
CMD ["bash"]
