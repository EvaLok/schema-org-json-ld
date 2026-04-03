#!/usr/bin/env bash
# signoff.sh <phase> — Validate work artifact exists, then create .done marker

PIZZA_DIR="/tmp/pizza"
BUILD_LOG="$PIZZA_DIR/build.log"
PHASE="$1"

if [ -z "$PHASE" ]; then
  echo "ERROR: Usage: signoff.sh <dough|sauce|toppings|cook>"
  exit 1
fi

ARTIFACT="$PIZZA_DIR/${PHASE}.artifact"
DONE_MARKER="$PIZZA_DIR/${PHASE}.done"

if [ -f "$DONE_MARKER" ]; then
  echo "Phase '$PHASE' already signed off."
  exit 0
fi

if [ ! -f "$ARTIFACT" ]; then
  echo "SIGNOFF REJECTED: No artifact at $ARTIFACT — run the phase script first."
  exit 1
fi

if [ ! -s "$ARTIFACT" ]; then
  echo "SIGNOFF REJECTED: Artifact at $ARTIFACT is empty."
  exit 1
fi

echo "signed_off_at=$(date -u '+%Y-%m-%dT%H:%M:%SZ')" > "$DONE_MARKER"
echo "[$(date -u '+%Y-%m-%dT%H:%M:%SZ')] SIGNOFF: $PHASE signed off" >> "$BUILD_LOG"

echo "Phase '$PHASE' signed off successfully."
