FROM --platform=$BUILDPLATFORM rust:bullseye as build
ARG TARGETPLATFORM
ARG BUILDPLATFORM
COPY . /app
WORKDIR /app
RUN apt update && apt full-upgrade -y
RUN apt install protobuf-compiler -y
RUN echo "I am running on $BUILDPLATFORM, building for $TARGETPLATFORM" 
RUN chmod +x /app/build.sh
RUN /app/build.sh

FROM --platform=$TARGETPLATFORM debian:11
ARG TARGETPLATFORM
ARG BUILDPLATFORM
RUN apt update && apt full-upgrade -y
COPY --from=build /app/arkalis /app/arkalis
COPY --from=build /app/arch.txt /app/arch.txt
WORKDIR /app
RUN cat arch.txt
RUN chmod +x /app/arkalis
ENTRYPOINT [ "./arkalis" ]