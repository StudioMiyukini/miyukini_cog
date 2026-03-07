// MIPOWER — app.js v0.4.0

// ── État global ───────────────────────────────────────────

let allSequences    = [];
let currentSlug     = null;
let currentFiles    = [];   // [{path, done}] — artefacts de la séquence ouverte
let currentFlatFiles = [];  // liste plate pour nav prev/next
let currentFileIndex = -1;
let currentTab       = null;          // onglet actif dans le rapport
let tabGroupsMap     = {};            // { groupKey: [{full, done, label}] }

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
    renderProgressPills(slug);
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

const CLASS_ORDER = ['T1', 'T2', 'T3', 'T4', 'T5'];
const STATUS_ORDER = ['active', 'done', 'archived'];

function sortSequences(list) {
  const sortBy = document.getElementById('sortBy')?.value || 'date-desc';
  return [...list].sort((a, b) => {
    switch (sortBy) {
      case 'date-asc':   return a.date.localeCompare(b.date);
      case 'date-desc':  return b.date.localeCompare(a.date);
      case 'name-asc':   return a.name.localeCompare(b.name);
      case 'class-asc': {
        const ia = CLASS_ORDER.indexOf(a.task_class || '');
        const ib = CLASS_ORDER.indexOf(b.task_class || '');
        return (ia === -1 ? 99 : ia) - (ib === -1 ? 99 : ib);
      }
      case 'status': {
        const ia = STATUS_ORDER.indexOf(a.status || 'active');
        const ib = STATUS_ORDER.indexOf(b.status || 'active');
        return ia - ib;
      }
      default: return b.date.localeCompare(a.date);
    }
  });
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
  grid.innerHTML = sortSequences(filtered).map(s => sequenceCardHTML(s)).join('');
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

['searchInput', 'statusFilter', 'sortBy'].forEach(id => {
  document.getElementById(id)?.addEventListener(id === 'searchInput' ? 'input' : 'change', () => {
    renderSequences(document.getElementById('sequenceGrid'));
  });
});

// ── Rapport — ouverture séquence ──────────────────────────

async function openSequence(slug) {
  currentSlug = slug;
  switchView('report');

  const tree    = document.getElementById('artefactTree');
  const tabsBar = document.getElementById('reportTabs');
  const title   = document.getElementById('reportTitle');
  if (tree)    tree.innerHTML    = '<div class="loading">Chargement…</div>';
  if (tabsBar) tabsBar.innerHTML = '';
  if (title)   title.textContent = slug;
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

  renderProgressPills(slug);
}

// ── Labels lisibles des onglets ──────────────────────────

const TAB_LABELS = {
  briefs:   'Brief',
  specs:    'Specs',
  plans_p3: 'P3',
  agents:   'Agents',
  audits:   'P4',
  phases:   'Phases',
  _root:    'Racine',
};

const TAB_ORDER = ['briefs', 'specs', 'plans_p3', 'agents', 'audits', 'phases', '_root'];

function renderArtefactTree(tree, files, seqRelPath) {
  if (!tree) return;
  const tabsBar = document.getElementById('reportTabs');

  if (!files.length) {
    tree.innerHTML = '<p class="empty-state">Aucun artefact .md trouvé.</p>';
    if (tabsBar) tabsBar.innerHTML = '';
    return;
  }

  // Normalise {path, done} ou string (compat)
  const normalised = files.map(f => typeof f === 'string' ? { path: f, done: false } : f);
  currentFlatFiles = normalised;
  currentFileIndex = -1;
  updateNavButtons();

  // Grouper par répertoire de premier niveau relatif à la séquence
  const groups = {};
  for (const f of normalised) {
    const afterSeq = seqRelPath ? f.path.replace(seqRelPath + '/', '') : f.path;
    const parts    = afterSeq.split('/');
    const group    = parts.length > 1 ? parts[0] : '_root';
    if (!groups[group]) groups[group] = [];
    groups[group].push({ full: f.path, done: f.done, label: parts[parts.length - 1] });
  }
  tabGroupsMap = groups;

  const keys = [...new Set([
    ...TAB_ORDER.filter(k => groups[k]),
    ...Object.keys(groups).filter(k => !TAB_ORDER.includes(k)),
  ])];

  // Rendre la barre d'onglets
  if (tabsBar) {
    tabsBar.innerHTML = keys.map(k => {
      const total  = groups[k].length;
      const done   = groups[k].filter(f => f.done).length;
      const allDone = done === total && total > 0;
      const cls    = allDone ? ' tab-done' : '';
      const label  = TAB_LABELS[k] || k;
      return `<button class="report-tab${cls}" data-tab="${k}">${label}<span class="tab-badge">${done}/${total}</span></button>`;
    }).join('');

    currentTab = keys[0] || null;
    tabsBar.querySelector('.report-tab')?.classList.add('active');

    tabsBar.querySelectorAll('.report-tab').forEach(btn => {
      btn.addEventListener('click', () => {
        tabsBar.querySelectorAll('.report-tab').forEach(b => b.classList.remove('active'));
        btn.classList.add('active');
        currentTab = btn.dataset.tab;
        renderTabPanel(tree, groups, currentTab);
      });
    });
  }

  // Afficher le premier onglet
  renderTabPanel(tree, groups, currentTab);

  // Auto-charge le brief si disponible
  const brief = normalised.find(f => f.path.includes('/briefs/'));
  if (brief) loadArtefact(brief.path);
}

function renderTabPanel(panel, groups, tabKey) {
  if (!panel) return;
  if (!tabKey || !groups[tabKey]) { panel.innerHTML = ''; return; }
  const items = groups[tabKey];
  let html = '<nav class="tree-nav"><ul>';
  for (const item of items) {
    const doneCls = item.done ? ' done' : ' pending';
    html += `<li><button class="tree-item${doneCls}" data-path="${item.full}" title="${item.full}">${item.label}</button></li>`;
  }
  html += '</ul></nav>';
  panel.innerHTML = html;

  panel.querySelectorAll('.tree-item').forEach(btn => {
    btn.addEventListener('click', () => {
      panel.querySelectorAll('.tree-item').forEach(b => b.classList.remove('active'));
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

    // Marquer l'item actif + mettre à jour index nav
    const idx = currentFlatFiles.findIndex(f => (f.path || f) === path);
    if (idx !== -1) { currentFileIndex = idx; }
    updateNavButtons();

    // Auto-switch onglet si nécessaire (nav prev/next cross-tab)
    const tree    = document.getElementById('artefactTree');
    const tabsBar = document.getElementById('reportTabs');
    let targetTab = null;
    for (const [k, items] of Object.entries(tabGroupsMap)) {
      if (items.some(i => i.full === path)) { targetTab = k; break; }
    }
    if (targetTab && targetTab !== currentTab && tree) {
      currentTab = targetTab;
      tabsBar?.querySelectorAll('.report-tab').forEach(b => {
        b.classList.toggle('active', b.dataset.tab === targetTab);
      });
      renderTabPanel(tree, tabGroupsMap, targetTab);
    }

    // Highlight item actif dans le panel courant
    document.querySelectorAll('#artefactTree .tree-item').forEach(b => {
      b.classList.toggle('active', b.dataset.path === path);
    });
  } catch (e) {
    body.innerHTML = `<p class="empty-state">Erreur : ${e.message}</p>`;
  }
}

// ── Progression pills (intégrées dans header rapport) ─────

async function renderProgressPills(slug) {
  const pills = document.getElementById('progressPills');
  if (!pills) return;
  try {
    const res    = await fetch(`/api/progress/${encodeURIComponent(slug)}`);
    const data   = await res.json();
    const phases = (data.phases || []).filter(p => p.total > 0);
    if (!phases.length) { pills.innerHTML = ''; return; }
    pills.innerHTML = phases.map(p => {
      const cls = p.done === p.total ? 'pill-done' : p.done > 0 ? 'pill-partial' : 'pill-pending';
      return `<span class="progress-pill ${cls}" title="${p.phase}: ${p.done}/${p.total}">${p.phase}&nbsp;${p.done}/${p.total}</span>`;
    }).join('');
  } catch {
    pills.innerHTML = '';
  }
}

// ── Navigation prev/next artefacts ───────────────────────

function updateNavButtons() {
  const prev    = document.getElementById('prevArtefact');
  const next    = document.getElementById('nextArtefact');
  const counter = document.getElementById('artefactCounter');
  const n = currentFlatFiles.length;
  if (prev)    prev.disabled    = currentFileIndex <= 0;
  if (next)    next.disabled    = currentFileIndex >= n - 1 || n === 0;
  if (counter) counter.textContent = n > 0 && currentFileIndex >= 0 ? `${currentFileIndex + 1}/${n}` : '';
}

document.getElementById('prevArtefact')?.addEventListener('click', () => {
  if (currentFileIndex > 0) { currentFileIndex--; loadArtefact(currentFlatFiles[currentFileIndex].path); }
});
document.getElementById('nextArtefact')?.addEventListener('click', () => {
  if (currentFileIndex < currentFlatFiles.length - 1) {
    currentFileIndex++;
    loadArtefact(currentFlatFiles[currentFileIndex].path);
  }
});
document.addEventListener('keydown', (e) => {
  if (e.altKey && e.key === 'ArrowLeft')  document.getElementById('prevArtefact')?.click();
  if (e.altKey && e.key === 'ArrowRight') document.getElementById('nextArtefact')?.click();
});

// ── Prompt Builder v2 ─────────────────────────────────────

// État tags (tableau de strings)
let pbTags = [];

// ── Stack preset ──────────────────────────────────────────

document.getElementById('pb-stack-preset')?.addEventListener('change', (e) => {
  const val      = e.target.value;
  const stackEl  = document.getElementById('pb-stack');
  if (!stackEl) return;
  if (val === '__autre__' || val === '') {
    stackEl.value = '';
    stackEl.placeholder = 'Saisie libre…';
  } else {
    stackEl.value = val;
  }
  updatePreview();
});

// ── Tags chips ────────────────────────────────────────────

function renderTags() {
  const list = document.getElementById('pb-tags-list');
  if (!list) return;
  list.innerHTML = pbTags.map((t, i) =>
    `<span class="pb-tag-chip" data-idx="${i}" title="Clic pour supprimer">${t}</span>`
  ).join('');
  list.querySelectorAll('.pb-tag-chip').forEach(chip => {
    chip.addEventListener('click', () => {
      pbTags.splice(Number(chip.dataset.idx), 1);
      renderTags();
      updatePreview();
      saveToLocalStorage();
    });
  });
}

document.getElementById('pb-tag-add')?.addEventListener('click', () => {
  const input = document.getElementById('pb-tag-input');
  if (!input) return;
  const tag = input.value.trim();
  if (tag && pbTags.length < 10 && !pbTags.includes(tag)) {
    pbTags.push(tag);
    input.value = '';
    renderTags();
    updatePreview();
    saveToLocalStorage();
  }
});

document.getElementById('pb-tag-input')?.addEventListener('keydown', (e) => {
  if (e.key === 'Enter') { e.preventDefault(); document.getElementById('pb-tag-add')?.click(); }
});

// ── Lecture état formulaire ────────────────────────────────

function getFormInput() {
  const agents = Array.from(document.querySelectorAll('input[name="pb-agent"]:checked'))
    .map(cb => cb.value);
  return {
    title:         document.getElementById('pb-title')?.value        || '',
    task_class:    document.getElementById('pb-class')?.value        || 'T5',
    domain:        document.getElementById('pb-domain')?.value       || 'fullstack',
    description:   document.getElementById('pb-desc')?.value         || '',
    constraints:   document.getElementById('pb-constraints')?.value  || '',
    stack:         document.getElementById('pb-stack')?.value        || '',
    autonomy_mode: document.getElementById('pb-autonomy')?.value     || '',
    agents,
    tags:          [...pbTags],
    urgency:       document.getElementById('pb-urgency')?.checked    || false,
    sensitive_data: document.getElementById('pb-sensitive')?.checked || false,
    msw_toggle:    document.getElementById('pb-msw')?.checked        || false,
  };
}

// ── Template JS local (miroir du template Rust) ───────────

function buildPromptLocal(input) {
  const stack       = input.stack       || 'A definir en P0';
  const constraints = input.constraints || 'Aucune contrainte specifique';

  const lines = [
    `Lance une sequence MIP pour : ${input.title}`,
    '',
    `Classe estimee : ${input.task_class}`,
    `Domaine : ${input.domain}`,
    `Stack : ${stack}`,
    `Contraintes : ${constraints}`,
  ];

  if (input.autonomy_mode) lines.push(`Mode autonomie : ${input.autonomy_mode}`);
  if (input.urgency)        lines.push('Urgence : Oui');
  if (input.sensitive_data) lines.push('Donnees sensibles : Oui');
  if (input.msw_toggle)     lines.push('Mode Sans Web : Oui');
  if (input.agents.length)  lines.push(`Agents actifs : ${input.agents.join(', ')}`);
  if (input.tags.length)    lines.push(`Tags : ${input.tags.join(', ')}`);

  lines.push('');
  lines.push(`Description :\n${input.description}`);
  lines.push('');
  lines.push('---');
  lines.push('Maria, classe cette demande et lance P0.');

  return lines.join('\n');
}

// ── Preview live (debounce 300ms) ─────────────────────────

let previewTimer = null;

function updatePreview() {
  clearTimeout(previewTimer);
  previewTimer = setTimeout(() => {
    const textarea = document.getElementById('promptText');
    if (!textarea) return;
    textarea.value = buildPromptLocal(getFormInput());
  }, 300);
}

// Attacher updatePreview à tous les champs de base
['pb-title', 'pb-class', 'pb-domain', 'pb-desc', 'pb-constraints', 'pb-stack', 'pb-autonomy'].forEach(id => {
  document.getElementById(id)?.addEventListener('input', updatePreview);
  document.getElementById(id)?.addEventListener('change', updatePreview);
});

// Agents + toggles
document.querySelectorAll('input[name="pb-agent"]').forEach(cb => cb.addEventListener('change', updatePreview));
['pb-urgency', 'pb-sensitive', 'pb-msw'].forEach(id => {
  document.getElementById(id)?.addEventListener('change', updatePreview);
});

// ── Soumission formulaire (appel API serveur) ─────────────

document.getElementById('promptForm')?.addEventListener('submit', async (e) => {
  e.preventDefault();
  const input = getFormInput();
  const body = {
    title:          input.title,
    task_class:     input.task_class,
    domain:         input.domain,
    description:    input.description,
    constraints:    input.constraints || null,
    stack:          input.stack       || null,
    autonomy_mode:  input.autonomy_mode || null,
    agents:         input.agents,
    tags:           input.tags,
    urgency:        input.urgency,
    sensitive_data: input.sensitive_data,
    msw_toggle:     input.msw_toggle,
  };
  try {
    const res  = await fetch('/api/prompt', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
    const data = await res.json();
    const textarea = document.getElementById('promptText');
    if (textarea) textarea.value = data.prompt || '';
  } catch (err) {
    console.error('Erreur génération prompt', err);
  }
});

// ── Copier ────────────────────────────────────────────────

document.getElementById('copyPrompt')?.addEventListener('click', () => {
  const textarea = document.getElementById('promptText');
  if (textarea?.value) {
    navigator.clipboard.writeText(textarea.value).catch(() => {
      textarea.select();
      document.execCommand('copy');
    });
  }
});

// ── Init séquence ─────────────────────────────────────────

document.getElementById('initSequence')?.addEventListener('click', async () => {
  const rawTitle  = document.getElementById('pb-title')?.value?.trim() || '';
  const slug      = rawTitle.toLowerCase().replace(/[^a-z0-9]+/g, '-').replace(/^-|-$/g, '');
  const complexity = document.getElementById('pb-complexity')?.value || 'C5';
  const statusEl  = document.getElementById('initStatus');

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
  } catch (err) {
    if (statusEl) { statusEl.textContent = `Erreur : ${err.message}`; statusEl.className = 'init-status error'; }
  }
});

// Preview initiale au chargement
updatePreview();

// ── localStorage — sauvegarde / restauration ──────────────

const LS_KEYS = {
  title:      'pb_title',
  class:      'pb_class',
  domain:     'pb_domain',
  complexity: 'pb_complexity',
  autonomy:   'pb_autonomy',
  stack:      'pb_stack',
  constraints:'pb_constraints',
  agents:     'pb_agents',   // JSON array
  tags:       'pb_tags',     // JSON array
  urgency:    'pb_urgency',
  sensitive:  'pb_sensitive',
  msw:        'pb_msw',
};

function saveToLocalStorage() {
  const agents = Array.from(document.querySelectorAll('input[name="pb-agent"]:checked')).map(cb => cb.value);
  localStorage.setItem(LS_KEYS.title,       document.getElementById('pb-title')?.value        || '');
  localStorage.setItem(LS_KEYS.class,       document.getElementById('pb-class')?.value        || 'T5');
  localStorage.setItem(LS_KEYS.domain,      document.getElementById('pb-domain')?.value       || 'fullstack');
  localStorage.setItem(LS_KEYS.complexity,  document.getElementById('pb-complexity')?.value   || 'C5');
  localStorage.setItem(LS_KEYS.autonomy,    document.getElementById('pb-autonomy')?.value     || '');
  localStorage.setItem(LS_KEYS.stack,       document.getElementById('pb-stack')?.value        || '');
  localStorage.setItem(LS_KEYS.constraints, document.getElementById('pb-constraints')?.value  || '');
  localStorage.setItem(LS_KEYS.agents,      JSON.stringify(agents));
  localStorage.setItem(LS_KEYS.tags,        JSON.stringify(pbTags));
  localStorage.setItem(LS_KEYS.urgency,     document.getElementById('pb-urgency')?.checked    ? '1' : '0');
  localStorage.setItem(LS_KEYS.sensitive,   document.getElementById('pb-sensitive')?.checked  ? '1' : '0');
  localStorage.setItem(LS_KEYS.msw,         document.getElementById('pb-msw')?.checked        ? '1' : '0');
}

function restoreFromLocalStorage() {
  const set = (id, key) => {
    const el = document.getElementById(id);
    const val = localStorage.getItem(key);
    if (el && val !== null) el.value = val;
  };
  const setCheck = (id, key) => {
    const el = document.getElementById(id);
    if (el) el.checked = localStorage.getItem(key) === '1';
  };

  set('pb-title',       LS_KEYS.title);
  set('pb-class',       LS_KEYS.class);
  set('pb-domain',      LS_KEYS.domain);
  set('pb-complexity',  LS_KEYS.complexity);
  set('pb-autonomy',    LS_KEYS.autonomy);
  set('pb-stack',       LS_KEYS.stack);
  set('pb-constraints', LS_KEYS.constraints);
  setCheck('pb-urgency',  LS_KEYS.urgency);
  setCheck('pb-sensitive', LS_KEYS.sensitive);
  setCheck('pb-msw',      LS_KEYS.msw);

  // Agents
  try {
    const savedAgents = JSON.parse(localStorage.getItem(LS_KEYS.agents) || '[]');
    document.querySelectorAll('input[name="pb-agent"]').forEach(cb => {
      cb.checked = savedAgents.includes(cb.value);
    });
  } catch { /* ignore */ }

  // Tags
  try {
    pbTags = JSON.parse(localStorage.getItem(LS_KEYS.tags) || '[]');
    renderTags();
  } catch { pbTags = []; }
}

// Attacher saveToLocalStorage aux champs
['pb-title', 'pb-class', 'pb-domain', 'pb-complexity', 'pb-autonomy', 'pb-stack', 'pb-constraints'].forEach(id => {
  document.getElementById(id)?.addEventListener('input',  saveToLocalStorage);
  document.getElementById(id)?.addEventListener('change', saveToLocalStorage);
});
document.querySelectorAll('input[name="pb-agent"]').forEach(cb => cb.addEventListener('change', saveToLocalStorage));
['pb-urgency', 'pb-sensitive', 'pb-msw'].forEach(id => {
  document.getElementById(id)?.addEventListener('change', saveToLocalStorage);
});

// Restaurer au chargement
restoreFromLocalStorage();
updatePreview();

// ── Init ──────────────────────────────────────────────────

if (typeof mermaid !== 'undefined') {
  mermaid.initialize({ startOnLoad: false, theme: 'dark' });
}

loadSequences();
