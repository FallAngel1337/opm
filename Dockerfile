FROM rust:1.57

WORKDIR /opt/opm
COPY . .

RUN cargo install --path .