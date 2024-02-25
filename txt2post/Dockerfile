
FROM rust:latest
# WORKDIR /src
COPY . .
RUN cargo build --release
CMD ["./target/release/txt2post"]

#FROM docker/whalesay:latest
#LABEL Name=obssubtitlesserver Version=0.0.1
#RUN apt-get -y update && apt-get install -y fortunes
#CMD ["sh", "-c", "/usr/games/fortune -a | cowsay"]
