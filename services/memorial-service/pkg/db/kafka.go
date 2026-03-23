package db

import (
	"github.com/segmentio/kafka-go"
)

func NewKafkaConnection(config *KafkaConfig) (*kafka.Conn, error) {
	conn, err := kafka.Dial("tcp", config.Brokers)
	if err != nil {
		return nil, err
	}

	defer conn.Close()

	return conn, nil
}
