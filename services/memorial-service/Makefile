.PHONY: proto build test lint run tidy

# ── Proto ────────────────────────────────────────────────────────────────────
# Install once: go install github.com/bufbuild/buf/cmd/buf@latest
#               go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
#               go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest
proto:
	buf generate

# ── Build ─────────────────────────────────────────────────────────────────────
build:
	go build -o bin/memorial-service ./cmd/...

# ── Test ──────────────────────────────────────────────────────────────────────
test:
	go test -v -race -count=1 ./...

test-cover:
	go test -v -race -count=1 -coverprofile=coverage.out ./...
	go tool cover -html=coverage.out -o coverage.html

# ── Lint ──────────────────────────────────────────────────────────────────────
# Install once: go install github.com/golangci/golangci-lint/cmd/golangci-lint@latest
lint:
	golangci-lint run ./...

# ── Run locally ───────────────────────────────────────────────────────────────
run:
	go run ./cmd/...

# ── Dependencies ──────────────────────────────────────────────────────────────
tidy:
	go mod tidy
