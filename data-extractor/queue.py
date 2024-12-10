#!/usr/bin/env python

import argparse
import json
import pika

parser = argparse.ArgumentParser(
    prog="queue",
    description="Enqueue data to RabbitMQ.",
)

parser.add_argument(
    "broker",
    help="The broker URL",
    type=str,
)

parser.add_argument(
    "data",
    help="The data to enqueue",
    type=str,
)

parser.add_argument(
    "password",
    help="The broker password",
    type=str,
)

args = parser.parse_args()

if not args.data.strip():
    print("No data received from ADO.")
    exit(0)

print("Connecting to RabbitMQ at " + args.broker)

credentials = pika.PlainCredentials("user", args.password)
connection = pika.BlockingConnection(pika.ConnectionParameters(host=args.broker, port=5672, credentials=credentials), )

channel = connection.channel()
channel.queue_declare(queue="work-items")

try:
    data_array = json.loads(args.data.strip())
    if not isinstance(data_array, list):
        raise ValueError("Input data must be a JSON array")

    for item in data_array:
        message = json.dumps(item)
        channel.basic_publish(exchange="", routing_key="work-items", body=message)

    print(f"Successfully published {len(data_array)} item(s).")
except json.JSONDecodeError as e:
    print(f"Error parsing JSON data: {e}")
    exit(1)
except ValueError as e:
    print(f"Could not parse value: {e}")
    exit(1)
except Exception as e:
    print(f"Unexpected error: {e}")
    exit(1)
finally:
    connection.close()
