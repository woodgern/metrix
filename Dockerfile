FROM nwoodger/rust-rocket

COPY . /home/rocket

WORKDIR /home/rocket

USER root

RUN cargo build

EXPOSE 8000
