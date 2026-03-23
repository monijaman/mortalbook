# ─────────────────────────────────────────────────────────────────────────────
# Stage 1: dependency builder
# Compile grpcio and psycopg2 here so the runtime image needs no build tools.
# ─────────────────────────────────────────────────────────────────────────────
FROM python:3.11-slim AS builder

RUN apt-get update && apt-get install -y --no-install-recommends \
    gcc \
    libpq-dev \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /install

COPY requirements.txt .
RUN pip install --no-cache-dir --prefix=/install/deps -r requirements.txt

# ─────────────────────────────────────────────────────────────────────────────
# Stage 2: production runtime
# ─────────────────────────────────────────────────────────────────────────────
FROM python:3.11-slim AS runtime

# Only runtime shared libraries (no gcc, no dev headers)
RUN apt-get update && apt-get install -y --no-install-recommends \
    libpq5 \
    && rm -rf /var/lib/apt/lists/*

# Non-root user
RUN groupadd --gid 1000 appuser \
    && useradd --uid 1000 --gid 1000 --no-create-home appuser

WORKDIR /app

# Copy compiled dependencies from builder
COPY --from=builder /install/deps/lib     /usr/local/lib
COPY --from=builder /install/deps/bin     /usr/local/bin

# Copy application source
COPY --chown=appuser:appuser . .

USER appuser

EXPOSE 5002

# 2 workers; tune via UVICORN_WORKERS env var in K8s Deployment if needed
CMD ["uvicorn", "app.main:app", "--host", "0.0.0.0", "--port", "5002", "--workers", "2", "--no-access-log"]
