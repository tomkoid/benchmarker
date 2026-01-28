-- Create categories table
CREATE TABLE IF NOT EXISTS categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    slug VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
    specification_schema JSONB,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create products table
CREATE TABLE IF NOT EXISTS products (
    id SERIAL PRIMARY KEY,
    category_id INTEGER NOT NULL REFERENCES categories(id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    manufacturer VARCHAR(100),
    model VARCHAR(100),
    specifications JSONB,
    price DECIMAL(10, 2),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create index on category_id for faster lookups
CREATE INDEX IF NOT EXISTS idx_products_category_id ON products(category_id);

-- Insert initial categories with specification schemas
INSERT INTO categories (name, slug, description, specification_schema) VALUES
    ('CPUs', 'cpus', 'Central Processing Units', '{
      "cores": {"type": "number", "label": "Cores", "unit": ""},
      "threads": {"type": "number", "label": "Threads", "unit": ""},
      "base_clock": {"type": "string", "label": "Base Clock", "unit": "GHz"},
      "boost_clock": {"type": "string", "label": "Boost Clock", "unit": "GHz"},
      "tdp": {"type": "string", "label": "TDP", "unit": "W"},
      "socket": {"type": "string", "label": "Socket", "unit": ""},
      "cache": {"type": "string", "label": "Cache", "unit": "MB"}
    }'::jsonb),
    ('GPUs', 'gpus', 'Graphics Processing Units', '{
      "vram": {"type": "string", "label": "VRAM", "unit": "GB"},
      "cuda_cores": {"type": "number", "label": "CUDA/Stream Processors", "unit": ""},
      "boost_clock": {"type": "string", "label": "Boost Clock", "unit": "GHz"},
      "memory_interface": {"type": "string", "label": "Memory Interface", "unit": "bit"},
      "tdp": {"type": "string", "label": "TDP", "unit": "W"},
      "outputs": {"type": "string", "label": "Display Outputs", "unit": ""}
    }'::jsonb),
    ('Dishwashers', 'dishwashers', 'Kitchen Dishwashers', '{
      "capacity": {"type": "string", "label": "Capacity", "unit": "place settings"},
      "noise_level": {"type": "string", "label": "Noise Level", "unit": "dBA"},
      "energy_star": {"type": "boolean", "label": "Energy Star Certified", "unit": ""},
      "cycles": {"type": "number", "label": "Wash Cycles", "unit": ""},
      "drying_type": {"type": "string", "label": "Drying Type", "unit": ""},
      "dimensions": {"type": "string", "label": "Dimensions (HxWxD)", "unit": "inches"}
    }'::jsonb),
    ('Microwaves', 'microwaves', 'Microwave Ovens', '{
      "wattage": {"type": "string", "label": "Wattage", "unit": "W"},
      "capacity": {"type": "string", "label": "Capacity", "unit": "cu ft"},
      "turntable_diameter": {"type": "string", "label": "Turntable Diameter", "unit": "inches"},
      "sensor_cooking": {"type": "boolean", "label": "Sensor Cooking", "unit": ""},
      "convection": {"type": "boolean", "label": "Convection", "unit": ""},
      "dimensions": {"type": "string", "label": "Dimensions (HxWxD)", "unit": "inches"}
    }'::jsonb)
ON CONFLICT (slug) DO NOTHING;
