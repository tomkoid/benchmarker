-- Create categories table
CREATE TABLE IF NOT EXISTS categories (
    id SERIAL PRIMARY KEY,
    name VARCHAR(100) NOT NULL UNIQUE,
    slug VARCHAR(100) NOT NULL UNIQUE,
    description TEXT,
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
CREATE INDEX idx_products_category_id ON products(category_id);

-- Insert some initial categories
INSERT INTO categories (name, slug, description) VALUES
    ('CPUs', 'cpus', 'Central Processing Units'),
    ('GPUs', 'gpus', 'Graphics Processing Units'),
    ('Dishwashers', 'dishwashers', 'Kitchen Dishwashers'),
    ('Microwaves', 'microwaves', 'Microwave Ovens')
ON CONFLICT (slug) DO NOTHING;
