FROM alpine:latest

COPY absolutus_rex /usr/local/bin/

CMD ["chmod","a+x","/usr/local/bin/absolutus_rex"]