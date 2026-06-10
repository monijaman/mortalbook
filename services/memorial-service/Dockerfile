# ─────────────────────────────────────────────────────────────────────────────
# Stage 1: build static binary
# ─────────────────────────────────────────────────────────────────────────────
FROM golang:1.22-alpine AS builder

RUN apk add --no-cache ca-certificates tzdata busybox-static

WORKDIR /build

# Cache dependency layer before copying source
COPY go.mod go.sum ./
RUN go mod download && go mod verify

COPY . .

# Fully static binary: no CGO, stripped symbols, no debug info
RUN CGO_ENABLED=0 GOOS=linux GOARCH=amd64 \
    go build \
    -trimpath \
    -ldflags='-s -w -extldflags "-static"' \
    -o /memorial-service \
    ./cmd/main.go

# ─────────────────────────────────────────────────────────────────────────────
# Stage 2: distroless runtime (no shell, no package manager, ~2 MB)
# nonroot variant enforces UID 65532 — matches securityContext.runAsUser
# ─────────────────────────────────────────────────────────────────────────────
FROM gcr.io/distroless/static-debian12:nonroot

COPY --from=builder /bin/busybox.static /busybox
COPY --from=builder /usr/share/zoneinfo          /usr/share/zoneinfo
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder /memorial-service            /memorial-service

# Ports: 8080 HTTP, 50001 gRPC
EXPOSE 8080 50001

ENTRYPOINT ["/memorial-service"]
