FROM rust:1.77 as builder

# Install ProtoC
ENV PROTOC_ZIP=protoc-26.0-linux-x86_64.zip
RUN curl -OL https://github.com/protocolbuffers/protobuf/releases/download/v26.0/${PROTOC_ZIP}
RUN unzip -o ${PROTOC_ZIP} -d ./proto 
RUN chmod 755 -R ./proto/bin
ENV BASE=/usr

RUN cp ./proto/bin/protoc ${BASE}/bin/
RUN cp -R ./proto/include/* ${BASE}/include/

WORKDIR /usr/src/arkalis
COPY . .

RUN cargo install --path .


FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install openssl curl -y

COPY --from=builder /usr/local/cargo/bin/arkalis /usr/local/bin/arkalis
EXPOSE 8000
ENTRYPOINT ["arkalis"]