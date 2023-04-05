FROM alpine:latest

COPY absolutus_rex /usr/local/bin/

CMD ["absolutus_rex","--help"]