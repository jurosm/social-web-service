FROM rust:1.78.0

RUN apt update
RUN apt install -y libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /usr/src/social-web-service

COPY . .

RUN cargo install --path .

EXPOSE 8000

CMD bash -c "diesel migration run && social_web_service"