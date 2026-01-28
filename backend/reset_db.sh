#!/bin/bash
# Script to reset migrations - USE WITH CAUTION

echo "This will drop and recreate the database."
echo "Make sure you don't have important data!"
read -p "Continue? (y/N) " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    exit 1
fi

# Load environment variables
source .env

# Extract database name from URL
DB_NAME=$(echo $DATABASE_URL | sed 's/.*\///' | sed 's/?.*//')
DB_HOST=$(echo $DATABASE_URL | sed 's|postgres://||' | sed 's|/.*||')

echo "Dropping database: $DB_NAME"
psql -h localhost -U postgres -c "DROP DATABASE IF EXISTS $DB_NAME;"

echo "Database dropped. Run 'cargo run' to recreate with fresh migrations."
