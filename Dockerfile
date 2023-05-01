FROM ubuntu:20.04

WORKDIR gid
COPY . .

RUN apt update \
    && apt install -y build-essential curl time git \
    && curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

ENV PATH="/root/.cargo/bin:${PATH}"

CMD [ "/bin/sh" ]
