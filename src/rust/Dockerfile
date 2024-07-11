FROM rust:1.79-slim-bookworm
WORKDIR /usr/app/
RUN rustup component add clippy
COPY ./ ./
ENTRYPOINT ["bash", "-c"]
CMD ["bash"]
