(function () {
  'use strict';

  var titleEl = document.getElementById('doc-title');
  var loadingEl = document.getElementById('doc-loading');
  var errorEl = document.getElementById('doc-error');
  var bodyEl = document.getElementById('doc-body');

  function getPathFromQuery() {
    var params = new URLSearchParams(window.location.search);
    return (params.get('path') || '').trim();
  }

  function setPageTitle(title) {
    document.title = (title || 'Document') + ' — Miyukini COG';
    if (titleEl) titleEl.textContent = (title || 'Document') + ' — Miyukini COG';
  }

  function showLoading() {
    if (loadingEl) loadingEl.classList.remove('hidden');
    if (errorEl) errorEl.classList.add('hidden');
    if (bodyEl) bodyEl.classList.add('hidden');
  }

  function showError(msg) {
    if (loadingEl) loadingEl.classList.add('hidden');
    if (bodyEl) bodyEl.classList.add('hidden');
    if (errorEl) {
      errorEl.textContent = msg;
      errorEl.classList.remove('hidden');
    }
  }

  function showContent(html, title) {
    if (loadingEl) loadingEl.classList.add('hidden');
    if (errorEl) errorEl.classList.add('hidden');
    if (bodyEl) {
      bodyEl.innerHTML = html;
      bodyEl.classList.remove('hidden');
    }
    setPageTitle(title);
  }

  function deriveTitleFromPath(path) {
    var base = path.replace(/\.md$/i, '').split('/').pop() || '';
    return base.replace(/-/g, ' ').replace(/\b\w/g, function (c) { return c.toUpperCase(); });
  }

  function resolveRelative(basePath, href) {
    if (!href || href.indexOf(':') !== -1) return href;
    var baseDir = basePath.replace(/\/[^/]*$/, '/') || '';
    var parts = (baseDir + href).split('/');
    var out = [];
    for (var i = 0; i < parts.length; i++) {
      if (parts[i] === '..') out.pop();
      else if (parts[i] !== '.' && parts[i] !== '') out.push(parts[i]);
    }
    return out.join('/');
  }

  function rewriteMdLinks(container, currentPath) {
    if (!container || !currentPath) return;
    var links = container.querySelectorAll('a[href*=".md"]');
    var docPage = 'doc.html';
    links.forEach(function (a) {
      var href = a.getAttribute('href');
      if (!href || href.indexOf('://') !== -1) return;
      var resolved = resolveRelative(currentPath, href.replace(/#.*$/, ''));
      if (resolved) a.setAttribute('href', docPage + '?path=' + encodeURIComponent(resolved) + (href.indexOf('#') !== -1 ? href.substring(href.indexOf('#')) : ''));
    });
  }

  function loadAndRender(path) {
    if (!path) {
      showError('Paramètre "path" manquant. Exemple : doc.html?path=presentation/presentation-generale.md');
      return;
    }
    showLoading();
    fetch(path)
      .then(function (r) {
        if (!r.ok) throw new Error('Document introuvable ou erreur serveur (' + r.status + ')');
        return r.text();
      })
      .then(function (md) {
        if (typeof marked !== 'undefined') {
          marked.setOptions({ gfm: true, breaks: true });
          var html = marked.parse(md);
          var title = deriveTitleFromPath(path);
          var h1Match = md.match(/^#\s+(.+)$/m);
          if (h1Match) title = h1Match[1].trim();
          showContent(html, title);
          rewriteMdLinks(bodyEl, path);
        } else {
          bodyEl.innerHTML = '<pre>' + escapeHtml(md) + '</pre>';
          bodyEl.classList.remove('hidden');
          loadingEl.classList.add('hidden');
          setPageTitle(deriveTitleFromPath(path));
        }
      })
      .catch(function (err) {
        showError(err.message || 'Impossible de charger le document.');
      });
  }

  function escapeHtml(s) {
    if (s == null) return '';
    var div = document.createElement('div');
    div.textContent = s;
    return div.innerHTML;
  }

  function init() {
    var path = getPathFromQuery();
    loadAndRender(path);
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
  } else {
    init();
  }
})();
