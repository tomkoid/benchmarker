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
echo "=================================================="
echo ""

echo "4. Get Microwave category schema:"
curl -s "$BASE_URL/categories/microwaves" | jq '.specification_schema'
echo ""

echo "5. Adding three microwaves..."
echo ""

# Add Panasonic NN-SN96JS
MW1=$(curl -s -X POST "$BASE_URL/products" \
  -H "Content-Type: application/json" \
  -d '{
    "category_id": 4,
    "name": "Panasonic NN-SN96JS",
    "manufacturer": "Panasonic",
    "model": "NN-SN96JS",
    "price": 329.95,
    "specifications": {
      "wattage": "1250",
      "capacity": "2.2",
      "turntable_diameter": "16.5",
      "sensor_cooking": true,
      "convection": false,
      "dimensions": "14.0 x 23.9 x 19.4"
    }
  }' | jq -r '.product.id')
echo "Added Panasonic NN-SN96JS (ID: $MW1)"

# Add Toshiba EM131A5C-BS
MW2=$(curl -s -X POST "$BASE_URL/products" \
  -H "Content-Type: application/json" \
  -d '{
    "category_id": 4,
    "name": "Toshiba EM131A5C-BS",
    "manufacturer": "Toshiba",
    "model": "EM131A5C-BS",
    "price": 159.99,
    "specifications": {
      "wattage": "1100",
      "capacity": "1.2",
      "turntable_diameter": "12.4",
      "sensor_cooking": true,
      "convection": false,
      "dimensions": "12.8 x 20.5 x 17.1"
    }
  }' | jq -r '.product.id')
echo "Added Toshiba EM131A5C-BS (ID: $MW2)"

# Add Breville BMO870BSS
MW3=$(curl -s -X POST "$BASE_URL/products" \
  -H "Content-Type: application/json" \
  -d '{
    "category_id": 4,
    "name": "Breville Combi Wave 3-in-1",
    "manufacturer": "Breville",
    "model": "BMO870BSS",
    "price": 499.95,
    "specifications": {
      "wattage": "1200",
      "capacity": "1.1",
      "turntable_diameter": "13.0",
      "sensor_cooking": false,
      "convection": true,
      "dimensions": "11.6 x 18.5 x 18.1"
    }
  }' | jq -r '.product.id')
echo "Added Breville Combi Wave 3-in-1 (ID: $MW3)"

echo ""
echo "6. Comparing all three microwaves:"
echo ""
curl -s "$BASE_URL/products/compare?ids=$MW1,$MW2,$MW3" | jq '.'

echo ""
echo "7. Search demo - Find microwaves under $200:"
echo ""
curl -s "$BASE_URL/products/search?category=microwaves&max_price=200" | jq '.products[] | {name, manufacturer, price}'

echo ""
echo "=== Demo Complete ==="

