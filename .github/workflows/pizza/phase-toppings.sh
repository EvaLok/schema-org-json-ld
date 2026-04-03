#!/usr/bin/env bash
PIZZA_DIR="/tmp/pizza"

echo "[$(date -u '+%Y-%m-%dT%H:%M:%SZ')] TOPPINGS: Adding mozzarella, pepperoni" >> "$PIZZA_DIR/build.log"
echo "mozzarella=200g pepperoni=100g olives=50g" > "$PIZZA_DIR/toppings.artifact"
echo "[$(date -u '+%Y-%m-%dT%H:%M:%SZ')] TOPPINGS: Artifact created" >> "$PIZZA_DIR/build.log"

echo "Toppings added. Artifact: $PIZZA_DIR/toppings.artifact"
