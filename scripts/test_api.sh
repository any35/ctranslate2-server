#!/bin/bash

# Configuration
URL="http://localhost:8080/v1/chat/completions"
MODEL=${1:-"nllb"}
CONTENT=${2:-"Hello, how are you today?"}

echo "Testing model: $MODEL"
echo "Content: $CONTENT"
echo "---"

curl -i -X POST "$URL" \
  -H "Content-Type: application/json" \
  -d "{
    \"model\": \"$MODEL\",
    \"messages\": [
      {\"role\": \"user\", \"content\": \"$CONTENT\"}
    ]
  }"

echo -e "\n---"
echo "Test complete."
