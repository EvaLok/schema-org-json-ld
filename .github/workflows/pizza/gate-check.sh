#!/usr/bin/env bash
# gate-check.sh — PreToolUse hook for the pizza pipeline
# Reads tool input JSON from stdin, enforces phase ordering via marker files.
# Exit 0 = allow, Exit 2 = block (stderr shown to agent)

PIZZA_DIR="/tmp/pizza"

INPUT=$(cat)
CMD=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

if [ -z "$CMD" ]; then
  exit 0
fi

# Block direct writes to pizza marker/artifact files
if echo "$CMD" | grep -qE '>\s*/tmp/pizza/'; then
  echo "BLOCKED: Direct writes to /tmp/pizza/ are not allowed. Use the phase scripts and signoff.sh." >&2
  exit 2
fi

# --- Phase script gating ---

if echo "$CMD" | grep -qE 'phase-dough\.sh'; then
  if [ -f "$PIZZA_DIR/dough.done" ]; then
    echo "BLOCKED: Dough phase already completed." >&2
    exit 2
  fi
  exit 0
fi

if echo "$CMD" | grep -qE 'phase-sauce\.sh'; then
  if [ ! -f "$PIZZA_DIR/dough.done" ]; then
    echo "BLOCKED: Cannot start sauce — dough phase not signed off." >&2
    exit 2
  fi
  if [ -f "$PIZZA_DIR/sauce.done" ]; then
    echo "BLOCKED: Sauce phase already completed." >&2
    exit 2
  fi
  exit 0
fi

if echo "$CMD" | grep -qE 'phase-toppings\.sh'; then
  if [ ! -f "$PIZZA_DIR/sauce.done" ]; then
    echo "BLOCKED: Cannot start toppings — sauce phase not signed off." >&2
    exit 2
  fi
  if [ -f "$PIZZA_DIR/toppings.done" ]; then
    echo "BLOCKED: Toppings phase already completed." >&2
    exit 2
  fi
  exit 0
fi

if echo "$CMD" | grep -qE 'phase-cook\.sh'; then
  if [ ! -f "$PIZZA_DIR/toppings.done" ]; then
    echo "BLOCKED: Cannot start cook — toppings phase not signed off." >&2
    exit 2
  fi
  if [ -f "$PIZZA_DIR/cook.done" ]; then
    echo "BLOCKED: Cook phase already completed." >&2
    exit 2
  fi
  exit 0
fi

# --- Signoff gating ---

if echo "$CMD" | grep -qE 'signoff\.sh'; then
  PHASE=$(echo "$CMD" | grep -oP 'signoff\.sh\s+\K\S+')

  if [ -z "$PHASE" ]; then
    echo "BLOCKED: signoff.sh requires a phase argument (dough|sauce|toppings|cook)." >&2
    exit 2
  fi

  case "$PHASE" in
    dough) ;;
    sauce)
      if [ ! -f "$PIZZA_DIR/dough.done" ]; then
        echo "BLOCKED: Cannot sign off sauce — dough not signed off yet." >&2
        exit 2
      fi
      ;;
    toppings)
      if [ ! -f "$PIZZA_DIR/sauce.done" ]; then
        echo "BLOCKED: Cannot sign off toppings — sauce not signed off yet." >&2
        exit 2
      fi
      ;;
    cook)
      if [ ! -f "$PIZZA_DIR/toppings.done" ]; then
        echo "BLOCKED: Cannot sign off cook — toppings not signed off yet." >&2
        exit 2
      fi
      ;;
    *)
      echo "BLOCKED: Unknown phase '$PHASE'. Valid: dough, sauce, toppings, cook." >&2
      exit 2
      ;;
  esac

  exit 0
fi

# Not a phase command — allow through
exit 0
