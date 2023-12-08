FROM rust:latest

WORKDIR /usr/src/auth
ENV CURRENT_DIR /usr/src/auth

COPY . .

RUN apt-get update && apt-get install -y openssl

RUN . $CURRENT_DIR/.env && \
    openssl ecparam -genkey -noout -name prime256v1 | \ 
    openssl pkcs8 -topk8 -nocrypt -out $JWT_PRIVATE_PEM_PATH && \
    openssl ec -in $JWT_PRIVATE_PEM_PATH -pubout -out $JWT_PUBLIC_PEM_PATH

ENV SQLX_OFFLINE true
RUN cargo build --release 

EXPOSE 8080
CMD ["./target/release/web_api"]
