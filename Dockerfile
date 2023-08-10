FROM alpine as builder
COPY . /workspace/source
RUN apk update \
    && apk upgrade \
    && apk add cargo \
    && cargo install --path /workspace/source --root /workspace

FROM alpine as runner
WORKDIR /workspace
RUN apk update --no-cache \
    && apk upgrade --no-cache \
    && apk add libgcc --no-cache
COPY --from=builder /workspace/bin/* .
EXPOSE 8080 8888
CMD ["/workspace/serv_stre"]

MAINTAINER Salfa <salfa@foxmail.com>
LABEL name="A Lighting Server"\
    version="0.1.2"\
    by="Salfa"
