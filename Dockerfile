FROM alpine:latest

COPY rex /usr/local/bin/

CMD ["chmod","a+x","/usr/local/bin/rex"]