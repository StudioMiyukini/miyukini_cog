(function () {
  'use strict';

  var INDEX_URL = 'index.json';
  var DOC_PAGE = 'doc.html';
  var data = { filters: [], cards: [] };
  var selectedFilters = { category: '', strate: '', type: '' };
  var searchQuery = '';

  function loadIndex() {
    return fetch(INDEX_URL)
      .then(function (r) {
        if (!r.ok) throw new Error('Impossible de charger index.json');
        return r.json();
      })
      .then(function (json) {
        data.filters = json.filters || [];
        data.cards = (json.cards || []).slice().sort(function (a, b) {
          return (a.order || 0) - (b.order || 0);
        });
        return data;
      });
  }

  function renderFilters() {
    var wrap = document.getElementById('filters-wrap');
    if (!wrap) return;
    wrap.innerHTML = '';
    data.filters.forEach(function (filter) {
      var label = document.createElement('label');
      label.textContent = filter.label + ' :';
      label.setAttribute('for', 'filter-' + filter.id);
      wrap.appendChild(label);
      var sel = document.createElement('select');
      sel.id = 'filter-' + filter.id;
      sel.setAttribute('data-filter-id', filter.id);
      sel.innerHTML = '<option value="">Tous</option>';
      (filter.options || []).forEach(function (opt) {
        sel.innerHTML += '<option value="' + escapeHtml(opt.value) + '">' + escapeHtml(opt.label) + '</option>';
      });
      sel.addEventListener('change', onFilterChange);
      wrap.appendChild(sel);
    });
  }

  function escapeHtml(s) {
    if (s == null) return '';
    var div = document.createElement('div');
    div.textContent = s;
    return div.innerHTML;
  }

  function onFilterChange() {
    data.filters.forEach(function (filter) {
      var sel = document.getElementById('filter-' + filter.id);
      if (sel) selectedFilters[filter.id] = (sel.value || '').trim();
    });
    applyFiltersAndSearch();
  }

  function onSearchInput() {
    searchQuery = (document.getElementById('search-input').value || '').trim().toLowerCase();
    applyFiltersAndSearch();
  }

  function cardMatchesFilters(card) {
    var tags = card.tags || [];
    if (selectedFilters.category && tags.indexOf(selectedFilters.category) === -1) return false;
    if (selectedFilters.strate && tags.indexOf(selectedFilters.strate) === -1) return false;
    if (selectedFilters.type && tags.indexOf(selectedFilters.type) === -1) return false;
    if (searchQuery) {
      var title = (card.title || '').toLowerCase();
      var desc = (card.description || '').toLowerCase();
      if (title.indexOf(searchQuery) === -1 && desc.indexOf(searchQuery) === -1) return false;
    }
    return true;
  }

  function applyFiltersAndSearch() {
    var container = document.getElementById('cards-container');
    var noResults = document.getElementById('no-results');
    var countEl = document.getElementById('card-count');
    if (!container) return;
    var visible = 0;
    [].forEach.call(container.querySelectorAll('.card'), function (cardEl) {
      var id = cardEl.getAttribute('data-card-id');
      var card = data.cards.find(function (c) { return c.id === id; });
      var show = card && cardMatchesFilters(card);
      cardEl.classList.toggle('hidden-by-filter', !show);
      if (show) visible++;
    });
    if (noResults) noResults.classList.toggle('hidden', visible > 0);
    if (countEl) countEl.textContent = visible + ' / ' + data.cards.length + ' document(s)';
  }

  function buildDocUrl(path) {
    return DOC_PAGE + '?path=' + encodeURIComponent(path);
  }

  // Labels des catégories pour les bandeaux
  var categoryLabels = {
    presentation: 'Présentation',
    architecture: 'Architecture',
    cores: 'Cores',
    interface: 'Interface',
    tools: 'Outils',
    services: 'Services',
    mws: 'Webway',
    security: 'Sécurité',
    admin: 'Admin',
    reference: 'Référence'
  };

  function getCategoryFromTags(tags) {
    if (!tags || !tags.length) return null;
    var categories = ['presentation', 'architecture', 'cores', 'interface', 'tools', 'services', 'mws', 'security', 'admin', 'reference'];
    for (var i = 0; i < tags.length; i++) {
      if (categories.indexOf(tags[i]) !== -1) return tags[i];
    }
    return null;
  }

  function renderCards() {
    var container = document.getElementById('cards-container');
    if (!container) return;
    container.innerHTML = '';
    data.cards.forEach(function (card) {
      var a = document.createElement('a');
      a.href = buildDocUrl(card.path);
      a.className = 'card';
      a.setAttribute('data-card-id', card.id);
      
      // Bandeau de catégorie
      var category = getCategoryFromTags(card.tags);
      var ribbon = '';
      if (category && categoryLabels[category]) {
        ribbon = '<span class="card-ribbon cat-' + category + '">' + categoryLabels[category] + '</span>';
      }
      
      var icon = card.icon ? '<span class="card-icon">' + getIcon(card.icon) + '</span>' : '';
      a.innerHTML =
        ribbon +
        icon +
        '<h2 class="card-title">' + escapeHtml(card.title || '') + '</h2>' +
        '<p class="card-desc">' + escapeHtml(card.description || '') + '</p>';
      container.appendChild(a);
    });
    applyFiltersAndSearch();
  }

  function getIcon(name) {
    var icons = {
      home: '⌂', cube: '◇', scale: '⚖', layers: '▤', shield: '🛡', cpu: '◉', crown: '♔',
      database: '▣', key: '🔑', 'shield-check': '✓', bell: '🔔', truck: '⬡', link: '⎔',
      toolbox: '▦', fingerprint: '◈', globe: '◉', network: '⊞', radar: '◎', grid: '▦',
      apps: '▥', bot: '🤖', briefcase: '▤', calculator: '▣', palette: '🎨', calendar: '▤',
      clock: '⏱', 'shopping-cart': '▤', users: '▥', gamepad: '▤', activity: '▤',
      server: '▣', repeat: '↻', search: '⌕', zap: '⚡', lock: '🔒', book: '▤', tag: '▤',
      settings: '⚙'
    };
    return icons[name] || '•';
  }

  function init() {
    loadIndex()
      .then(function () {
        renderFilters();
        renderCards();
        var searchInput = document.getElementById('search-input');
        if (searchInput) {
          searchInput.addEventListener('input', onSearchInput);
          searchInput.addEventListener('search', onSearchInput);
        }
      })
      .catch(function (err) {
        var container = document.getElementById('cards-container');
        if (container) container.innerHTML = '<p class="no-results">Erreur : ' + escapeHtml(err.message) + '</p>';
      });
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
  } else {
    init();
  }
})();
