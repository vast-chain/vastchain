#!/bin/bash
# checks if vast has a fully synced blockchain

VAST_SYNCING=$(curl -X POST --data '{"jsonrpc":"2.0","mvastod":"vast_syncing","params":[],"id":1}' http://localhost:8545 -H 'Content-Type: application/json')
RESULT=$(echo "$VAST_SYNCING" | jq -r .result)

if [ "$RESULT" == "false" ]; then
  echo "Vast is ready to start accepting traffic"
  exit 0
else
  echo "Vast is still syncing the blockchain"
  exit 1
fi
