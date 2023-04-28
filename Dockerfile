FROM rust:1.67

WORKDIR gid
COPY . .

RUN cargo install --path . \
    && apt update \
    && apt -y install time

CMD [ "/bin/sh" ]
