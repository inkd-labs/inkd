#!/usr/bin/env bash
# Deploy helper for Inkd.
# Assumes `anchor` and `solana` are installed and a funded keypair is configured.
set -euo pipefail

CLUSTER="${CLUSTER:-devnet}"
PROGRAM_ID="6VKQVC6RQj5wDnXQ1pVfuTGga2iUUjFPMxAHSN6fjPck"

echo "using cluster: $CLUSTER"
echo "program id:    $PROGRAM_ID"

pushd "$(dirname "$0")/.." > /dev/null

anchor build
anchor deploy --provider.cluster "$CLUSTER"

echo "deploy finished. remember to copy the IDL to sdk/ if the interface changed."
popd > /dev/null
