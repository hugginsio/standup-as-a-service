#!/bin/sh

set -eu

echo "Container started: $(az version | jq -c .)"
echo "Loaded configuration from environment:"
echo "ADO_DE_USER  $ADO_DE_USER"
echo "ADO_DE_EMAIL $ADO_DE_EMAIL"
echo "AD_DE_ORG    $ADO_DE_ORG"
echo ""
echo "Attempting to connect to Redis:"
echo "$REDIS PING...$(redis-cli -h "$REDIS" -p 6379 PING)"
echo ""

az devops configure -d organization=$ADO_DE_ORG
echo "$AZURE_DEVOPS_EXT_PAT" | az devops login --only-show-errors
ADO_OUTPUT=$(az boards query --wiql "$WIQL" -o json | jq -c .)

echo "Querying ADO..."
echo "$ADO_OUTPUT"
echo ""
echo "Posting to Redis..."
echo "XADD work-items * data '$ADO_OUTPUT'" | redis-cli -h "$REDIS" -p 6379
