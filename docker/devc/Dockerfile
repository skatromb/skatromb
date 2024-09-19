FROM python:3.12
ENTRYPOINT ["bash", "-c"]

# `tflint`, needs to be installed by superuser
RUN curl --silent https://raw.githubusercontent.com/terraform-linters/tflint/master/install_linux.sh | bash

# Set user
ARG USER_NAME=app
RUN useradd --home-dir /${USER_NAME} ${USER_NAME} --groups sudo
USER app
WORKDIR /${USER_NAME}

# `gcloud`
RUN curl https://sdk.cloud.google.com | bash > /dev/null
ENV PATH=${PATH}:/${USER_NAME}/google-cloud-sdk/bin
RUN echo "source ~/google-cloud-sdk/completion.bash.inc" >> .bashrc

# `kubectl`
RUN gcloud components install kubectl

# `awscli`
ARG AWS_CLI_DIR=/${USER_NAME}/aws-cli
RUN curl "https://awscli.amazonaws.com/awscli-exe-linux-aarch64.zip"  \
    --output "awscliv2.zip" && \
    unzip -q awscliv2.zip && \
    ./aws/install --bin-dir ${AWS_CLI_DIR}/bin --install-dir ${AWS_CLI_DIR} && \
    rm -rf awscliv2.zip aws
ENV PATH=${PATH}:${AWS_CLI_DIR}/bin
RUN echo "complete -C aws_completer aws" >> .bashrc

# `chamber`
ARG CHAMBER_VERSION=v2.12.0
RUN mkdir chamber && \
    curl --location https://github.com/segmentio/chamber/releases/download/${CHAMBER_VERSION}/chamber-${CHAMBER_VERSION}-linux-arm64 \
         --output ~/chamber/chamber && \
    chmod +x ~/chamber/chamber
ENV PATH=${PATH}:/${USER_NAME}/chamber

# dive to `code` folder
WORKDIR /app/code
