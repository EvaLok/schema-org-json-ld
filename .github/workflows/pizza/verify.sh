#!/usr/bin/env bash
# verify.sh — Post-agent verification of pipeline completion

PIZZA_DIR="/tmp/pizza"
PASS=true

echo "=== Pizza Pipeline Verification ==="
echo ""

for PHASE in dough sauce toppings cook; do
  A="$PIZZA_DIR/${PHASE}.artifact"
  D="$PIZZA_DIR/${PHASE}.done"

  if [ -f "$A" ] && [ -f "$D" ]; then
    echo "PASS: $PHASE"
  else
    echo "FAIL: $PHASE — artifact=$([ -f "$A" ] && echo found || echo MISSING) signoff=$([ -f "$D" ] && echo found || echo MISSING)"
    PASS=false
  fi
done

echo ""
echo "=== Build Log ==="
if [ -f "$PIZZA_DIR/build.log" ]; then
  cat "$PIZZA_DIR/build.log"
else
  echo "(no build log found)"
fi

echo ""
if [ "$PASS" = true ]; then
  echo "RESULT: ALL PHASES COMPLETE"
else
  echo "RESULT: INCOMPLETE"
  exit 1
fi
