# Schema Management

The specification schemas for product categories are now defined in a clean, organized YAML file instead of being hardcoded in SQL migrations.

## How It Works

### 1. Schema Definition (`schemas.yaml`)

All category specifications are defined in `schemas.yaml`:

```yaml
categories:
  - slug: cpus
    name: CPUs
    description: Central Processing Units
    specifications:
      cores:
        type: number
        label: Cores
        unit: ""
      threads:
        type: number
        label: Threads
        unit: ""
      # ... more fields
```

### 2. Automatic Sync on Startup

When the application starts, it:
1. Reads `schemas.yaml`
2. Parses the category definitions
3. Syncs them to the database (INSERT or UPDATE)
4. Logs each synced category

This happens automatically after migrations run.

### 3. Benefits

**Better Organization:**
- Clean, readable YAML format
- Easy to understand structure
- All schemas in one place

**Easy Maintenance:**
- Add new categories: just add to YAML
- Modify schemas: edit YAML and restart
- Version control friendly

**Automatic Updates:**
- Changes sync on every startup
- No manual SQL editing needed
- No migration files to manage for schema changes

## Adding a New Category

Simply edit `schemas.yaml`:

```yaml
categories:
  # ... existing categories
  
  - slug: monitors
    name: Monitors
    description: Computer Monitors
    specifications:
      screen_size:
        type: string
        label: Screen Size
        unit: inches
      resolution:
        type: string
        label: Resolution
        unit: ""
      refresh_rate:
        type: string
        label: Refresh Rate
        unit: Hz
      panel_type:
        type: string
        label: Panel Type
        unit: ""
```

Then restart the application - the new category is automatically synced!

## Modifying Existing Schemas

To add a new specification field to an existing category:

1. Edit `schemas.yaml`
2. Add the new field under the category's specifications
3. Restart the application

Example - adding L3 cache to CPUs:

```yaml
specifications:
  cores:
    type: number
    label: Cores
    unit: ""
  # ... other fields
  l3_cache:
    type: string
    label: L3 Cache
    unit: MB
```

## Field Types

Supported field types:
- `number` - Numeric values (cores, threads, etc.)
- `string` - Text values (clock speeds, dimensions, etc.)
- `boolean` - True/false values (features like sensor_cooking)

## Higher is Better Flag

Each specification includes a `higher_is_better` field that indicates whether a higher value is better for comparison:

- `true` - Higher values are better (e.g., more cores, more cache, higher clock speed)
- `false` - Lower values are better (e.g., lower TDP, lower noise level)
- `null` - Not applicable for comparison (e.g., socket type, dimensions)

This helps the frontend highlight the best values in comparisons.

**Examples:**

```yaml
# Higher is better
cores:
  type: number
  label: Cores
  unit: ""
  higher_is_better: true

# Lower is better
tdp:
  type: string
  label: TDP
  unit: W
  higher_is_better: false

# Not comparable
socket:
  type: string
  label: Socket
  unit: ""
  higher_is_better: null
```

## File Location

The `schemas.yaml` file must be in the project root (same directory as `Cargo.toml`).

## Migration Cleanup

The initial migration (`20260126000000_init.sql`) now only creates the tables. Schema population happens via the YAML sync, keeping migrations clean and simple.
