#!/usr/bin/env bash
PIZZA_DIR="/tmp/pizza"

echo "[$(date -u '+%Y-%m-%dT%H:%M:%SZ')] COOK: Preheating oven to 250C" >> "$PIZZA_DIR/build.log"
echo "temp=250C time=12min style=neapolitan" > "$PIZZA_DIR/cook.artifact"
echo "[$(date -u '+%Y-%m-%dT%H:%M:%SZ')] COOK: Artifact created" >> "$PIZZA_DIR/build.log"

echo "Pizza cooked. Artifact: $PIZZA_DIR/cook.artifact"
