FROM python:latest
ENTRYPOINT bash

# environment
ARG USER_NAME=app
RUN useradd --home-dir /${USER_NAME} ${USER_NAME} --groups sudo
USER app
WORKDIR /${USER_NAME}

# `gcloud`
RUN curl -sSL https://sdk.cloud.google.com | bash
RUN echo "source google-cloud-sdk/completion.bash.inc" >> .bashrc
ENV PATH=${PATH}:/${USER_NAME}/google-cloud-sdk/bin

# `kubectl`
RUN gcloud components install kubectl

# `awscli`
RUN pip install awscliv2 --no-cache-dir
ENV PATH=${PATH}:/${USER_NAME}/.local/bin
RUN echo "source aws_bash_completer" >> .bashrc

# `chamber`
ARG CHAMBER_VERSION=v2.12.0
RUN curl --location https://github.com/segmentio/chamber/releases/download/${CHAMBER_VERSION}/chamber-${CHAMBER_VERSION}-linux-arm64 \
         --output ~/.local/bin/chamber && \
    chmod +x ~/.local/bin/chamber
