# Migration Conflict Fix

## Problem
The migration files were renamed, but the database still has the old migration records in the `_sqlx_migrations` table.

## Solutions

### Option 1: Drop and Recreate Database (Recommended for Development)

```bash
# Connect to PostgreSQL
psql -U postgres

# Drop the database
DROP DATABASE product_comparison;

# Exit psql
\q

# Run the app - it will recreate everything
cargo run
```

### Option 2: Use the Reset Script

```bash
./reset_db.sh
cargo run
```

### Option 3: Manual Migration Table Fix (Advanced)

```bash
psql -U postgres -d product_comparison

# Delete the old migration records
DELETE FROM _sqlx_migrations;

# Exit
\q

# Run the app
cargo run
```

## What Changed

The migrations were consolidated into a single file:
- `migrations/20260126000000_init.sql` - Creates tables, indexes, and inserts categories with schemas

This combines the original init migration and the schema addition migration into one atomic operation.

## After Reset

When you run `cargo run` after resetting the database:
1. Database will be created automatically if it doesn't exist
2. Migration will run and create tables
3. Categories with specification schemas will be populated
4. Server will start on port 3000
