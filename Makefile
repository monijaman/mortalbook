.PHONY: help build up down logs clean build-services \
        k8s-apply k8s-delete k8s-status k8s-logs \
        k8s-rollout k8s-port-forward

NAMESPACE     ?= mortalbook
VERSION       ?= latest
REGISTRY      ?= mortalbook

help:
	@echo "Mortalbook Microservices - Available Commands"
	@echo ""
	@echo "Development (docker-compose):"
	@echo "  make up               Start all services"
	@echo "  make down             Stop all services"
	@echo "  make logs             Tail all service logs"
	@echo "  make build            Build all Docker images"
	@echo "  make clean            Remove containers and volumes"
	@echo ""
	@echo "Kubernetes:"
	@echo "  make k8s-apply        Apply all manifests (namespace-first, ordered)"
	@echo "  make k8s-delete       Tear down all resources"
	@echo "  make k8s-status       Show pod / HPA / service status"
	@echo "  make k8s-logs SVC=memorial-service"
	@echo "  make k8s-rollout SVC=memorial-service"
	@echo "  make k8s-port-forward SVC=memorial-service PORT=8080"
	@echo ""

# ─────────────────────────────────────────────────────────────────────────────
# Docker Compose
# ─────────────────────────────────────────────────────────────────────────────
build:
	docker compose build

up:
	docker compose up -d

down:
	docker compose down

logs:
	docker compose logs -f

build-services:
	@echo "Building service images…"
	docker build -t $(REGISTRY)/memorial-service:$(VERSION) ./services/memorial-service
	docker build -t $(REGISTRY)/admin-service:$(VERSION)    ./services/admin-service
	docker build -t $(REGISTRY)/frontend:$(VERSION)         ./services/frontend
	docker build -t $(REGISTRY)/admin-panel:$(VERSION)      ./services/admin-panel
	@echo "All images built."

clean:
	docker compose down -v
	docker system prune -f

# ─────────────────────────────────────────────────────────────────────────────
# Kubernetes — ordered apply (namespace must exist before other resources)
# ─────────────────────────────────────────────────────────────────────────────
k8s-apply:
	@echo "--- Creating namespace, quota, limits ---"
	kubectl apply -f k8s/namespace.yaml
	@echo "--- Applying global ConfigMaps and Secrets ---"
	kubectl apply -f k8s/configmap.yaml
	kubectl apply -f k8s/secrets.yaml
	@echo "--- Deploying infrastructure ---"
	kubectl apply -f infrastructure/postgres/k8s/
	kubectl apply -f infrastructure/redis/k8s/
	kubectl apply -f infrastructure/kafka/k8s/
	@echo "--- Waiting for infrastructure to be ready ---"
	kubectl rollout status statefulset/postgres -n $(NAMESPACE) --timeout=120s
	kubectl rollout status statefulset/redis    -n $(NAMESPACE) --timeout=60s
	@echo "--- Deploying microservices ---"
	kubectl apply -f services/memorial-service/k8s/
	kubectl apply -f services/admin-service/k8s/
	kubectl apply -f services/frontend/k8s/
	kubectl apply -f services/admin-panel/k8s/
	@echo "--- Applying network policies and ingress ---"
	kubectl apply -f k8s/network-policy.yaml
	kubectl apply -f k8s/ingress.yaml
	@echo "Deployment complete! Run 'make k8s-status' to verify."

k8s-delete:
	@echo "Removing microservices…"
	kubectl delete -f services/admin-panel/k8s/     --ignore-not-found -n $(NAMESPACE)
	kubectl delete -f services/frontend/k8s/        --ignore-not-found -n $(NAMESPACE)
	kubectl delete -f services/admin-service/k8s/   --ignore-not-found -n $(NAMESPACE)
	kubectl delete -f services/memorial-service/k8s/ --ignore-not-found -n $(NAMESPACE)
	@echo "Removing infrastructure…"
	kubectl delete -f infrastructure/kafka/k8s/    --ignore-not-found -n $(NAMESPACE)
	kubectl delete -f infrastructure/redis/k8s/    --ignore-not-found -n $(NAMESPACE)
	kubectl delete -f infrastructure/postgres/k8s/ --ignore-not-found -n $(NAMESPACE)
	@echo "Done."

k8s-status:
	@echo "=== Pods ==="
	kubectl get pods -n $(NAMESPACE) -o wide
	@echo ""
	@echo "=== Services ==="
	kubectl get svc -n $(NAMESPACE)
	@echo ""
	@echo "=== HPAs ==="
	kubectl get hpa -n $(NAMESPACE)
	@echo ""
	@echo "=== Ingress ==="
	kubectl get ingress -n $(NAMESPACE)

k8s-logs:
	kubectl logs -f deployment/$(SVC) -n $(NAMESPACE) --tail=100

k8s-rollout:
	kubectl rollout restart deployment/$(SVC) -n $(NAMESPACE)
	kubectl rollout status  deployment/$(SVC) -n $(NAMESPACE)

k8s-port-forward:
	kubectl port-forward svc/$(SVC) $(PORT):$(PORT) -n $(NAMESPACE)

.DEFAULT_GOAL := help
