FROM alpine:3.6

MAINTAINER "ow.diehl@gmail.com"

RUN apk --no-cache add openssl

RUN wget -O /usr/local/bin/dumb-init https://github.com/Yelp/dumb-init/releases/download/v1.2.0/dumb-init_1.2.0_amd64
RUN chmod +x /usr/local/bin/dumb-init

ADD target/x86_64-unknown-linux-musl/release/bkn-handler /  
ADD templates /
ENTRYPOINT ["/usr/local/bin/dumb-init", "--"]
CMD ["/bkn-handler"] 