FROM nwoodger/rust-rocket

COPY . /home/rocket

USER root

RUN cargo build

EXPOSE 8000
