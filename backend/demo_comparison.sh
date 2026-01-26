#!/bin/bash
# Example script to demonstrate product comparison

BASE_URL="http://localhost:3000/api/v1"

echo "=== Product Comparison Demo ==="
echo ""

echo "1. Get CPU category schema:"
curl -s "$BASE_URL/categories/cpus" | jq '.specification_schema'
echo ""

echo "2. Adding three CPUs..."
echo ""

# Add Intel Core i9-13900K
CPU1=$(curl -s -X POST "$BASE_URL/products" \
  -H "Content-Type: application/json" \
  -d '{
    "category_id": 1,
    "name": "Intel Core i9-13900K",
    "manufacturer": "Intel",
    "model": "i9-13900K",
    "price": 589.99,
    "specifications": {
      "cores": 24,
      "threads": 32,
      "base_clock": "3.0",
      "boost_clock": "5.8",
      "tdp": "125",
      "socket": "LGA1700",
      "cache": "36"
    }
  }' | jq -r '.product.id')
echo "Added Intel i9-13900K (ID: $CPU1)"

# Add AMD Ryzen 9 7950X
CPU2=$(curl -s -X POST "$BASE_URL/products" \
  -H "Content-Type: application/json" \
  -d '{
    "category_id": 1,
    "name": "AMD Ryzen 9 7950X",
    "manufacturer": "AMD",
    "model": "7950X",
    "price": 549.99,
    "specifications": {
      "cores": 16,
      "threads": 32,
      "base_clock": "4.5",
      "boost_clock": "5.7",
      "tdp": "170",
      "socket": "AM5",
      "cache": "64"
    }
  }' | jq -r '.product.id')
echo "Added AMD Ryzen 9 7950X (ID: $CPU2)"

# Add Intel Core i5-13600K
CPU3=$(curl -s -X POST "$BASE_URL/products" \
  -H "Content-Type: application/json" \
  -d '{
    "category_id": 1,
    "name": "Intel Core i5-13600K",
    "manufacturer": "Intel",
    "model": "i5-13600K",
    "price": 319.99,
    "specifications": {
      "cores": 14,
      "threads": 20,
      "base_clock": "3.5",
      "boost_clock": "5.1",
      "tdp": "125",
      "socket": "LGA1700",
      "cache": "24"
    }
  }' | jq -r '.product.id')
echo "Added Intel i5-13600K (ID: $CPU3)"

echo ""
echo "3. Comparing all three CPUs:"
echo ""
curl -s "$BASE_URL/products/compare?ids=$CPU1,$CPU2,$CPU3" | jq '.'

echo ""
echo "=== Demo Complete ==="
