// MIPOWER — app.js (E00 bootstrap — sera remplacé par app.ts compilé en E03)

// ── Navigation ────────────────────────────────────────────

const navItems = document.querySelectorAll('.nav-item');
const views = document.querySelectorAll('.view');

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

// ── Sidebar toggle ────────────────────────────────────────

const sidebar = document.getElementById('sidebar');
const toggle = document.getElementById('sidebarToggle');
toggle?.addEventListener('click', () => {
  sidebar?.classList.toggle('collapsed');
  toggle.textContent = sidebar?.classList.contains('collapsed') ? '›' : '‹';
});

// ── Settings ──────────────────────────────────────────────

const savedRoot = localStorage.getItem('mipRoot') || '';
const mipRootInput = document.getElementById('mipRoot');
if (mipRootInput) mipRootInput.value = savedRoot;

document.getElementById('saveMipRoot')?.addEventListener('click', () => {
  const root = mipRootInput?.value?.trim();
  if (root) {
    localStorage.setItem('mipRoot', root);
    const status = document.getElementById('settingsStatus');
    if (status) { status.textContent = 'Chemin enregistré.'; setTimeout(() => status.textContent = '', 2000); }
    loadSequences();
  }
});

// ── Dashboard — chargement séquences ─────────────────────

async function loadSequences() {
  const grid = document.getElementById('sequenceGrid');
  if (!grid) return;
  grid.innerHTML = '<div class="loading">Chargement…</div>';
  try {
    const res = await fetch('/api/sequences');
    const data = await res.json();
    renderSequences(data.sequences || [], grid);
  } catch (e) {
    grid.innerHTML = `<div class="loading">Erreur : ${e.message}</div>`;
  }
}

function renderSequences(sequences, grid) {
  if (!sequences.length) {
    grid.innerHTML = '<div class="loading">Aucune séquence trouvée. Configure le chemin MIP dans les Paramètres.</div>';
    return;
  }
  const search = document.getElementById('searchInput')?.value?.toLowerCase() || '';
  const status = document.getElementById('statusFilter')?.value || '';
  const filtered = sequences.filter(s =>
    (!search || s.slug.toLowerCase().includes(search)) &&
    (!status || s.status === status)
  );
  grid.innerHTML = filtered.map(s => sequenceCardHTML(s)).join('');
  grid.querySelectorAll('.sequence-card').forEach(card => {
    card.addEventListener('click', () => {
      switchView('report');
      // TODO E03 : charger les artefacts de la séquence
    });
  });
}

function sequenceCardHTML(s) {
  const tClass = (s.task_class || '').toLowerCase();
  const statusClass = s.status || 'active';
  return `
    <div class="sequence-card" data-slug="${s.slug}">
      <div class="card-header">
        <span class="card-slug">${s.slug}</span>
        <span class="card-date">${s.date}</span>
      </div>
      <div class="card-badges">
        ${s.task_class ? `<span class="badge badge-${tClass}">${s.task_class}</span>` : ''}
        ${s.complexity ? `<span class="badge">${s.complexity}</span>` : ''}
        <span class="badge badge-${statusClass}">${statusClass}</span>
      </div>
    </div>`;
}

document.getElementById('searchInput')?.addEventListener('input', () => loadSequences());
document.getElementById('statusFilter')?.addEventListener('change', () => loadSequences());

// ── Prompt Builder ────────────────────────────────────────

document.getElementById('promptForm')?.addEventListener('submit', async (e) => {
  e.preventDefault();
  const input = {
    title: document.getElementById('pb-title')?.value || '',
    task_class: document.getElementById('pb-class')?.value || 'T5',
    domain: document.getElementById('pb-domain')?.value || 'fullstack',
    description: document.getElementById('pb-desc')?.value || '',
    constraints: document.getElementById('pb-constraints')?.value || null,
    stack: document.getElementById('pb-stack')?.value || null,
    tags: [],
  };
  try {
    const res = await fetch('/api/prompt', { method: 'POST', headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(input) });
    const data = await res.json();
    const output = document.getElementById('promptOutput');
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

// ── Init ─────────────────────────────────────────────────

loadSequences();
