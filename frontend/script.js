// API URL - ZDE VLOŽTE ADRESU VAŠEHO SERVERU
const API_URL = 'https://utilbench-api.tomkoid.cz/api/v1/products'; // Změňte na správnou adresu
const CATEGORIES_API_URL = 'https://utilbench-api.tomkoid.cz/api/v1/categories'; // API pro kategorie
let products = []; // Globální proměnná pro produkty
let categories = []; // Globální proměnná pro kategorie

// Tato funkce kontroluje, zda se používá SPA verze
function isSPA() {
  return window.location.pathname.includes('index-spa.html');
}

// Načti kategorie z API
async function loadCategories() {
  try {
    console.log('Načítám kategorie z API:', CATEGORIES_API_URL);
    
    const response = await fetch(CATEGORIES_API_URL);
    
    if (!response.ok) {
      throw new Error(`Chyba API: ${response.status}`);
    }
    
    const data = await response.json();
    categories = data.categories || data;
    console.log('Kategorie načteny:', categories);
    
  } catch (error) {
    console.error('Chyba při načítání kategorií:', error);
  }
}

// Detectuj kterou stránku uživatel prohlíží
function getCurrentPageTag() {
  const filename = window.location.pathname.split('/').pop().split('.')[0];
  return filename || 'index';
}

// Filtruj produkty podle category_id a tagu na aktuální stránce
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
  
  // Pokud má stránka specifický tag, filtruj také podle něj
  if (tag) {
    filtered = filtered.filter(product => {
      if (!product.tag) return false;
      return product.tag.toLowerCase().includes(tag.toLowerCase());
    });
  }
  
  return filtered;
}

// Načti a zobraz produkty ze serveru
async function loadProducts() {
  try {
    console.log('Načítám data z API:', API_URL);
    
    const response = await fetch(API_URL);
    
    if (!response.ok) {
      throw new Error(`Chyba API: ${response.status}`);
    }
    
    const data = await response.json();
    products = data.products || data;
    console.log('Data načtena:', products);
    
    // Filtruj produkty podle aktuální stránky
    const filteredProducts = filterProductsByPage(products);
    displayBenchmark(filteredProducts);
    
  } catch (error) {
    console.error('Chyba při načítání dat:', error);
    const benchmarkSection = document.querySelector('.benchmark');
    if (benchmarkSection) {
      benchmarkSection.innerHTML = `<p style="color: red;">Chyba: Nelze se připojit k serveru. ${error.message}</p>`;
    }
  }
}

// Zobraz produkty v benchmark sekci
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
    
    // Příprava fotky
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

// Filtruj a zobraz produkty podle vyhledávacího dotazu
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

// Spusť loading po načtení stránky
document.addEventListener('DOMContentLoaded', async () => {
  // Nejdřív načti kategorie
  await loadCategories();
  
  // Pak načti produkty
  loadProducts();
  
  // Naslouchej změnám v search boxu
  const searchInput = document.getElementById('searchInput');
  if (searchInput) {
    searchInput.addEventListener('input', (e) => {
      filterAndDisplay(e.target.value);
    });
  }
});
