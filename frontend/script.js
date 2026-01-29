// API URLs
const API_URL = 'https://utilbench-api.tomkoid.cz/api/v1/products';
const CATEGORIES_API_URL = 'https://utilbench-api.tomkoid.cz/api/v1/categories';
let products = [];
let categories = [];

// Check if using SPA version
function isSPA() {
    return window.location.pathname.includes('index.html');
}

// Load categories from API
async function loadCategories() {
    try {
        console.log('Loading categories from API:', CATEGORIES_API_URL);
        
        const response = await fetch(CATEGORIES_API_URL);
        
        if (!response.ok) {
            throw new Error(`API Error: ${response.status}`);
        }
        
        const data = await response.json();
        categories = data.categories || data;
        console.log('Categories loaded:', categories);
        
    } catch (error) {
        console.error('Error loading categories:', error);
    }
}

// Detect which page the user is viewing
function getCurrentPageTag() {
    const filename = window.location.pathname.split('/').pop().split('.')[0];
    return filename || 'index';
}

// Filter products by category_id and tag on current page
function filterProductsByPage(allProducts) {
    const pageTag = getCurrentPageTag();
    
    if (pageTag === 'index') {
        return allProducts;
    }
    
    const config = pageConfigMap[pageTag];
    if (!config) return allProducts;
    
    const { categoryIds, tag } = config;
    
    let filtered = allProducts.filter(product => {
        if (product.category_id === null || product.category_id === undefined) return false;
        return categoryIds.includes(Number(product.category_id));
    });
    
    // If page has a specific tag, filter by it too
    if (tag) {
        filtered = filtered.filter(product => {
            if (!product.tag) return false;
            return product.tag.toLowerCase().includes(tag.toLowerCase());
        });
    }
    
    return filtered;
}

// Load and display products from server
async function loadProducts() {
    try {
        console.log('Loading data from API:', API_URL);
        
        const response = await fetch(API_URL);
        
        if (!response.ok) {
            throw new Error(`API Error: ${response.status}`);
        }
        
        const data = await response.json();
        products = data.products || data;
        console.log('Data loaded:', products);
        
        // Filter products by current page
        const filteredProducts = filterProductsByPage(products);
        displayBenchmark(filteredProducts);
        
    } catch (error) {
        console.error('Error loading data:', error);
        const benchmarkSection = document.querySelector('.benchmark');
        if (benchmarkSection) {
            benchmarkSection.innerHTML = `<p style="color: red;">Error: Cannot connect to server. ${error.message}</p>`;
        }
    }
}

// Display products in benchmark section
function displayBenchmark(data) {
    const benchmarkSection = document.querySelector('.benchmark');
    if (!benchmarkSection) return;
    
    benchmarkSection.innerHTML = '<h2>Benchmark – Porovnání Produktů</h2><div class="products-container"></div>';
    const container = benchmarkSection.querySelector('.products-container');
    
    if (!data || data.length === 0) {
        benchmarkSection.innerHTML += '<p>Žádné produkty nenalezeny.</p>';
        return;
    }
    
    data.forEach(product => {
        const card = document.createElement('div');
        card.className = 'card';
        
        // Prepare image
        let imageUrl = '';
        const productId = product.id || product.product_id;
        if (productId) {
            imageUrl = `images/image-${productId}.jpg`;
        } else {
            imageUrl = 'images/placeholder.png';
        }
        
        let specsHTML = '';
        if (product.specifications && typeof product.specifications === 'object') {
            specsHTML = '<div class="card-specs"><table>';
            for (const [key, value] of Object.entries(product.specifications)) {
                specsHTML += `<tr><th>${key}</th><td>${value}</td></tr>`;
            }
            specsHTML += '</table></div>';
        }
        
        const categoryName = product.category_name || 'Ostatní';
        
        card.innerHTML = `
            <div class="card-header">
                <div class="card-image-small"><img src="${imageUrl}" alt="${product.name}" onerror="this.parentElement.style.background='linear-gradient(135deg, #667eea 0%, #764ba2 100%)'"></div>
                <h3>${product.name}</h3>
            </div>
            <div class="card-body">
                <p class="category-badge">${categoryName}</p>
                ${specsHTML}
            </div>
        `;
        
        container.appendChild(card);
    });
}

// Filter and display products by search query
function filterAndDisplay(query) {
    if (!query.trim()) {
        const filteredProducts = filterProductsByPage(products);
        displayBenchmark(filteredProducts);
        return;
    }
    
    const pageFiltered = filterProductsByPage(products);
    const filtered = pageFiltered.filter(p => 
        (p.name && p.name.toLowerCase().includes(query.toLowerCase())) ||
        (p.category_name && p.category_name.toLowerCase().includes(query.toLowerCase()))
    );
    
    displayBenchmark(filtered);
}

// Start loading after page loads
document.addEventListener('DOMContentLoaded', async () => {
    // Load categories first
    await loadCategories();
    
    // Then load products
    loadProducts();
    
    // Listen to search box changes
    const searchInput = document.getElementById('searchInput');
    if (searchInput) {
        searchInput.addEventListener('input', (e) => {
            filterAndDisplay(e.target.value);
        });
    }
});
