FROM rust:1.31

WORKDIR /usr/src/amb_back
COPY . .
RUN cargo install --path .
CMD [amb_back]