# build everything
FROM golang:alpine

COPY *.go /go/src/
WORKDIR bin
RUN for f in $(find /go/src -name \*.go); do GOBIN=/src/bin go build $f; done

