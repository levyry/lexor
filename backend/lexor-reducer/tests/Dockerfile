FROM alpine:3.23

RUN apk add --no-cache coreutils npm bash socat

WORKDIR /usr/src/app

RUN npm init -y && \
    npm install @dallaylaen/ski-interpreter

COPY server.sh .
RUN chmod +x server.sh

EXPOSE 1500

CMD ["socat", "TCP-LISTEN:1500,reuseaddr,fork", "EXEC:/usr/src/app/server.sh"]
