FROM nwoodger/rust-rocket

COPY . /home/rocket

USER root

EXPOSE 8000

