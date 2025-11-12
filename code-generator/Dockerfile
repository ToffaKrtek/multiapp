FROM golang:1.25-bookworm AS builder
WORKDIR /app
COPY go.mod go.sum ./
RUN go mod download

COPY . .

RUN go build -o code-generator ./cmd/main.go

FROM alpine:latest
WORKDIR /root/
COPY --from=builder /app/code-generator .
CMD ["./code-generator"]
