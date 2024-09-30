FROM rust:1.81-bookworm
WORKDIR /usr/app/

# pyo3
RUN apt-get update && \
    apt-get install -yq \
        python3 \
        python3-pip \
        python3-dev \
        openssh-server \
    --no-install-recommends && \
    apt-get clean
RUN pip3 install maturin --no-cache-dir --break-system-packages

# rust
RUN rustup component add clippy rustfmt
RUN cargo install cargo-clean-all

# zed remote
# including https://kinsta.com/blog/ssh-into-docker-container/
# ENV SHELL=bash
# RUN curl -f https://zed.dev/install.sh | ZED_CHANNEL=preview sh
# RUN echo 'root:root' | chpasswd
# RUN mkdir /var/run/sshd
# RUN sed -i 's/#PermitRootLogin prohibit-password/PermitRootLogin yes/' /etc/ssh/sshd_config
# CMD ["/usr/sbin/sshd", "-D"]

COPY ./ ./
