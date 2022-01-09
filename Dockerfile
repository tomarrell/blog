# Builder image
FROM golang:latest as builder

COPY . /build

WORKDIR /build
RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 go build -a -o blog . 

# Exec image
FROM alpine:latest

COPY --from=builder /build/blog /blog

ENTRYPOINT ["/blog"]
