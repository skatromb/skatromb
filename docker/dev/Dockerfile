FROM python:latest
ENTRYPOINT bash

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
RUN pip install awscli --no-cache-dir
ENV PATH=${PATH}:~/.local/bin
RUN echo "source aws_bash_completer" >> .bashrc

# TODO: `chamber`
