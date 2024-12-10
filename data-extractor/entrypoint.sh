#!/bin/bash

set -euo pipefail

echo "Container started: $(az version | jq -c .)"
echo "Loaded configuration from environment:"
echo "ADO_DE_ORG   $ADO_DE_ORG"
echo "BROKER       $BROKER"
echo "USER_ID      $USER_ID"
WIQL="${WIQL//&user;/"$USER_ID"}"
echo "WIQL         $WIQL"
echo ""

az devops configure -d organization=$ADO_DE_ORG
echo "$AZURE_DEVOPS_EXT_PAT" | az devops login --only-show-errors

echo "Querying ADO..."
ADO_OUTPUT=$(az boards query --wiql "$WIQL" -o json | jq -c .)
echo "$ADO_OUTPUT"

echo ""
echo "(\ /)"
echo "(-.-)"
python3 /app/queue.py "$BROKER" "$ADO_OUTPUT" "$BROKER_PASSWORD"
