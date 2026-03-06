// MIPOWER — app.js (E03+E04+E05+E06+E07)

// ── État global ───────────────────────────────────────────

let allSequences = [];
let currentSlug  = null;
let currentFiles = [];

// ── Navigation ────────────────────────────────────────────

const navItems = document.querySelectorAll('.nav-item');
const views    = document.querySelectorAll('.view');

function switchView(viewId) {
  views.forEach(v => v.classList.remove('active'));
  navItems.forEach(n => n.classList.remove('active'));
  const view = document.getElementById(`view-${viewId}`);
  if (view) view.classList.add('active');
  const nav = document.querySelector(`[data-view="${viewId}"]`);
  if (nav) nav.classList.add('active');
}

navItems.forEach(item => {
  item.addEventListener('click', (e) => {
    e.preventDefault();
    switchView(item.dataset.view);
  });
});

// ── Sidebar toggle (persiste en localStorage) ─────────────

const sidebar = document.getElementById('sidebar');
const toggle  = document.getElementById('sidebarToggle');

if (localStorage.getItem('sidebarCollapsed') === '1') {
  sidebar?.classList.add('collapsed');
  if (toggle) toggle.textContent = '›';
}

toggle?.addEventListener('click', () => {
  sidebar?.classList.toggle('collapsed');
  const collapsed = sidebar?.classList.contains('collapsed');
  toggle.textContent = collapsed ? '›' : '‹';
  toggle.setAttribute('aria-label', collapsed ? 'Ouvrir la barre latérale' : 'Réduire la barre latérale');
  localStorage.setItem('sidebarCollapsed', collapsed ? '1' : '0');
});

// ── SSE — suivi live des modifications ────────────────────

const sse = new EventSource('/sse');
sse.onmessage = (e) => {
  const slug = e.data.trim();
  loadSequences();
  if (currentSlug && currentSlug === slug) {
    loadProgress(slug);
  }
};
sse.onerror = () => { /* reconnect automatique navigateur */ };

// ── Settings ──────────────────────────────────────────────

const savedRoot  = localStorage.getItem('mipRoot') || '';
const mipRootInput = document.getElementById('mipRoot');
if (mipRootInput) mipRootInput.value = savedRoot;

document.getElementById('saveMipRoot')?.addEventListener('click', async () => {
  const root = mipRootInput?.value?.trim();
  if (!root) return;
  localStorage.setItem('mipRoot', root);
  const status = document.getElementById('settingsStatus');
  try {
    await fetch('/api/settings', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ mip_root: root }),
    });
    if (status) { status.textContent = 'Chemin enregistré.'; setTimeout(() => status.textContent = '', 2000); }
  } catch {
    if (status) status.textContent = 'Erreur lors de la sauvegarde.';
  }
  loadSequences();
});

// ── Dashboard — chargement séquences ─────────────────────

async function loadSequences() {
  const grid = document.getElementById('sequenceGrid');
  if (!grid) return;
  try {
    const res  = await fetch('/api/sequences');
    const data = await res.json();
    allSequences = data.sequences || [];
    renderSequences(grid);
  } catch (e) {
    grid.innerHTML = `<div class="loading">Erreur : ${e.message}</div>`;
  }
}

function renderSequences(grid) {
  if (!grid) return;
  if (!allSequences.length) {
    grid.innerHTML = '<div class="loading">Aucune séquence trouvée. Configure le chemin MIP dans les Paramètres.</div>';
    return;
  }
  const search    = document.getElementById('searchInput')?.value?.toLowerCase() || '';
  const statusVal = document.getElementById('statusFilter')?.value || '';
  const filtered  = allSequences.filter(s =>
    (!search    || s.slug.toLowerCase().includes(search) || s.name.toLowerCase().includes(search)) &&
    (!statusVal || s.status === statusVal)
  );
  if (!filtered.length) {
    grid.innerHTML = '<div class="loading">Aucun résultat pour ce filtre.</div>';
    return;
  }
  grid.innerHTML = filtered.map(s => sequenceCardHTML(s)).join('');
  grid.querySelectorAll('.sequence-card').forEach(card => {
    card.addEventListener('click', () => openSequence(card.dataset.slug));
  });
}

function sequenceCardHTML(s) {
  const tClass    = (s.task_class || '').toLowerCase();
  const statusCls = s.status || 'active';
  const phase     = s.current_phase ? `<span class="badge">${s.current_phase}</span>` : '';
  return `
    <div class="sequence-card" data-slug="${s.slug}">
      <div class="card-header">
        <span class="card-slug">${s.slug}</span>
        <span class="card-date">${s.date}</span>
      </div>
      <div class="card-name">${s.name || ''}</div>
      <div class="card-badges">
        ${s.task_class ? `<span class="badge badge-${tClass}">${s.task_class}</span>` : ''}
        ${s.complexity ? `<span class="badge">${s.complexity}</span>` : ''}
        ${phase}
        <span class="badge badge-${statusCls}">${statusCls}</span>
      </div>
    </div>`;
}

document.getElementById('searchInput')?.addEventListener('input', () => {
  renderSequences(document.getElementById('sequenceGrid'));
});
document.getElementById('statusFilter')?.addEventListener('change', () => {
  renderSequences(document.getElementById('sequenceGrid'));
});

// ── Rapport — ouverture séquence ──────────────────────────

async function openSequence(slug) {
  currentSlug = slug;
  switchView('report');

  const tree = document.getElementById('artefactTree');
  if (tree) tree.innerHTML = '<div class="loading">Chargement…</div>';
  document.getElementById('reportBody').innerHTML = '<p class="empty-state">Sélectionne un artefact.</p>';
  document.getElementById('reportToc').innerHTML  = '';

  try {
    const res  = await fetch(`/api/artefacts/${encodeURIComponent(slug)}`);
    const data = await res.json();
    currentFiles = data.files || [];
    renderArtefactTree(tree, currentFiles, data.path || '');
  } catch (e) {
    if (tree) tree.innerHTML = `<div class="loading">Erreur : ${e.message}</div>`;
  }

  loadProgress(slug);
}

function renderArtefactTree(tree, files, seqRelPath) {
  if (!tree) return;
  if (!files.length) {
    tree.innerHTML = '<p class="empty-state">Aucun artefact .md trouvé.</p>';
    return;
  }

  // Grouper par répertoire de premier niveau relatif à la séquence
  const groups = {};
  for (const f of files) {
    // f est relatif à mip_root : ex. ".mip/sequences/2026-03-06-slug/briefs/brief.md"
    const afterSeq = seqRelPath ? f.replace(seqRelPath + '/', '') : f;
    const parts    = afterSeq.split('/');
    const group    = parts.length > 1 ? parts[0] : '_root';
    if (!groups[group]) groups[group] = [];
    groups[group].push({ full: f, label: parts[parts.length - 1] });
  }

  const ORDER = ['briefs', 'specs', 'plans_p3', 'agents', 'phases', '_root'];
  const keys  = [...new Set([...ORDER.filter(k => groups[k]), ...Object.keys(groups).filter(k => !ORDER.includes(k))])];

  let html = '<nav class="tree-nav">';
  for (const group of keys) {
    const label = group === '_root' ? 'Racine' : group;
    html += `<div class="tree-group"><div class="tree-group-label">${label}/</div><ul>`;
    for (const item of groups[group]) {
      html += `<li><button class="tree-item" data-path="${item.full}" title="${item.full}">${item.label}</button></li>`;
    }
    html += '</ul></div>';
  }
  html += '</nav>';
  tree.innerHTML = html;

  // Auto-charge le brief si disponible
  const brief = files.find(f => f.includes('/briefs/'));
  if (brief) loadArtefact(brief);

  tree.querySelectorAll('.tree-item').forEach(btn => {
    btn.addEventListener('click', () => {
      tree.querySelectorAll('.tree-item').forEach(b => b.classList.remove('active'));
      btn.classList.add('active');
      loadArtefact(btn.dataset.path);
    });
  });
}

// ── Rapport — chargement artefact ─────────────────────────

async function loadArtefact(path) {
  const body = document.getElementById('reportBody');
  const toc  = document.getElementById('reportToc');
  if (!body) return;
  body.innerHTML = '<div class="loading">Chargement…</div>';
  if (toc) toc.innerHTML = '';

  try {
    const res  = await fetch(`/api/artefact?path=${encodeURIComponent(path)}`);
    if (!res.ok) { body.innerHTML = `<p class="empty-state">Erreur ${res.status}</p>`; return; }
    const data = await res.json();

    // Rendre le Markdown
    const rawHtml = marked.parse(data.content || '');
    const clean   = DOMPurify.sanitize(rawHtml, { ADD_ATTR: ['class'] });
    body.innerHTML = clean;

    // Convertir les blocs mermaid
    body.querySelectorAll('code.language-mermaid').forEach(block => {
      const pre = block.parentElement;
      const div = document.createElement('div');
      div.className = 'mermaid';
      div.textContent = block.textContent;
      pre.replaceWith(div);
    });
    if (typeof mermaid !== 'undefined') {
      const nodes = body.querySelectorAll('.mermaid');
      if (nodes.length) await mermaid.run({ nodes });
    }

    // Générer la TOC depuis les H2
    const h2s = body.querySelectorAll('h2');
    if (toc && h2s.length > 1) {
      let tocHtml = '<nav class="toc"><ul>';
      h2s.forEach((h, i) => {
        const id = `h2-${i}`;
        h.id = id;
        tocHtml += `<li><a href="#${id}">${h.textContent}</a></li>`;
      });
      tocHtml += '</ul></nav>';
      toc.innerHTML = tocHtml;
    }

    // Marquer l'item actif dans l'arbre
    document.querySelectorAll('.tree-item').forEach(b => {
      b.classList.toggle('active', b.dataset.path === path);
    });
  } catch (e) {
    body.innerHTML = `<p class="empty-state">Erreur : ${e.message}</p>`;
  }
}

// ── E05 — Progression ─────────────────────────────────────

async function loadProgress(slug) {
  const panel = document.getElementById('progressPanel');
  const bars  = document.getElementById('progressBars');
  if (!panel || !bars) return;

  try {
    const res  = await fetch(`/api/progress/${encodeURIComponent(slug)}`);
    const data = await res.json();
    const phases = data.phases || [];
    if (!phases.length || phases.every(p => p.total === 0)) {
      panel.style.display = 'none';
      return;
    }
    bars.innerHTML = phases.map(p => {
      const pct = p.total ? Math.round((p.done / p.total) * 100) : 0;
      return `
        <div class="progress-item">
          <div class="progress-label"><span>${p.phase}</span><span>${p.done}/${p.total}</span></div>
          <div class="progress-bar"><div class="progress-fill" style="width:${pct}%"></div></div>
        </div>`;
    }).join('');
    panel.style.display = 'block';
  } catch {
    panel.style.display = 'none';
  }
}

// ── Prompt Builder ────────────────────────────────────────

document.getElementById('promptForm')?.addEventListener('submit', async (e) => {
  e.preventDefault();
  const input = {
    title:       document.getElementById('pb-title')?.value       || '',
    task_class:  document.getElementById('pb-class')?.value       || 'T5',
    domain:      document.getElementById('pb-domain')?.value      || 'fullstack',
    description: document.getElementById('pb-desc')?.value        || '',
    constraints: document.getElementById('pb-constraints')?.value || null,
    stack:       document.getElementById('pb-stack')?.value       || null,
    tags: [],
  };
  try {
    const res  = await fetch('/api/prompt', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(input),
    });
    const data = await res.json();
    const output   = document.getElementById('promptOutput');
    const textarea = document.getElementById('promptText');
    if (output && textarea) {
      textarea.value = data.prompt || '';
      output.style.display = 'flex';
      output.style.flexDirection = 'column';
    }
  } catch (e) {
    console.error('Erreur génération prompt', e);
  }
});

document.getElementById('copyPrompt')?.addEventListener('click', () => {
  const textarea = document.getElementById('promptText');
  if (textarea?.value) {
    navigator.clipboard.writeText(textarea.value).catch(() => {
      textarea.select();
      document.execCommand('copy');
    });
  }
});

document.getElementById('initSequence')?.addEventListener('click', async () => {
  const titleInput = document.getElementById('pb-title');
  const rawTitle   = titleInput?.value?.trim() || '';
  const slug       = rawTitle.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '');
  const complexity = document.getElementById('pb-complexity')?.value || 'C5';
  const statusEl   = document.getElementById('initStatus');

  if (!slug) {
    if (statusEl) { statusEl.textContent = 'Remplis le titre avant d\'initialiser.'; statusEl.style.display = 'block'; statusEl.className = 'init-status error'; }
    return;
  }

  if (statusEl) { statusEl.textContent = 'Initialisation…'; statusEl.style.display = 'block'; statusEl.className = 'init-status'; }

  try {
    const today = new Date().toISOString().slice(0, 10);
    const res   = await fetch('/api/init-sequence', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ slug, complexity, date: today }),
    });
    const data = await res.json();
    if (data.ok) {
      if (statusEl) { statusEl.textContent = `✓ ${data.path}`; statusEl.className = 'init-status success'; }
      loadSequences();
    } else {
      if (statusEl) { statusEl.textContent = `Erreur : ${data.error || 'inconnue'}`; statusEl.className = 'init-status error'; }
    }
  } catch (e) {
    if (statusEl) { statusEl.textContent = `Erreur : ${e.message}`; statusEl.className = 'init-status error'; }
  }
});

// ── Init ──────────────────────────────────────────────────

if (typeof mermaid !== 'undefined') {
  mermaid.initialize({ startOnLoad: false, theme: 'dark' });
}

loadSequences();
