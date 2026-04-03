#!/usr/bin/env bash
PIZZA_DIR="/tmp/pizza"
mkdir -p "$PIZZA_DIR"

echo "[$(date -u '+%Y-%m-%dT%H:%M:%SZ')] DOUGH: Mixing flour, water, yeast" >> "$PIZZA_DIR/build.log"
echo "flour=500g water=300ml yeast=7g salt=10g" > "$PIZZA_DIR/dough.artifact"
echo "[$(date -u '+%Y-%m-%dT%H:%M:%SZ')] DOUGH: Artifact created" >> "$PIZZA_DIR/build.log"

echo "Dough prepared. Artifact: $PIZZA_DIR/dough.artifact"
