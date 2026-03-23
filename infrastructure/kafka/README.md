# Kafka Infrastructure

Kafka 7.5 with Zookeeper for event streaming.

## Setup

```bash
kubectl apply -f zookeeper.yaml
kubectl apply -f kafka.yaml
```

## Connection

```bash
Brokers: kafka:9092
Zookeeper: zookeeper:2181
```

## Verification

```bash
kubectl exec -it deployment/kafka -- kafka-broker-api-versions.sh --bootstrap-server kafka:9092
```

## Create Topic

```bash
kubectl exec -it deployment/kafka -- kafka-topics.sh --create \
  --bootstrap-server kafka:9092 \
  --topic memorial-events \
  --partitions 3 \
  --replication-factor 1
```
