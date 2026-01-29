#!/bin/bash
# Example script to demonstrate product comparison

BASE_URL="http://localhost:3000/api/v1"

echo "=== Product Comparison Demo ==="
echo ""

echo "1. Get CPU category schema:"
curl -s "$BASE_URL/categories/cpus" | jq '.specification_schema'
echo ""

echo "2. Adding five CPUs..."
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
      "cache": "36",
      "l1_cache": "1920",
      "l2_cache": "32",
      "l3_cache": "36",
      "architecture": "Raptor Lake",
      "process_node": "10",
      "max_memory": "128",
      "memory_type": "DDR5-5600, DDR4-3200",
      "memory_channels": 2,
      "pcie_lanes": "20",
      "pcie_version": "5.0",
      "integrated_graphics": "Intel UHD Graphics 770",
      "max_temp": "100",
      "unlocked": true,
      "virtualization": true,
      "hyperthreading": true,
      "ecc_support": false,
      "release_date": "2022-10-20",
      "lithography": "10"
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
      "cache": "64",
      "l1_cache": "1024",
      "l2_cache": "16",
      "l3_cache": "64",
      "architecture": "Zen 4",
      "process_node": "5",
      "max_memory": "128",
      "memory_type": "DDR5-5200",
      "memory_channels": 2,
      "pcie_lanes": "28",
      "pcie_version": "5.0",
      "integrated_graphics": "AMD Radeon Graphics",
      "max_temp": "95",
      "unlocked": true,
      "virtualization": true,
      "hyperthreading": true,
      "ecc_support": true,
      "release_date": "2022-09-27",
      "lithography": "5"
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
      "cache": "24",
      "l1_cache": "1120",
      "l2_cache": "20",
      "l3_cache": "24",
      "architecture": "Raptor Lake",
      "process_node": "10",
      "max_memory": "128",
      "memory_type": "DDR5-5600, DDR4-3200",
      "memory_channels": 2,
      "pcie_lanes": "20",
      "pcie_version": "5.0",
      "integrated_graphics": "Intel UHD Graphics 770",
      "max_temp": "100",
      "unlocked": true,
      "virtualization": true,
      "hyperthreading": true,
      "ecc_support": false,
      "release_date": "2022-10-20",
      "lithography": "10"
    }
  }' | jq -r '.product.id')
echo "Added Intel i5-13600K (ID: $CPU3)"

# Add AMD Ryzen 7 7800X3D
CPU4=$(curl -s -X POST "$BASE_URL/products" \
  -H "Content-Type: application/json" \
  -d '{
    "category_id": 1,
    "name": "AMD Ryzen 7 7800X3D",
    "manufacturer": "AMD",
    "model": "7800X3D",
    "price": 449.99,
    "specifications": {
      "cores": 8,
      "threads": 16,
      "base_clock": "4.2",
      "boost_clock": "5.0",
      "tdp": "120",
      "socket": "AM5",
      "cache": "96",
      "l1_cache": "512",
      "l2_cache": "8",
      "l3_cache": "96",
      "architecture": "Zen 4",
      "process_node": "5",
      "max_memory": "128",
      "memory_type": "DDR5-5200",
      "memory_channels": 2,
      "pcie_lanes": "28",
      "pcie_version": "5.0",
      "integrated_graphics": "AMD Radeon Graphics",
      "max_temp": "89",
      "unlocked": false,
      "virtualization": true,
      "hyperthreading": true,
      "ecc_support": true,
      "release_date": "2023-04-06",
      "lithography": "5"
    }
  }' | jq -r '.product.id')
echo "Added AMD Ryzen 7 7800X3D (ID: $CPU4)"

# Add Intel Core i7-14700K
CPU5=$(curl -s -X POST "$BASE_URL/products" \
  -H "Content-Type: application/json" \
  -d '{
    "category_id": 1,
    "name": "Intel Core i7-14700K",
    "manufacturer": "Intel",
    "model": "i7-14700K",
    "price": 419.99,
    "specifications": {
      "cores": 20,
      "threads": 28,
      "base_clock": "3.4",
      "boost_clock": "5.6",
      "tdp": "125",
      "socket": "LGA1700",
      "cache": "33",
      "l1_cache": "1600",
      "l2_cache": "28",
      "l3_cache": "33",
      "architecture": "Raptor Lake Refresh",
      "process_node": "10",
      "max_memory": "192",
      "memory_type": "DDR5-5600, DDR4-3200",
      "memory_channels": 2,
      "pcie_lanes": "20",
      "pcie_version": "5.0",
      "integrated_graphics": "Intel UHD Graphics 770",
      "max_temp": "100",
      "unlocked": true,
      "virtualization": true,
      "hyperthreading": true,
      "ecc_support": false,
      "release_date": "2023-10-17",
      "lithography": "10"
    }
  }' | jq -r '.product.id')
echo "Added Intel i7-14700K (ID: $CPU5)"

echo ""
echo "3. Comparing all five CPUs:"
echo ""
curl -s "$BASE_URL/products/compare?ids=$CPU1,$CPU2,$CPU3,$CPU4,$CPU5" | jq '.'

echo ""
echo "=================================================="
echo ""

echo "4. Get Microwave category schema:"
curl -s "$BASE_URL/categories/microwaves" | jq '.specification_schema'
echo ""

echo "5. Adding six microwaves..."
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
      "dimensions": "14.0 x 23.9 x 19.4",
      "weight": "36.8",
      "power_levels": 10,
      "grill_function": false,
      "smart_features": false,
      "inverter_technology": true,
      "child_lock": true,
      "preset_programs": 14,
      "defrost_modes": 3,
      "energy_rating": "A",
      "noise_level": "58",
      "door_type": "Pull Handle",
      "interior_material": "Stainless Steel",
      "exterior_finish": "Stainless Steel",
      "installation_type": "Countertop",
      "color": "Silver",
      "warranty": "1",
      "eco_mode": true,
      "keep_warm_function": true
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
      "dimensions": "12.8 x 20.5 x 17.1",
      "weight": "29.8",
      "power_levels": 10,
      "grill_function": false,
      "smart_features": false,
      "inverter_technology": false,
      "child_lock": true,
      "preset_programs": 10,
      "defrost_modes": 2,
      "energy_rating": "B",
      "noise_level": "62",
      "door_type": "Pull Handle",
      "interior_material": "Painted",
      "exterior_finish": "Black Stainless Steel",
      "installation_type": "Countertop",
      "color": "Black",
      "warranty": "1",
      "eco_mode": false,
      "keep_warm_function": false
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
      "dimensions": "11.6 x 18.5 x 18.1",
      "weight": "41.2",
      "power_levels": 11,
      "grill_function": true,
      "smart_features": true,
      "inverter_technology": true,
      "child_lock": true,
      "preset_programs": 19,
      "defrost_modes": 4,
      "energy_rating": "A+",
      "noise_level": "55",
      "door_type": "Push Button",
      "interior_material": "Stainless Steel",
      "exterior_finish": "Brushed Stainless Steel",
      "installation_type": "Countertop",
      "color": "Silver",
      "warranty": "2",
      "eco_mode": true,
      "keep_warm_function": true
    }
  }' | jq -r '.product.id')
echo "Added Breville Combi Wave 3-in-1 (ID: $MW3)"

# Add Samsung MS14K6000AS
MW4=$(curl -s -X POST "$BASE_URL/products" \
  -H "Content-Type: application/json" \
  -d '{
    "category_id": 4,
    "name": "Samsung MS14K6000AS",
    "manufacturer": "Samsung",
    "model": "MS14K6000AS",
    "price": 229.99,
    "specifications": {
      "wattage": "1000",
      "capacity": "1.4",
      "turntable_diameter": "13.6",
      "sensor_cooking": true,
      "convection": false,
      "dimensions": "12.2 x 21.7 x 17.3",
      "weight": "32.5",
      "power_levels": 10,
      "grill_function": true,
      "smart_features": false,
      "inverter_technology": false,
      "child_lock": true,
      "preset_programs": 12,
      "defrost_modes": 3,
      "energy_rating": "B+",
      "noise_level": "60",
      "door_type": "Pull Handle",
      "interior_material": "Ceramic Enamel",
      "exterior_finish": "Stainless Steel",
      "installation_type": "Countertop",
      "color": "Mirror",
      "warranty": "1",
      "eco_mode": true,
      "keep_warm_function": true
    }
  }' | jq -r '.product.id')
echo "Added Samsung MS14K6000AS (ID: $MW4)"

# Add GE JES1097SMSS
MW5=$(curl -s -X POST "$BASE_URL/products" \
  -H "Content-Type: application/json" \
  -d '{
    "category_id": 4,
    "name": "GE JES1097SMSS",
    "manufacturer": "GE",
    "model": "JES1097SMSS",
    "price": 189.99,
    "specifications": {
      "wattage": "900",
      "capacity": "0.9",
      "turntable_diameter": "10.5",
      "sensor_cooking": false,
      "convection": false,
      "dimensions": "10.3 x 17.3 x 13.6",
      "weight": "26.4",
      "power_levels": 8,
      "grill_function": false,
      "smart_features": false,
      "inverter_technology": false,
      "child_lock": true,
      "preset_programs": 6,
      "defrost_modes": 2,
      "energy_rating": "C",
      "noise_level": "65",
      "door_type": "Pull Handle",
      "interior_material": "Painted",
      "exterior_finish": "Stainless Steel",
      "installation_type": "Countertop",
      "color": "Silver",
      "warranty": "1",
      "eco_mode": false,
      "keep_warm_function": false
    }
  }' | jq -r '.product.id')
echo "Added GE JES1097SMSS (ID: $MW5)"

# Add LG LMC2075ST
MW6=$(curl -s -X POST "$BASE_URL/products" \
  -H "Content-Type: application/json" \
  -d '{
    "category_id": 4,
    "name": "LG LMC2075ST NeoChef",
    "manufacturer": "LG",
    "model": "LMC2075ST",
    "price": 379.99,
    "specifications": {
      "wattage": "1200",
      "capacity": "2.0",
      "turntable_diameter": "15.5",
      "sensor_cooking": true,
      "convection": false,
      "dimensions": "12.8 x 23.9 x 19.1",
      "weight": "38.6",
      "power_levels": 10,
      "grill_function": false,
      "smart_features": true,
      "inverter_technology": true,
      "child_lock": true,
      "preset_programs": 16,
      "defrost_modes": 3,
      "energy_rating": "A",
      "noise_level": "57",
      "door_type": "Pull Handle",
      "interior_material": "EasyClean Coating",
      "exterior_finish": "Stainless Steel",
      "installation_type": "Countertop",
      "color": "Black Stainless",
      "warranty": "2",
      "eco_mode": true,
      "keep_warm_function": true
    }
  }' | jq -r '.product.id')
echo "Added LG LMC2075ST NeoChef (ID: $MW6)"
echo ""
echo "6. Comparing all six microwaves:"
echo ""
curl -s "$BASE_URL/products/compare?ids=$MW1,$MW2,$MW3,$MW4,$MW5,$MW6" | jq '.'

echo ""
echo "7. Search demo - Find microwaves under $200:"
echo ""
curl -s "$BASE_URL/products/search?category=microwaves&max_price=200" | jq '.products[] | {name, manufacturer, price}'

echo ""
echo "=== Demo Complete ==="

