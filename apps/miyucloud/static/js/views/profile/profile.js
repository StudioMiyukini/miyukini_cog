const API = '/api';

/* ── i18n helper — falls back to key if i18n not ready ── */
/*
 * @id     cog.cloudViewProfile
 * @do     gerer_profil_utilisateur
 * @role   securite
 * @layer  ui
 * @human  Vue du profil : change le mot de passe et gère les mots de passe d'application (création, révocation).
 */
/*
 * @id     cog.cloudViewProfile.t
 * @do     traduire_cle_profil
 * @role   ui
 * @layer  ui
 * @human  Raccourci de traduction local à la vue profil.
 */
function t(key, params) {
  if (window.i18n && typeof window.i18n.t === 'function') return window.i18n.t(key, params);
  return key.split('.').pop().replace(/_/g, ' ');
}

function headers() {
  return { 'Content-Type': 'application/json', ...getCsrfHeaders() };
}

function formatBytes(bytes) {
  if (bytes === 0) return '0 B';
  const k = 1024, sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
  const i = Math.floor(Math.log(bytes) / Math.log(k));
  return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
}

function timeAgo(dateStr) {
  if (!dateStr) return t('profile.never');
  const d = new Date(dateStr);
  const now = new Date();
  const secs = Math.floor((now - d) / 1000);
  if (secs < 60) return t('profile.just_now');
  if (secs < 3600) return t('profile.minutes_ago', { n: Math.floor(secs/60) });
  if (secs < 86400) return t('profile.hours_ago', { n: Math.floor(secs/3600) });
  if (secs < 2592000) return t('profile.days_ago', { n: Math.floor(secs/86400) });
  return d.toLocaleDateString();
}

/*
 * @id     cog.cloudViewProfile.init
 * @do     initialiser_vue_profil
 * @role   orchestration
 * @layer  ui
 * @human  Initialise la vue profil : charge les infos du compte et les mots de passe d'application.
 */
async function init() {
  try {
    const resp = await fetch(API + '/auth/me', { headers: headers(), credentials: 'same-origin' });
    if (!resp.ok) { showError(); return; }
    const user = await resp.json();

    const initials = (user.username || '?').substring(0, 2).toUpperCase();
    document.getElementById('p-avatar').textContent = initials;
    document.getElementById('p-username').textContent = user.username;
    document.getElementById('p-email').textContent = user.email || '';

    const badge = document.getElementById('p-role-badge');
    if (user.role === 'admin') {
      badge.className = 'role-badge role-badge-admin';
      badge.innerHTML = '<i class="fas fa-shield-alt"></i> ' + t('profile.role_admin');
    } else {
      badge.className = 'role-badge role-badge-user';
      badge.innerHTML = '<i class="fas fa-user"></i> ' + t('profile.role_user');
    }

    document.getElementById('p-detail-username').textContent = user.username;
    document.getElementById('p-detail-email').textContent = user.email || '—';
    document.getElementById('p-detail-role').textContent = user.role === 'admin' ? t('profile.role_admin') : t('profile.role_user');
    document.getElementById('p-detail-login').textContent = timeAgo(user.last_login_at);

    const used = user.storage_used_bytes || 0;
    const quota = user.storage_quota_bytes || 0;
    const pct = quota > 0 ? Math.min(Math.round((used / quota) * 100), 100) : 0;

    document.getElementById('p-storage-used').textContent = formatBytes(used);
    document.getElementById('p-storage-quota').textContent = quota > 0 ? formatBytes(quota) : '∞';
    document.getElementById('p-storage-pct').textContent = quota > 0 ? pct + '%' : '—';

    const bar = document.getElementById('p-storage-bar');
    bar.style.width = pct + '%';
    bar.className = 'storage-fill ' + (pct > 90 ? 'red' : pct > 70 ? 'orange' : 'green');
    document.getElementById('p-storage-text').textContent = formatBytes(used) + ' / ' + (quota > 0 ? formatBytes(quota) : t('profile.unlimited'));

    if (user.auth_provider && user.auth_provider !== 'local') {
      document.getElementById('password-section').classList.add('hidden');
    }

    loadAppPasswords();
    loadMiyuDevices();
    loadMiyuIdentities();

    try {
      const oidcResp = await fetch(API + '/auth/oidc/providers', { credentials: 'same-origin' });
      if (oidcResp.ok) {
        const oidcInfo = await oidcResp.json();
        if (!oidcInfo.password_login_enabled) {
          document.getElementById('password-section').classList.add('hidden');
        }
      }
    } catch (oidcErr) {
    }

    document.getElementById('loading').style.display = 'none';
    document.getElementById('main-content').style.display = 'block';
  } catch (e) {
    console.error(e);
    showError();
  }
}

function showError() {
  document.getElementById('loading').style.display = 'none';
  document.getElementById('auth-error').style.display = 'block';
}

/*
 * @id     cog.cloudViewProfile.changePassword
 * @do     changer_mot_de_passe
 * @role   securite
 * @layer  ui
 * @human  Change le mot de passe du compte après vérification de l'ancien.
 */
async function changePassword(e) {
  e.preventDefault();
  const currentPw = document.getElementById('current-password').value;
  const newPw = document.getElementById('new-password').value;
  const confirmPw = document.getElementById('confirm-password').value;
  const statusEl = document.getElementById('pw-status');

  if (newPw !== confirmPw) {
    statusEl.innerHTML = '<div class="alert alert-error"><i class="fas fa-exclamation-circle"></i> ' + escapeHtml(t('profile.passwords_no_match')) + '</div>';
    return false;
  }

  if (newPw.length < 8) {
    statusEl.innerHTML = '<div class="alert alert-error"><i class="fas fa-exclamation-circle"></i> ' + escapeHtml(t('profile.password_too_short')) + '</div>';
    return false;
  }

  const btn = document.getElementById('pw-submit');
  btn.disabled = true;
  btn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> ' + escapeHtml(t('profile.updating'));

  try {
    const resp = await fetch(API + '/auth/change-password', {
      method: 'PUT',
      headers: headers(),
      credentials: 'same-origin',
      body: JSON.stringify({ current_password: currentPw, new_password: newPw })
    });

    if (resp.ok) {
      statusEl.innerHTML = '<div class="alert alert-success"><i class="fas fa-check-circle"></i> ' + escapeHtml(t('profile.password_updated')) + '</div>';
      document.getElementById('password-form').reset();
    } else {
      const err = await resp.json().catch(() => ({}));
      statusEl.innerHTML = '<div class="alert alert-error"><i class="fas fa-exclamation-circle"></i> ' + escapeHtml(err.message || t('profile.password_change_failed')) + '</div>';
    }
  } catch (err) {
    statusEl.innerHTML = '<div class="alert alert-error"><i class="fas fa-exclamation-circle"></i> ' + escapeHtml(t('profile.error_network', { message: err.message })) + '</div>';
  }

  btn.disabled = false;
  btn.innerHTML = '<i class="fas fa-save"></i> ' + escapeHtml(t('profile.update_password'));
  return false;
}

// ── App Passwords ──

const AUTO_LABELS = ['Nextcloud', 'Nextcloud (OIDC)'];

function isAutoPassword(pw) {
  return AUTO_LABELS.includes(pw.label);
}

function renderPwRow(pw) {
  const tr = document.createElement('tr');
  const label = document.createElement('td');
  label.textContent = pw.label;
  const created = document.createElement('td');
  created.textContent = new Date(pw.created_at).toLocaleDateString();
  const lastUsed = document.createElement('td');
  lastUsed.textContent = pw.last_used_at ? timeAgo(pw.last_used_at) : t('profile.never');
  const status = document.createElement('td');
  const badge = document.createElement('span');
  if (pw.active !== false) {
    badge.className = 'badge badge-active';
    badge.textContent = t('profile.active');
  } else {
    badge.className = 'badge badge-expired';
    badge.textContent = t('profile.revoked');
  }
  status.appendChild(badge);
  const actions = document.createElement('td');
  if (pw.active !== false) {
    const btn = document.createElement('button');
    btn.className = 'btn btn-danger-sm';
    btn.innerHTML = '<i class="fas fa-trash"></i>';
    btn.title = t('profile.revoke_title');
    btn.addEventListener('click', function () { revokeAppPassword(pw.id, pw.label); });
    actions.appendChild(btn);
  }
  tr.append(label, created, lastUsed, status, actions);
  return tr;
}

/*
 * @id     cog.cloudViewProfile.loadAppPasswords
 * @do     charger_mots_de_passe_app
 * @role   securite
 * @layer  ui
 * @human  Charge la liste des mots de passe d'application du compte.
 */
async function loadAppPasswords() {
  try {
    const resp = await fetch(API + '/auth/app-passwords', { headers: headers(), credentials: 'same-origin' });
    if (!resp.ok) {
      document.getElementById('app-passwords-section').classList.add('hidden');
      return;
    }
    const data = await resp.json();
    const passwords = data.app_passwords || data;
    const userPws = passwords.filter(function (pw) { return !isAutoPassword(pw); });
    const autoPws = passwords.filter(isAutoPassword);

    // User-created passwords
    const tbody = document.getElementById('app-pw-tbody');
    const table = document.getElementById('app-pw-table');
    const empty = document.getElementById('app-pw-empty');
    tbody.innerHTML = '';
    if (userPws.length === 0) {
      table.classList.add('hidden');
      empty.classList.remove('hidden');
    } else {
      table.classList.remove('hidden');
      empty.classList.add('hidden');
      for (const pw of userPws) tbody.appendChild(renderPwRow(pw));
    }

    // Auto-generated (client session) passwords
    const autoSection = document.getElementById('app-pw-auto-section');
    if (autoPws.length === 0) {
      autoSection.classList.add('hidden');
    } else {
      autoSection.classList.remove('hidden');
      document.getElementById('app-pw-auto-count').textContent = autoPws.length;
      const autoTbody = document.getElementById('app-pw-auto-tbody');
      autoTbody.innerHTML = '';
      for (const pw of autoPws) autoTbody.appendChild(renderPwRow(pw));
    }
  } catch (e) {
    console.error('Failed to load app passwords', e);
  }
}

function toggleAutoPasswords() {
  const body = document.getElementById('app-pw-auto-body');
  const chevron = document.getElementById('app-pw-auto-chevron');
  const isHidden = body.classList.contains('hidden');
  body.classList.toggle('hidden', !isHidden);
  chevron.className = isHidden ? 'fas fa-chevron-down' : 'fas fa-chevron-right';
}

/*
 * @id     cog.cloudViewProfile.createAppPassword
 * @do     creer_mot_de_passe_app
 * @role   securite
 * @layer  ui
 * @human  Crée un nouveau mot de passe d'application (pour un client tiers).
 */
async function createAppPassword() {
  const labelInput = document.getElementById('app-pw-label');
  const label = labelInput.value.trim();
  const statusEl = document.getElementById('app-pw-status');
  const btn = document.getElementById('app-pw-generate');

  if (!label) {
    statusEl.innerHTML = '<div class="alert alert-error"><i class="fas fa-exclamation-circle"></i> ' + escapeHtml(t('profile.error_label_required')) + '</div>';
    return;
  }

  btn.disabled = true;
  btn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> ' + escapeHtml(t('profile.generating'));
  statusEl.innerHTML = '';

  try {
    const resp = await fetch(API + '/auth/app-passwords', {
      method: 'POST',
      headers: headers(),
      credentials: 'same-origin',
      body: JSON.stringify({ label: label })
    });
    if (!resp.ok) {
      const err = await resp.json().catch(() => ({}));
      statusEl.innerHTML = '<div class="alert alert-error"><i class="fas fa-exclamation-circle"></i> ' + escapeHtml(err.message || t('profile.error_create_pw')) + '</div>';
      return;
    }
    const result = await resp.json();
    document.getElementById('app-pw-created-label').textContent = result.label;
    document.getElementById('app-pw-created-password').textContent = result.password;
    document.getElementById('app-pw-created').classList.remove('hidden');
    labelInput.value = '';
    loadAppPasswords();
  } catch (err) {
    statusEl.innerHTML = '<div class="alert alert-error"><i class="fas fa-exclamation-circle"></i> ' + err.message + '</div>';
  } finally {
    btn.disabled = false;
    btn.innerHTML = '<i class="fas fa-plus"></i> ' + escapeHtml(t('profile.generate'));
  }
}

function copyAppPassword() {
  const pw = document.getElementById('app-pw-created-password').textContent;
  navigator.clipboard.writeText(pw).then(function () {
    const btn = document.getElementById('app-pw-copy-btn');
    btn.innerHTML = '<i class="fas fa-check"></i>';
    setTimeout(function () { btn.innerHTML = '<i class="fas fa-copy"></i>'; }, 1500);
  });
}

/*
 * @id     cog.cloudViewProfile.revokeAppPassword
 * @do     revoquer_mot_de_passe_app
 * @role   securite
 * @layer  ui
 * @human  Révoque un mot de passe d'application existant.
 */
async function revokeAppPassword(id, label) {
  if (!confirm(t('profile.confirm_revoke', { label: label }))) return;
  try {
    const resp = await fetch(API + '/auth/app-passwords/' + encodeURIComponent(id), {
      method: 'DELETE',
      headers: headers(),
      credentials: 'same-origin'
    });
    if (resp.ok || resp.status === 204) {
      document.getElementById('app-pw-created').classList.add('hidden');
      loadAppPasswords();
    } else {
      const err = await resp.json().catch(() => ({}));
      alert(err.message || t('profile.error_revoke'));
    }
  } catch (err) {
    alert(t('profile.error_network', { message: err.message }));
  }
}

// ── Appareils et identités Miyukini (I6) ──

function statusAlert(el, ok, message) {
  el.innerHTML = '<div class="alert ' + (ok ? 'alert-success' : 'alert-error') + '"><i class="fas ' +
    (ok ? 'fa-check-circle' : 'fa-exclamation-circle') + '"></i> ' + escapeHtml(message) + '</div>';
}

async function errorMessage(resp, fallbackKey) {
  const err = await resp.json().catch(() => ({}));
  return err.message || err.error || t(fallbackKey);
}

/** Empreinte courte d'une clé publique (base64 URL) pour l'affichage. */
function shortKey(key) {
  return (key || '').length > 12 ? key.substring(0, 6) + '…' + key.substring(key.length - 6) : (key || '');
}

function cell(text) {
  const td = document.createElement('td');
  td.textContent = text;
  return td;
}

function removeButton(titleKey, handler) {
  const td = document.createElement('td');
  const btn = document.createElement('button');
  btn.className = 'btn btn-danger-sm';
  btn.innerHTML = '<i class="fas fa-trash"></i>';
  btn.title = t(titleKey);
  btn.addEventListener('click', handler);
  td.appendChild(btn);
  return td;
}

/*
 * @id     cog.cloudViewProfile.loadMiyuDevices
 * @do     charger_appareils_lies
 * @role   securite
 * @layer  ui
 * @human  Liste les clés d'appareil liées au compte (nom, empreinte, portées, usage).
 */
async function loadMiyuDevices() {
  const section = document.getElementById('miyu-devices-section');
  try {
    const resp = await fetch(API + '/auth/miyu/appareils', { headers: headers(), credentials: 'same-origin' });
    if (!resp.ok) { section.classList.add('hidden'); return; }
    section.classList.remove('hidden');
    const devices = await resp.json();
    const tbody = document.getElementById('miyu-dev-tbody');
    tbody.innerHTML = '';
    document.getElementById('miyu-dev-table').classList.toggle('hidden', devices.length === 0);
    document.getElementById('miyu-dev-empty').classList.toggle('hidden', devices.length !== 0);
    for (const d of devices) {
      const tr = document.createElement('tr');
      const key = cell(shortKey(d.public_key));
      key.title = d.public_key;
      tr.append(
        cell(d.client_name), key, cell(d.scopes),
        cell(new Date(d.created_at).toLocaleDateString()),
        cell(d.last_used_at ? timeAgo(d.last_used_at) : t('profile.never')),
        removeButton('profile.miyu_remove_device', function () { removeMiyuDevice(d.id, d.client_name); })
      );
      tbody.appendChild(tr);
    }
  } catch (e) {
    console.error('Failed to load linked devices', e);
  }
}

/*
 * @id     cog.cloudViewProfile.linkMiyuDevice
 * @do     lier_cle_appareil
 * @role   securite
 * @layer  ui
 * @human  Lie une nouvelle clé d'appareil à partir de sa demande de liaison signée (MLA2).
 */
async function linkMiyuDevice(e) {
  e.preventDefault();
  const statusEl = document.getElementById('miyu-dev-status');
  const scopes = Array.from(document.querySelectorAll('#miyu-dev-scopes input:checked')).map(function (c) { return c.value; });
  if (scopes.length === 0) { statusAlert(statusEl, false, t('profile.miyu_scope_required')); return; }
  const btn = document.getElementById('miyu-dev-submit');
  btn.disabled = true;
  try {
    const resp = await fetch(API + '/auth/miyu/appareils', {
      method: 'POST',
      headers: headers(),
      credentials: 'same-origin',
      body: JSON.stringify({
        demande: document.getElementById('miyu-dev-request').value.trim(),
        nom: document.getElementById('miyu-dev-name').value.trim(),
        portees: scopes.join(',')
      })
    });
    if (resp.status === 201) {
      statusAlert(statusEl, true, t('profile.miyu_device_linked'));
      document.getElementById('miyu-dev-form').reset();
      loadMiyuDevices();
    } else {
      statusAlert(statusEl, false, await errorMessage(resp, 'profile.miyu_link_failed'));
    }
  } catch (err) {
    statusAlert(statusEl, false, t('profile.error_network', { message: err.message }));
  } finally {
    btn.disabled = false;
  }
}

/*
 * @id     cog.cloudViewProfile.removeMiyuDevice
 * @do     retirer_cle_appareil
 * @role   securite
 * @layer  ui
 * @human  Retire une clé d'appareil : ses requêtes signées et ses visas sont refusés aussitôt.
 */
async function removeMiyuDevice(id, name) {
  if (!confirm(t('profile.miyu_confirm_remove_device', { label: name }))) return;
  const statusEl = document.getElementById('miyu-dev-status');
  try {
    const resp = await fetch(API + '/auth/miyu/appareils/' + encodeURIComponent(id), {
      method: 'DELETE', headers: headers(), credentials: 'same-origin'
    });
    if (resp.status === 204) {
      statusAlert(statusEl, true, t('profile.miyu_device_removed'));
      loadMiyuDevices();
    } else {
      statusAlert(statusEl, false, await errorMessage(resp, 'profile.error_revoke'));
    }
  } catch (err) {
    statusAlert(statusEl, false, t('profile.error_network', { message: err.message }));
  }
}

/*
 * @id     cog.cloudViewProfile.loadMiyuIdentities
 * @do     charger_identites_liees
 * @role   securite
 * @layer  ui
 * @human  Liste les identités Miyukini liées au compte.
 */
async function loadMiyuIdentities() {
  const section = document.getElementById('miyu-identities-section');
  try {
    const resp = await fetch(API + '/auth/miyu/liaisons', { headers: headers(), credentials: 'same-origin' });
    if (!resp.ok) { section.classList.add('hidden'); return; }
    section.classList.remove('hidden');
    const ids = await resp.json();
    const tbody = document.getElementById('miyu-id-tbody');
    tbody.innerHTML = '';
    document.getElementById('miyu-id-table').classList.toggle('hidden', ids.length === 0);
    document.getElementById('miyu-id-empty').classList.toggle('hidden', ids.length !== 0);
    for (const i of ids) {
      const tr = document.createElement('tr');
      tr.append(
        cell(i.label), cell(i.cog_id),
        cell(new Date(i.created_at).toLocaleDateString()),
        cell(i.last_used_at ? timeAgo(i.last_used_at) : t('profile.never')),
        removeButton('profile.miyu_unlink_identity', function () { unlinkMiyuIdentity(i.id, i.label); })
      );
      tbody.appendChild(tr);
    }
  } catch (e) {
    console.error('Failed to load linked identities', e);
  }
}

/*
 * @id     cog.cloudViewProfile.linkMiyuIdentity
 * @do     lier_identite_miyukini
 * @role   securite
 * @layer  ui
 * @human  Lie une identité Miyukini à partir d'une preuve collée (visa, preuve de possession, délégations).
 */
async function linkMiyuIdentity(e) {
  e.preventDefault();
  const statusEl = document.getElementById('miyu-id-status');
  let proof;
  try {
    proof = JSON.parse(document.getElementById('miyu-id-proof').value);
  } catch (_) {
    statusAlert(statusEl, false, t('profile.miyu_proof_invalid'));
    return;
  }
  if (!proof || typeof proof.visa !== 'string' || typeof proof.pop !== 'string') {
    statusAlert(statusEl, false, t('profile.miyu_proof_invalid'));
    return;
  }
  const h = headers();
  delete h['Content-Type'];
  h['X-Miyu-Visa'] = proof.visa;
  h['X-Miyu-PoP'] = proof.pop;
  if (typeof proof.delegations === 'string' && proof.delegations) h['X-Miyu-Delegations'] = proof.delegations;
  const label = document.getElementById('miyu-id-label').value.trim();
  // Corps vide : la preuve de l'appareil est liée à « POST /api/auth/miyu/liaisons » sans corps.
  const url = API + '/auth/miyu/liaisons' + (label ? '?label=' + encodeURIComponent(label) : '');
  const btn = document.getElementById('miyu-id-submit');
  btn.disabled = true;
  try {
    const resp = await fetch(url, { method: 'POST', headers: h, credentials: 'same-origin' });
    if (resp.status === 201) {
      statusAlert(statusEl, true, t('profile.miyu_identity_linked'));
      document.getElementById('miyu-id-form').reset();
      loadMiyuIdentities();
    } else {
      statusAlert(statusEl, false, await errorMessage(resp, 'profile.miyu_link_failed'));
    }
  } catch (err) {
    statusAlert(statusEl, false, t('profile.error_network', { message: err.message }));
  } finally {
    btn.disabled = false;
  }
}

/*
 * @id     cog.cloudViewProfile.unlinkMiyuIdentity
 * @do     delier_identite_miyukini
 * @role   securite
 * @layer  ui
 * @human  Retire la liaison d'une identité Miyukini (effet immédiat).
 */
async function unlinkMiyuIdentity(id, label) {
  if (!confirm(t('profile.miyu_confirm_unlink', { label: label }))) return;
  const statusEl = document.getElementById('miyu-id-status');
  try {
    const resp = await fetch(API + '/auth/miyu/liaisons/' + encodeURIComponent(id), {
      method: 'DELETE', headers: headers(), credentials: 'same-origin'
    });
    if (resp.status === 204) {
      loadMiyuIdentities();
    } else {
      statusAlert(statusEl, false, await errorMessage(resp, 'profile.error_revoke'));
    }
  } catch (err) {
    statusAlert(statusEl, false, t('profile.error_network', { message: err.message }));
  }
}

function escapeHtml(str) {
  var div = document.createElement('div');
  div.textContent = str || '';
  return div.innerHTML;
}

init();

/* Wire up event handlers (replaces inline onclick/onsubmit) */
document.getElementById('password-form').addEventListener('submit', changePassword);
document.getElementById('app-pw-generate').addEventListener('click', createAppPassword);
document.getElementById('app-pw-copy-btn').addEventListener('click', copyAppPassword);
document.getElementById('app-pw-auto-toggle').addEventListener('click', toggleAutoPasswords);
document.getElementById('miyu-dev-form').addEventListener('submit', linkMiyuDevice);
document.getElementById('miyu-id-form').addEventListener('submit', linkMiyuIdentity);

/* Re-render when language changes */
window.addEventListener('translationsLoaded', function () { init(); });
window.addEventListener('localeChanged', function () { init(); });
