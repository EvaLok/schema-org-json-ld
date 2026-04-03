#!/usr/bin/env bash
PIZZA_DIR="/tmp/pizza"

echo "[$(date -u '+%Y-%m-%dT%H:%M:%SZ')] SAUCE: Crushing tomatoes, adding garlic" >> "$PIZZA_DIR/build.log"
echo "tomatoes=400g garlic=3cloves basil=fresh olive_oil=2tbsp" > "$PIZZA_DIR/sauce.artifact"
echo "[$(date -u '+%Y-%m-%dT%H:%M:%SZ')] SAUCE: Artifact created" >> "$PIZZA_DIR/build.log"

echo "Sauce prepared. Artifact: $PIZZA_DIR/sauce.artifact"
