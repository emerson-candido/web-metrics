FROM ubuntu:24.04
ARG APP_NAME=web_metrics
ARG USERNAME=$APP_NAME
ARG USER_UID=1001
ARG USER_GID=$USER_UID
ENV APP_NAME=${APP_NAME}

RUN groupadd --gid $USER_GID $USERNAME \
    && useradd --uid $USER_UID --gid $USER_GID -m $USERNAME

COPY ./target/x86_64-unknown-linux-musl/release/$APP_NAME /usr/local/bin/

USER $USER_UID

ENTRYPOINT [ "${APP_NAME}" ]
