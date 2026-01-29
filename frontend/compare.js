// compare.js
// načte produkty z API a zobrazí dva vybrané podle URL parametrů

const API_URL = 'https://utilbench-api.tomkoid.cz/api/v1/products';

document.addEventListener('DOMContentLoaded', async () => {
  const params = new URLSearchParams(window.location.search);
  const name1 = params.get('product1');
  const name2 = params.get('product2');
  if (!name1 || !name2) return;

  // Načti produkty
  let products = [];
  try {
    const res = await fetch(API_URL);
    const data = await res.json();
    products = data.products || data;
  } catch (e) {
    document.getElementById('compare-content').innerHTML = '<p>Chyba při načítání produktů.</p>';
    return;
  }


  // Najdi produkty podle jména
  const prod1 = products.find(p => p.name === decodeURIComponent(name1));
  const prod2 = products.find(p => p.name === decodeURIComponent(name2));
  if (!prod1 || !prod2) {
    document.getElementById('compare-content').innerHTML = '<p>Produkty nenalezeny.</p>';
    return;
  }

  // Najdi všechny produkty ve stejné kategorii jako prod1 a prod2 (pro dropdown)
  const cat1 = prod1.category_id || prod1.categoryId || prod1.category;
  const cat2 = prod2.category_id || prod2.categoryId || prod2.category;
  const options1 = products.filter(p => (p.category_id||p.categoryId||p.category) == cat1);
  const options2 = products.filter(p => (p.category_id||p.categoryId||p.category) == cat2);

  function renderSelect(selectedName, options, selectId) {
    const selectedProduct = options.find(p => p.name === selectedName);
    let imageUrl = '';
    if (selectedProduct && selectedProduct.id) {
      imageUrl = `images/image-${selectedProduct.id}.jpg`;
    } else if (selectedProduct && selectedProduct.image) {
      imageUrl = selectedProduct.image;
    } else {
      imageUrl = 'images/placeholder.png';
    }
    return `
      <span style="display:inline-flex;align-items:center;gap:10px;background:#fff;border-radius:8px;padding:0 0 0 8px;box-shadow:0 2px 8px rgba(0,0,0,0.04);border:1px solid #ccc;min-width:220px;max-width:350px;">
        <img src="${imageUrl}" alt="mini" style="width:32px;height:32px;object-fit:contain;border-radius:6px;background:#f4f6f8;box-shadow:0 1px 3px rgba(0,0,0,0.04);">
        <select id="${selectId}" style="font-size:1.1em;padding:10px 18px 10px 6px;border:none;outline:none;background:transparent;min-width:160px;max-width:300px;">
          ${options.map(p => `<option value="${encodeURIComponent(p.name)}"${p.name===selectedName?' selected':''}>${p.name}</option>`).join('')}
        </select>
      </span>
    `;
  }

  // Porovnávací tabulka specifikací
  const specs1 = prod1.specifications || prod1.specs || {};
  const specs2 = prod2.specifications || prod2.specs || {};
  const allKeys = Array.from(new Set([...Object.keys(specs1), ...Object.keys(specs2)]));
  function highlight(key, v1, v2) {
    // Speciální logika pro cenu (nižší je lepší)
    if (key.toLowerCase().includes('cena') || key.toLowerCase().includes('price')) {
      const n1 = parseFloat((v1+'').replace(/[^\d.,-]/g, '').replace(',', '.'));
      const n2 = parseFloat((v2+'').replace(/[^\d.,-]/g, '').replace(',', '.'));
      if (!isNaN(n1) && !isNaN(n2)) {
        if (n1 < n2) return ['<span style="color:#1db954;font-weight:bold">'+v1+'</span>', v2];
        if (n2 < n1) return [v1, '<span style="color:#1db954;font-weight:bold">'+v2+'</span>'];
      }
      return [v1, v2];
    }
    // Pro čísla: vyšší je lepší
    const n1 = parseFloat((v1+'').replace(/[^\d.,-]/g, '').replace(',', '.'));
    const n2 = parseFloat((v2+'').replace(/[^\d.,-]/g, '').replace(',', '.'));
    if (!isNaN(n1) && !isNaN(n2)) {
      if (n1 > n2) return ['<span style="color:#1db954;font-weight:bold">'+v1+'</span>', v2];
      if (n2 > n1) return [v1, '<span style="color:#1db954;font-weight:bold">'+v2+'</span>'];
    }
    return [v1, v2];
  }
  const compareTable = `
    <table class="compare-table" style="width:100%;max-width:900px;margin:40px auto 30px;background:white;border-radius:12px;box-shadow:0 2px 10px rgba(0,0,0,0.08);overflow:hidden;">
      <thead style="background:#667eea;color:white;">
        <tr>
          <th style="width:33%">Specifikace</th>
          <th style="width:33%">${prod1.name}</th>
          <th style="width:33%">${prod2.name}</th>
        </tr>
      </thead>
      <tbody>
        ${allKeys.map(key => {
          let v1 = specs1[key] !== undefined ? specs1[key] : '<span style="color:#bbb">—</span>';
          let v2 = specs2[key] !== undefined ? specs2[key] : '<span style="color:#bbb">—</span>';
          [v1, v2] = highlight(key, v1, v2);
          return `
            <tr>
              <td style="font-weight:bold;padding:10px 8px;border-bottom:1px solid #eee;">${key}</td>
              <td style="padding:10px 8px;border-bottom:1px solid #eee;">${v1}</td>
              <td style="padding:10px 8px;border-bottom:1px solid #eee;">${v2}</td>
            </tr>
          `;
        }).join('')}
      </tbody>
    </table>
  `;
    // Popisovací text nad tabulkou
    const descText = `
      <div style="max-width:900px;margin:40px auto 0;padding:0 10px 10px 10px;">
        <h2 style="font-size:1.6em;margin-bottom:0.2em;">Detailní specifikace</h2>
        <div style="color:#888;font-size:1.08em;margin-bottom:1.2em;">
          Porovnání základních parametrů produktů <b>${prod1.name}</b> a <b>${prod2.name}</b> jako je počet jader, frekvence, velikost cache, litografie, teploty a další. Pro přesné posouzení doporučujeme sledovat i výsledky testů a recenzí.
        </div>
      </div>
    `;
    // Shrnutí: spočítej, který produkt má více "lepších" číselných hodnot (kromě ceny, kde je lepší nižší)
    let score1 = 0, score2 = 0, compared = 0;
    allKeys.forEach(key => {
      const v1 = specs1[key];
      const v2 = specs2[key];
      if (v1 === undefined || v2 === undefined) return;
      // Cena: nižší je lepší
      if (key.toLowerCase().includes('cena') || key.toLowerCase().includes('price')) {
        const n1 = parseFloat((v1+'').replace(/[^\d.,-]/g, '').replace(',', '.'));
        const n2 = parseFloat((v2+'').replace(/[^\d.,-]/g, '').replace(',', '.'));
        if (!isNaN(n1) && !isNaN(n2)) {
          compared++;
          if (n1 < n2) score1++; else if (n2 < n1) score2++;
        }
        return;
      }
      // Ostatní čísla: vyšší je lepší
      const n1 = parseFloat((v1+'').replace(/[^\d.,-]/g, '').replace(',', '.'));
      const n2 = parseFloat((v2+'').replace(/[^\d.,-]/g, '').replace(',', '.'));
      if (!isNaN(n1) && !isNaN(n2)) {
        compared++;
        if (n1 > n2) score1++; else if (n2 > n1) score2++;
      }
    });
    let summary = '';
    if (compared > 0) {
      if (score1 > score2) summary = `<div style="background:#e6ffe6;border-radius:10px;padding:18px 20px;margin:30px auto 0;max-width:700px;font-size:1.18em;color:#1a4d1a;text-align:center;font-weight:bold;">${prod1.name} má lepší parametry v ${score1} z ${compared} případů.</div>`;
      else if (score2 > score1) summary = `<div style="background:#e6ffe6;border-radius:10px;padding:18px 20px;margin:30px auto 0;max-width:700px;font-size:1.18em;color:#1a4d1a;text-align:center;font-weight:bold;">${prod2.name} má lepší parametry v ${score2} z ${compared} případů.</div>`;
      else summary = `<div style="background:#fffbe6;border-radius:10px;padding:18px 20px;margin:30px auto 0;max-width:700px;font-size:1.18em;color:#665c00;text-align:center;font-weight:bold;">Oba produkty mají stejný počet lepších parametrů (${score1} z ${compared}).</div>`;
    }
    document.getElementById('compare-content').innerHTML = `
      <div style="display:flex;gap:40px;justify-content:center;align-items:center;margin-bottom:20px;margin-top:38px;">
        ${renderSelect(prod1.name, options1, 'select1')}
        <span style="font-size:1.1em;color:#888;">vs</span>
        ${renderSelect(prod2.name, options2, 'select2')}
      </div>
      ${descText}
      ${compareTable}
      ${summary}
    `;

    // Event listenery pro změnu produktu
    document.getElementById('select1').addEventListener('change', function() {
      const new1 = this.value;
      const url = new URL(window.location.href);
      url.searchParams.set('product1', new1);
      window.location.href = url.toString();
    });
    document.getElementById('select2').addEventListener('change', function() {
      const new2 = this.value;
      const url = new URL(window.location.href);
      url.searchParams.set('product2', new2);
      window.location.href = url.toString();
    });
});

function renderProduct(product) {
  // Zobrazit všechny dostupné informace o produktu
  const specs = product.specifications || product.specs || {};
  const category = product.category_name || product.category || '';
  const description = product.description || '';
  const price = product.price ? `<div class="product-price">Cena: ${product.price} Kč</div>` : '';
  const buy = product.buy_url ? `<a class="buy" href="${product.buy_url}" target="_blank">Koupit</a>` : '';
  // Logika pro obrázek jako na hlavní stránce
  let imageUrl = '';
  if (product.id) {
    imageUrl = `images/image-${product.id}.jpg`;
  } else {
    imageUrl = product.image || 'images/placeholder.png';
  }
  return `
    <div class="card" style="min-width:320px;max-width:400px;">
      <div class="card-image">
        <img src="${imageUrl}" alt="${product.name}" onerror="this.parentElement.innerHTML='<svg width=\"100%\" height=\"100%\" viewBox=\"0 0 300 200\" xmlns=\"http://www.w3.org/2000/svg\"><defs><linearGradient id=\"grad\" x1=\"0%\" y1=\"0%\" x2=\"100%\" y2=\"100%\"><stop offset=\"0%\" style=\"stop-color:#667eea;stop-opacity:1\" /><stop offset=\"100%\" style=\"stop-color:#764ba2;stop-opacity:1\" /></linearGradient></defs><rect width=\"300\" height=\"200\" fill=\"url(#grad)\"/><circle cx=\"150\" cy=\"70\" r=\"30\" fill=\"white\" opacity=\"0.3\"/><rect x=\"80\" y=\"100\" width=\"140\" height=\"70\" fill=\"white\" opacity=\"0.2\" rx=\"5\"/><text x=\"150\" y=\"155\" font-family=\"Arial\" font-size=\"14\" fill=\"white\" text-anchor=\"middle\">Fotka není dostupná</text></svg>'">
      </div>
      <div class="card-header"><h3>${product.name}</h3></div>
      <div class="card-body">
        <div class="category-badge">${category}</div>
        <div class="card-specs">
          <table>
            ${Object.entries(specs).map(([k,v])=>`<tr><th>${k}</th><td>${v}</td></tr>`).join('')}
          </table>
        </div>
        <p>${description}</p>
        ${price}
        ${buy}
      </div>
    </div>
  `;
}
