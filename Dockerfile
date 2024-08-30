FROM alt:p10

WORKDIR /usr/src/

RUN apt-get update && apt-get install -y curl

COPY Cargo.toml Cargo.toml
COPY src src
COPY .env .env

CMD ["/bin/bash"]