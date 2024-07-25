FROM rust:1.80-slim-bookworm
WORKDIR /usr/app/
RUN rustup component add clippy
COPY ./ ./

ENTRYPOINT ["bash", "-c"]
CMD ["bash"]
