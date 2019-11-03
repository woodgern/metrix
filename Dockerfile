FROM nwoodger/rust-rocket

COPY . /home/rocket

WORKDIR /home/rocket/metrix

USER root

RUN cargo build

EXPOSE 8000
