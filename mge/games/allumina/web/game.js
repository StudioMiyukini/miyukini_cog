"use strict";
/*
 * Allumina — client web.
 *
 * Se connecte au bridge WebSocket (ws_bridge.rs) servi sur le même hôte/port que
 * cette page (wss://<host>/ via le tunnel Cloudflare), qui relaie en bincode
 * vers le serveur TCP autoritatif. Tous les messages transitent en JSON
 * « internally tagged » : { "type": "Foo", ...champs }.
 *
 * Rendu : Canvas 2D isométrique (diamants). Entités interpolées entre les
 * snapshots 20 Hz, avec prédiction locale du déplacement du joueur.
 */

// ── Constantes de rendu iso ──────────────────────────────────────────────────
const TILE_W = 64;          // largeur d'un losange en px
const TILE_H = 32;          // hauteur d'un losange en px
const HALF_W = TILE_W / 2;
const HALF_H = TILE_H / 2;
const MOVE_SPEED = 4.5;     // tuiles/seconde (cf. Predictor côté Rust)
const SNAP_INTERP = 12.0;   // vitesse de rattrapage de l'interpolation
const GROUND_RADIUS = 22;   // demi-taille de la grille de sol dessinée

// Couleurs par catégorie d'entité.
const KIND_COLOR = {
  Player: "#3a7fe0", Mob: "#d8463c", Boss: "#a01818",
  Summon: "#30c0c0", Mercenary: "#e0902a", Npc: "#9a9a9a",
};
// Body colour per class discriminant (0 Soldat,1 Clerc,2 Mage,3 Archer).
const CLASS_COLOR = ["#5c80a8", "#dcce96", "#8552cc", "#3f8a55"];
const RARITY_COLOR = ["#c8c4b8", "#6f8fff", "#f0c850", "#d88020", "#20c080", "#c04020"];

// ── Petits utilitaires couleur / formes ──────────────────────────────────────
function hexToRgb(h) { const n = parseInt(h.slice(1), 16); return [n >> 16 & 255, n >> 8 & 255, n & 255]; }
function rgbToHex(r, g, b) { return "#" + [r, g, b].map(v => Math.max(0, Math.min(255, v | 0)).toString(16).padStart(2, "0")).join(""); }
function shade(h, f) { const [r, g, b] = hexToRgb(h); return rgbToHex(r * f, g * f, b * f); }
function mix(a, b, t) { const x = hexToRgb(a), y = hexToRgb(b); return rgbToHex(x[0] + (y[0] - x[0]) * t, x[1] + (y[1] - x[1]) * t, x[2] + (y[2] - x[2]) * t); }
function roundRect(c, x, y, w, h, r) {
  if (c.roundRect) { c.beginPath(); c.roundRect(x, y, w, h, r); return; }
  c.beginPath(); c.moveTo(x + r, y); c.arcTo(x + w, y, x + w, y + h, r); c.arcTo(x + w, y + h, x, y + h, r);
  c.arcTo(x, y + h, x, y, r); c.arcTo(x, y, x + w, y, r); c.closePath();
}

// ── État global ──────────────────────────────────────────────────────────────
const G = {
  ws: null,
  phase: "connect",            // connect | lobby | playing
  playerId: randomId(),
  name: "Héros",
  cls: "Soldat",
  selfId: 0,
  entities: new Map(),         // id -> entité interpolée
  ground: new Map(),           // id -> { x, y, name, rarity, kind }
  predict: { x: 0, y: 0, tx: 0, ty: 0, moving: false, init: false },
  cam: { x: 0, y: 0 },
  mouse: { sx: 0, sy: 0, wx: 0, wy: 0 },
  dmgNumbers: [],
  particles: [],
  chat: [],
  gold: 0,
  inventory: [],               // [{idx, name, rarity}]
  hp: 100, hpMax: 100, mp: 50, mpMax: 50, level: 1,
  ping: null, lastPingSent: 0,
  invOpen: false, menuOpen: false, mapBig: false, chatOpen: false,
  lastTime: performance.now(),
};

// ── Helpers DOM ──────────────────────────────────────────────────────────────
const $ = (id) => document.getElementById(id);
const canvas = $("game");
const ctx = canvas.getContext("2d");
const mini = $("minimap");
const mctx = mini.getContext("2d");

function randomId() {
  const h = () => Math.floor(Math.random() * 0x10000).toString(16).padStart(4, "0");
  return `web-${h()}${h()}-${h()}-${h()}`;
}

function resize() {
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;
}
window.addEventListener("resize", resize);
resize();

// ── Projection isométrique ───────────────────────────────────────────────────
function worldToScreen(wx, wy) {
  const isoX = (wx - wy) * HALF_W;
  const isoY = (wx + wy) * HALF_H;
  const camIsoX = (G.cam.x - G.cam.y) * HALF_W;
  const camIsoY = (G.cam.x + G.cam.y) * HALF_H;
  return {
    x: isoX - camIsoX + canvas.width / 2,
    y: isoY - camIsoY + canvas.height / 2,
  };
}
function screenToWorld(sx, sy) {
  const camIsoX = (G.cam.x - G.cam.y) * HALF_W;
  const camIsoY = (G.cam.x + G.cam.y) * HALF_H;
  const isoX = sx - canvas.width / 2 + camIsoX;
  const isoY = sy - canvas.height / 2 + camIsoY;
  const a = isoX / HALF_W;     // = wx - wy
  const b = isoY / HALF_H;     // = wx + wy
  return { x: (a + b) / 2, y: (b - a) / 2 };
}

// ── Réseau ───────────────────────────────────────────────────────────────────
function connect() {
  const raw = $("server").value.trim();
  const proto = location.protocol === "https:" ? "wss" : "ws";
  const url = `${proto}://${raw}`;
  setConnectStatus("Connexion…");
  try {
    G.ws = new WebSocket(url);
  } catch (e) {
    setConnectStatus("URL invalide");
    return;
  }
  G.ws.onopen = () => {
    send({ type: "Login", player_id: G.playerId, character_name: G.name, class: G.cls });
  };
  G.ws.onmessage = (ev) => {
    let msg;
    try { msg = JSON.parse(ev.data); } catch { return; }
    handleServer(msg);
  };
  G.ws.onclose = () => {
    if (G.phase !== "connect") {
      pushChat("⚠ Connexion perdue", true);
      toConnect("Connexion perdue");
    } else {
      setConnectStatus("Connexion fermée");
    }
  };
  G.ws.onerror = () => setConnectStatus("Échec de connexion");
}

function send(obj) {
  if (G.ws && G.ws.readyState === WebSocket.OPEN) G.ws.send(JSON.stringify(obj));
}

function handleServer(m) {
  switch (m.type) {
    case "LoginAccepted":
      G.selfId = m.your_entity_id;
      toLobby();
      send({ type: "ListGames" });
      break;
    case "LoginRejected":
      setConnectStatus("Refusé : " + m.reason);
      break;
    case "GameList":
      renderGameList(m.games || []);
      break;
    case "GameCreated":
      break; // le JoinedGame suit
    case "JoinedGame":
      G.selfId = m.your_entity_id;
      toPlaying();
      break;
    case "JoinFailed":
      $("lobby-status").textContent = m.reason || "Échec";
      break;
    case "ReturnedToLobby":
      toLobby();
      send({ type: "ListGames" });
      break;
    case "GameClosed":
      break;
    case "WorldSnapshot":
      applySnapshot(m);
      break;
    case "EntitySpawned":
      if (m.snapshot) upsertEntity(m.snapshot, true);
      break;
    case "EntityDied":
      onEntityDied(m.entity_id);
      break;
    case "LootDropped":
      if (m.item) {
        G.ground.set(m.item.id, m.item);
        pushChat(`Butin : ${m.item.name}`, true);
      }
      break;
    case "DamageDealt":
      spawnDamage(m.target, m.amount, m.crit);
      break;
    case "SkillUsed":
      if (m.pos) spawnSkillFx(m.pos[0], m.pos[1]);
      break;
    case "InventoryUpdate":
      applyInventory(m.slots || []);
      break;
    case "GoldUpdate":
      G.gold = m.amount; $("gold").textContent = "Or: " + m.amount;
      break;
    case "ChatBroadcast":
      pushChat(`${m.sender}: ${m.text}`);
      break;
    case "PlayerJoined":
      pushChat(`${m.name} (${m.class}) a rejoint`, true);
      break;
    case "PlayerLeft":
      pushChat(`${m.name} est parti`, true);
      break;
    case "WaypointDiscovered":
      pushChat(`${m.player_name} a découvert ${m.wp_name}`, true);
      break;
    case "Pong":
      G.ping = Math.max(0, Date.now() - m.client_timestamp);
      $("ping").textContent = G.ping + " ms";
      break;
    case "ServerShutdown":
      pushChat("Serveur arrêté : " + m.reason, true);
      toConnect("Serveur arrêté");
      break;
    default:
      break;
  }
}

// ── Snapshots & interpolation ────────────────────────────────────────────────
function applySnapshot(m) {
  const seen = new Set();
  for (const e of m.entities || []) {
    upsertEntity(e, false);
    seen.add(e.id);
  }
  // Purge des entités absentes du snapshot.
  for (const id of G.entities.keys()) {
    if (!seen.has(id)) G.entities.delete(id);
  }
  // Items au sol : remplacement complet.
  G.ground.clear();
  for (const gi of m.ground_items || []) G.ground.set(gi.id, gi);

  // Réconciliation de la prédiction locale.
  const self = m.entities && m.entities.find((e) => e.id === G.selfId);
  if (self) {
    if (!G.predict.init) {
      G.predict.x = self.x; G.predict.y = self.y;
      G.predict.tx = self.x; G.predict.ty = self.y;
      G.predict.init = true;
    } else {
      // Blend doux vers la position autoritative.
      G.predict.x += (self.x - G.predict.x) * 0.25;
      G.predict.y += (self.y - G.predict.y) * 0.25;
    }
    G.hp = self.hp; G.hpMax = self.hp_max;
    if (self.level != null) G.level = self.level;
    refreshVitals();
  }
}

function upsertEntity(s, spawn) {
  let e = G.entities.get(s.id);
  if (!e) {
    e = { id: s.id, x: s.x, y: s.y, tx: s.x, ty: s.y };
    G.entities.set(s.id, e);
    if (spawn) spawnSkillFx(s.x, s.y);
  }
  e.tx = s.x; e.ty = s.y;
  e.kind = s.kind; e.name = s.name; e.hp = s.hp; e.hpMax = s.hp_max;
  e.dir = s.direction; e.anim = s.anim_state; e.level = s.level; e.cls = s.class;
  e.weapon = s.weapon; e.offhand = s.offhand;
}

function onEntityDied(id) {
  const e = G.entities.get(id);
  if (e) {
    spawnDeath(e.x, e.y, KIND_COLOR[e.kind] || "#d8463c");
    G.entities.delete(id);
  }
}

// ── Effets visuels ───────────────────────────────────────────────────────────
function spawnDamage(targetId, amount, crit) {
  const e = G.entities.get(targetId);
  const x = e ? e.x : G.predict.x;
  const y = e ? e.y : G.predict.y;
  G.dmgNumbers.push({
    x, y, age: 0, life: 1.1,
    text: crit ? `${Math.round(amount)}!` : `${Math.round(amount)}`,
    color: crit ? "#ffd24a" : "#ff8a6a",
    size: crit ? 22 : 15,
  });
}
function spawnDeath(x, y, color) {
  for (let i = 0; i < 18; i++) {
    const a = Math.random() * Math.PI * 2, sp = 1 + Math.random() * 3;
    G.particles.push({ x, y, vx: Math.cos(a) * sp, vy: Math.sin(a) * sp, age: 0, life: 0.7, color, size: 3 });
  }
}
function spawnSkillFx(x, y) {
  for (let i = 0; i < 10; i++) {
    const a = Math.random() * Math.PI * 2, sp = 0.5 + Math.random() * 2;
    G.particles.push({ x, y, vx: Math.cos(a) * sp, vy: Math.sin(a) * sp, age: 0, life: 0.5, color: "#9a7fff", size: 2.5 });
  }
}

// ── Boucle de jeu ────────────────────────────────────────────────────────────
function loop(now) {
  const dt = Math.min(0.05, (now - G.lastTime) / 1000);
  G.lastTime = now;

  if (G.phase === "playing") {
    update(dt);
    render();
  }
  requestAnimationFrame(loop);
}

function update(dt) {
  // Prédiction du joueur : avance vers la cible cliquée.
  const p = G.predict;
  if (p.moving) {
    const dx = p.tx - p.x, dy = p.ty - p.y;
    const d = Math.hypot(dx, dy);
    const step = MOVE_SPEED * dt;
    if (d <= step || d < 1e-3) { p.x = p.tx; p.y = p.ty; p.moving = false; }
    else { p.x += (dx / d) * step; p.y += (dy / d) * step; }
  }

  // Caméra suit le joueur prédit.
  G.cam.x += (p.x - G.cam.x) * Math.min(1, dt * 8);
  G.cam.y += (p.y - G.cam.y) * Math.min(1, dt * 8);

  // Interpolation des entités vers leur cible serveur.
  const k = Math.min(1, dt * SNAP_INTERP);
  for (const e of G.entities.values()) {
    if (e.id === G.selfId) { e.x = p.x; e.y = p.y; continue; }
    e.x += (e.tx - e.x) * k;
    e.y += (e.ty - e.y) * k;
  }

  // Coords monde sous la souris (pour skills).
  const w = screenToWorld(G.mouse.sx, G.mouse.sy);
  G.mouse.wx = w.x; G.mouse.wy = w.y;

  // Particules & damage numbers.
  for (const q of G.particles) { q.age += dt; q.x += q.vx * dt; q.y += q.vy * dt; }
  G.particles = G.particles.filter((q) => q.age < q.life);
  for (const n of G.dmgNumbers) n.age += dt;
  G.dmgNumbers = G.dmgNumbers.filter((n) => n.age < n.life);

  // Ping périodique.
  if (Date.now() - G.lastPingSent > 3000) {
    G.lastPingSent = Date.now();
    send({ type: "Ping", timestamp: Date.now() });
  }

  $("player-count").textContent = "Joueurs: " + countPlayers();
}

function countPlayers() {
  let n = 0;
  for (const e of G.entities.values()) if (e.kind === "Player") n++;
  return n;
}

// ── Rendu ────────────────────────────────────────────────────────────────────
function render() {
  ctx.fillStyle = "#07070b";
  ctx.fillRect(0, 0, canvas.width, canvas.height);

  drawGround();
  drawGroundItems();
  drawEntities();
  drawParticles();
  drawDamageNumbers();
  drawMinimap();
}

function drawGround() {
  const cx = Math.round(G.cam.x), cy = Math.round(G.cam.y);
  for (let gy = -GROUND_RADIUS; gy <= GROUND_RADIUS; gy++) {
    for (let gx = -GROUND_RADIUS; gx <= GROUND_RADIUS; gx++) {
      const wx = cx + gx, wy = cy + gy;
      const s = worldToScreen(wx, wy);
      if (s.x < -TILE_W || s.x > canvas.width + TILE_W || s.y < -TILE_H || s.y > canvas.height + TILE_H) continue;
      // Damier subtil.
      const even = ((wx + wy) & 1) === 0;
      ctx.fillStyle = even ? "#171a20" : "#13161c";
      diamond(s.x, s.y);
      ctx.fill();
      ctx.strokeStyle = "rgba(60,70,90,0.18)";
      ctx.lineWidth = 1;
      ctx.stroke();
    }
  }
}

function diamond(x, y) {
  ctx.beginPath();
  ctx.moveTo(x, y - HALF_H);
  ctx.lineTo(x + HALF_W, y);
  ctx.lineTo(x, y + HALF_H);
  ctx.lineTo(x - HALF_W, y);
  ctx.closePath();
}

function drawGroundItems() {
  for (const gi of G.ground.values()) {
    const s = worldToScreen(gi.x, gi.y);
    const col = RARITY_COLOR[gi.rarity] || RARITY_COLOR[0];
    ctx.save();
    ctx.translate(s.x, s.y);
    ctx.fillStyle = col;
    ctx.shadowColor = col; ctx.shadowBlur = 10;
    ctx.beginPath();
    ctx.moveTo(0, -7); ctx.lineTo(6, 0); ctx.lineTo(0, 7); ctx.lineTo(-6, 0);
    ctx.closePath(); ctx.fill();
    ctx.restore();
  }
}

function drawEntities() {
  // Tri par profondeur (y monde croissant = plus bas à l'écran = devant).
  const list = [...G.entities.values()].sort((a, b) => (a.x + a.y) - (b.x + b.y));
  for (const e of list) {
    const s = worldToScreen(e.x, e.y);
    const self = e.id === G.selfId;
    const boss = e.kind === "Boss";
    const size = boss ? 30 : (e.kind === "Npc" ? 18 : 22);

    // Couleur du corps : selon la classe pour joueurs/mercenaires, sinon le type.
    let color;
    if (self) color = "#46b4ff";
    else if ((e.kind === "Player" || e.kind === "Mercenary") && e.cls != null && CLASS_COLOR[e.cls])
      color = e.kind === "Mercenary" ? mix(CLASS_COLOR[e.cls], "#46c83c", 0.45) : CLASS_COLOR[e.cls];
    else color = KIND_COLOR[e.kind] || "#cccccc";

    // Orientation écran : E-ish → droite (+1), W-ish → gauche (−1), sinon face.
    const face = [1, 2, 3].includes(e.dir) ? 1 : [5, 6, 7].includes(e.dir) ? -1 : 0;
    const fx = face || 1;

    // Ombre au sol.
    ctx.fillStyle = "rgba(0,0,0,0.35)";
    ctx.beginPath();
    ctx.ellipse(s.x, s.y + 2, size * 0.55, size * 0.26, 0, 0, Math.PI * 2);
    ctx.fill();

    const headR = size * 0.26;
    const legH = size * 0.22, torsoH = size * 0.6, torsoW = size * 0.66;
    const torsoY = s.y - legH - torsoH;
    const handY = torsoY + torsoH * 0.45;

    // Arme derrière le corps si l'entité fait dos (dir N/NE/NW = 0/1/7).
    const away = [0, 1, 7].includes(e.dir);
    if (away && e.weapon) drawWeapon(s.x + fx * torsoW * 0.62, handY, fx, e.weapon, size, color, true);

    // Jambes.
    ctx.fillStyle = shade(color, 0.55);
    ctx.fillRect(s.x - torsoW * 0.34, s.y - legH, torsoW * 0.26, legH);
    ctx.fillRect(s.x + torsoW * 0.08, s.y - legH, torsoW * 0.26, legH);
    // Torse.
    roundRect(ctx, s.x - torsoW / 2, torsoY, torsoW, torsoH, torsoW * 0.22);
    ctx.fillStyle = color; ctx.fill();
    ctx.strokeStyle = self ? "#cfeaff" : "rgba(0,0,0,0.45)"; ctx.lineWidth = self ? 2 : 1; ctx.stroke();
    // Tête.
    const headY = torsoY - headR * 0.5;
    ctx.beginPath(); ctx.arc(s.x, headY, headR, 0, Math.PI * 2);
    ctx.fillStyle = away ? shade(color, 0.7) : "#e8c69a"; ctx.fill();
    ctx.strokeStyle = "rgba(0,0,0,0.4)"; ctx.lineWidth = 1; ctx.stroke();

    // Bouclier (offhand 255) ou seconde arme (dual-wield) côté opposé.
    if (e.offhand === 255) {
      ctx.fillStyle = "#b9a06a";
      roundRect(ctx, s.x - fx * torsoW * 0.5 - 3, handY - 7, 8, 15, 3); ctx.fill();
      ctx.strokeStyle = "rgba(0,0,0,0.4)"; ctx.stroke();
    } else if (e.offhand) {
      drawWeapon(s.x - fx * torsoW * 0.5, handY, -fx, e.offhand, size, color, false);
    }
    // Arme principale devant.
    if (!away && e.weapon) drawWeapon(s.x + fx * torsoW * 0.5, handY, fx, e.weapon, size, color, false);

    // Barre de vie.
    if (e.hpMax > 0 && e.kind !== "Npc") {
      const frac = Math.max(0, Math.min(1, e.hp / e.hpMax));
      const bw = size + 6, bh = 4, bx = s.x - bw / 2, by = headY - headR - 9;
      ctx.fillStyle = "#000"; ctx.fillRect(bx - 1, by - 1, bw + 2, bh + 2);
      ctx.fillStyle = frac > 0.5 ? "#46c83c" : frac > 0.25 ? "#e0b020" : "#d83020";
      ctx.fillRect(bx, by, bw * frac, bh);
    }

    // Nom (joueurs / boss).
    if (e.kind === "Player" || boss) {
      ctx.font = "11px Segoe UI";
      ctx.textAlign = "center";
      ctx.fillStyle = self ? "#bfe6ff" : "#e8e4d8";
      ctx.shadowColor = "#000"; ctx.shadowBlur = 3;
      const label = e.level != null ? `${e.name} (${e.level})` : e.name;
      ctx.fillText(label, s.x, headY - headR - 16);
      ctx.shadowBlur = 0;
    }
  }
}

// Dessine une arme par code ([`Weapon::to_code`] côté Rust) à une main.
function drawWeapon(x, y, face, code, size, glowCol, behind) {
  const steel = behind ? "#7f8694" : "#c9d2e0", wood = behind ? "#5d3f25" : "#8a5a32";
  const L = size * 0.6;
  ctx.save();
  ctx.lineCap = "round";
  ctx.lineWidth = Math.max(2, size * 0.09);
  switch (code) {
    case 1: case 2: { // épée / épée courte
      const len = code === 2 ? L * 0.7 : L;
      ctx.strokeStyle = steel; ctx.beginPath();
      ctx.moveTo(x, y); ctx.lineTo(x + face * len * 0.5, y - len * 0.8); ctx.stroke();
      ctx.strokeStyle = "#7a5a2a"; ctx.lineWidth = Math.max(2, size * 0.05);
      ctx.beginPath(); ctx.moveTo(x - face * 5, y - 2); ctx.lineTo(x + face * 6, y - 7); ctx.stroke();
      break;
    }
    case 3: case 10: { // hache / os
      ctx.strokeStyle = wood; ctx.beginPath();
      ctx.moveTo(x, y + L * 0.3); ctx.lineTo(x + face * L * 0.4, y - L * 0.7); ctx.stroke();
      ctx.fillStyle = code === 10 ? "#d8d2c0" : steel;
      ctx.beginPath(); ctx.arc(x + face * L * 0.4, y - L * 0.6, size * 0.16, 0, Math.PI * 2); ctx.fill();
      break;
    }
    case 4: case 5: case 6: { // bâton / baguette / sceptre
      const len = code === 5 ? L * 0.7 : L;
      ctx.strokeStyle = wood; ctx.beginPath();
      ctx.moveTo(x, y + len * 0.3); ctx.lineTo(x + face * len * 0.3, y - len * 0.7); ctx.stroke();
      const tx = x + face * len * 0.3, ty = y - len * 0.7;
      ctx.fillStyle = mix(glowCol, "#ffffff", 0.4); ctx.beginPath(); ctx.arc(tx, ty, size * 0.13, 0, Math.PI * 2); ctx.fill();
      break;
    }
    case 7: { // orbe
      ctx.fillStyle = mix(glowCol, "#ffffff", 0.5); ctx.shadowColor = glowCol; ctx.shadowBlur = 8;
      ctx.beginPath(); ctx.arc(x + face * 4, y - 2, size * 0.16, 0, Math.PI * 2); ctx.fill(); ctx.shadowBlur = 0;
      break;
    }
    case 8: case 9: { // arc / arbalète
      ctx.strokeStyle = wood; ctx.beginPath();
      ctx.arc(x, y, L * 0.5, -1.1, 1.1, face < 0); ctx.stroke();
      break;
    }
    case 11: { // fourche
      ctx.strokeStyle = wood; ctx.beginPath();
      ctx.moveTo(x, y + L * 0.3); ctx.lineTo(x + face * L * 0.45, y - L * 0.7); ctx.stroke();
      break;
    }
  }
  ctx.restore();
}

function drawParticles() {
  for (const q of G.particles) {
    const s = worldToScreen(q.x, q.y);
    const a = 1 - q.age / q.life;
    ctx.globalAlpha = Math.max(0, a);
    ctx.fillStyle = q.color;
    ctx.fillRect(s.x - q.size / 2, s.y - q.size / 2, q.size, q.size);
  }
  ctx.globalAlpha = 1;
}

function drawDamageNumbers() {
  ctx.textAlign = "center";
  for (const n of G.dmgNumbers) {
    const s = worldToScreen(n.x, n.y);
    const a = 1 - n.age / n.life;
    ctx.globalAlpha = Math.max(0, a);
    ctx.font = `bold ${n.size}px Segoe UI`;
    ctx.fillStyle = n.color;
    ctx.shadowColor = "#000"; ctx.shadowBlur = 4;
    ctx.fillText(n.text, s.x, s.y - 28 - n.age * 30);
    ctx.shadowBlur = 0;
  }
  ctx.globalAlpha = 1;
}

function drawMinimap() {
  const W = mini.width, H = mini.height, scale = G.mapBig ? 6 : 3;
  mctx.clearRect(0, 0, W, H);
  mctx.fillStyle = "rgba(8,8,12,0.85)";
  mctx.fillRect(0, 0, W, H);
  const toMini = (wx, wy) => ({ x: W / 2 + (wx - G.cam.x) * scale, y: H / 2 + (wy - G.cam.y) * scale });
  for (const e of G.entities.values()) {
    const p = toMini(e.x, e.y);
    if (p.x < 0 || p.x > W || p.y < 0 || p.y > H) continue;
    mctx.fillStyle = e.id === G.selfId ? "#46b4ff" : (KIND_COLOR[e.kind] || "#aaa");
    const r = e.kind === "Boss" ? 3 : 2;
    mctx.fillRect(p.x - r, p.y - r, r * 2, r * 2);
  }
}

// ── Vitals / UI ──────────────────────────────────────────────────────────────
function refreshVitals() {
  const hf = Math.max(0, Math.min(1, G.hp / G.hpMax)) * 100;
  $("hp-fill").style.width = hf + "%";
  $("hp-text").textContent = `${Math.round(G.hp)}/${Math.round(G.hpMax)}`;
  const mf = Math.max(0, Math.min(1, G.mp / G.mpMax)) * 100;
  $("mp-fill").style.width = mf + "%";
  $("mp-text").textContent = `${Math.round(G.mp)}/${Math.round(G.mpMax)}`;
  $("level-badge").textContent = "Niv. " + G.level;
}

function buildHotbar() {
  const hb = $("hotbar");
  hb.innerHTML = "";
  for (let i = 1; i <= 9; i++) {
    const d = document.createElement("div");
    d.className = "slot";
    d.innerHTML = `<span class="key">${i}</span>`;
    d.onclick = () => useSkill(i);
    hb.appendChild(d);
  }
}

function pushChat(text, sys) {
  G.chat.push({ text, sys: !!sys });
  if (G.chat.length > 60) G.chat.shift();
  const log = $("chat-log");
  log.innerHTML = "";
  for (const c of G.chat.slice(-12)) {
    const div = document.createElement("div");
    if (c.sys) div.className = "sys";
    div.textContent = c.text;
    log.appendChild(div);
  }
  log.scrollTop = log.scrollHeight;
}

function applyInventory(slots) {
  for (const [idx, item] of slots) {
    G.inventory[idx] = item ? { name: item.name, rarity: rarityNum(item) } : null;
  }
  if (G.invOpen) renderInventory();
}
function rarityNum(item) {
  // SavedItem n'expose pas forcément la rareté ; on tente quelques champs.
  if (typeof item.rarity === "number") return item.rarity;
  return 0;
}
function renderInventory() {
  const grid = $("inv-grid");
  grid.innerHTML = "";
  const total = Math.max(40, G.inventory.length);
  for (let i = 0; i < total; i++) {
    const cell = document.createElement("div");
    cell.className = "inv-cell";
    const it = G.inventory[i];
    if (it) {
      cell.classList.add("r" + (it.rarity || 0));
      cell.textContent = it.name;
      cell.title = it.name;
      cell.onclick = () => send({ type: "EquipItem", inventory_slot: i });
    }
    grid.appendChild(cell);
  }
}

// ── Lobby UI ─────────────────────────────────────────────────────────────────
function renderGameList(games) {
  const list = $("game-list");
  list.innerHTML = "";
  if (!games.length) {
    list.innerHTML = `<div class="empty">Aucune partie — créez-en une !</div>`;
    return;
  }
  for (const g of games) {
    const row = document.createElement("div");
    row.className = "game-row";
    const lock = g.has_password ? " 🔒" : "";
    row.innerHTML =
      `<div><span class="gname">${escapeHtml(g.name)}</span>${lock}` +
      `<div class="meta">par ${escapeHtml(g.host_name)} · ${diffLabel(g.difficulty)} · niv~${Math.round(g.avg_level)}</div></div>` +
      `<div class="meta">${g.players}/${g.max_players}</div>`;
    row.onclick = () => {
      let pw = null;
      if (g.has_password) { pw = prompt("Mot de passe :"); if (pw === null) return; }
      send({ type: "JoinGame", game_id: g.id, password: pw });
    };
    list.appendChild(row);
  }
}
function diffLabel(d) {
  if (typeof d === "number") return ["Normal", "Cauchemar", "Enfer"][d] || "Normal";
  return d;
}
function escapeHtml(s) {
  return String(s).replace(/[&<>"]/g, (c) => ({ "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;" }[c]));
}

// ── Transitions d'écran ──────────────────────────────────────────────────────
function setConnectStatus(t) { $("connect-status").textContent = t; }
function toConnect(msg) {
  G.phase = "connect";
  $("connect").classList.remove("hidden");
  $("lobby").classList.add("hidden");
  $("create-modal").classList.add("hidden");
  $("hud").classList.add("hidden");
  G.entities.clear(); G.ground.clear();
  if (msg) setConnectStatus(msg);
}
function toLobby() {
  G.phase = "lobby";
  $("connect").classList.add("hidden");
  $("hud").classList.add("hidden");
  $("create-modal").classList.add("hidden");
  $("lobby").classList.remove("hidden");
  $("lobby-status").textContent = "";
}
function toPlaying() {
  G.phase = "playing";
  $("lobby").classList.add("hidden");
  $("connect").classList.add("hidden");
  $("create-modal").classList.add("hidden");
  $("hud").classList.remove("hidden");
  G.entities.clear(); G.ground.clear();
  G.predict.init = false;
  G.chat = []; pushChat("Bienvenue dans Allumina !", true);
  refreshVitals();
  G.lastTime = performance.now();
}

// ── Entrées : skills, clics ──────────────────────────────────────────────────
function useSkill(idx) {
  // Cible = mob le plus proche du curseur (optionnel).
  let target = null, best = 2.0;
  for (const e of G.entities.values()) {
    if (e.kind !== "Mob" && e.kind !== "Boss") continue;
    const d = Math.hypot(e.x - G.mouse.wx, e.y - G.mouse.wy);
    if (d < best) { best = d; target = e.id; }
  }
  send({ type: "UseSkill", skill_index: idx, target, cursor_x: G.mouse.wx, cursor_y: G.mouse.wy });
  spawnSkillFx(G.predict.x, G.predict.y);
}

canvas.addEventListener("contextmenu", (e) => e.preventDefault());
canvas.addEventListener("mousemove", (e) => {
  const r = canvas.getBoundingClientRect();
  G.mouse.sx = e.clientX - r.left;
  G.mouse.sy = e.clientY - r.top;
});
canvas.addEventListener("mousedown", (e) => {
  if (G.phase !== "playing") return;
  const r = canvas.getBoundingClientRect();
  const sx = e.clientX - r.left, sy = e.clientY - r.top;
  const w = screenToWorld(sx, sy);
  if (e.button === 0) {
    // Clic gauche : déplacement.
    G.predict.tx = w.x; G.predict.ty = w.y; G.predict.moving = true; G.predict.init = true;
    send({ type: "MoveIntent", target_x: w.x, target_y: w.y });
  } else if (e.button === 2) {
    // Clic droit : attaque du mob le plus proche du clic.
    let target = null, best = 1.6;
    for (const ent of G.entities.values()) {
      if (ent.kind !== "Mob" && ent.kind !== "Boss") continue;
      const d = Math.hypot(ent.x - w.x, ent.y - w.y);
      if (d < best) { best = d; target = ent.id; }
    }
    if (target != null) send({ type: "AttackTarget", entity_id: target });
  }
});

// ── Clavier ──────────────────────────────────────────────────────────────────
window.addEventListener("keydown", (e) => {
  // Chat ouvert : capture tout.
  if (G.chatOpen) {
    if (e.key === "Enter") {
      const inp = $("chat-input");
      const t = inp.value.trim();
      if (t) send({ type: "ChatMessage", text: t });
      inp.value = ""; inp.classList.add("hidden"); inp.blur(); G.chatOpen = false;
      e.preventDefault();
    } else if (e.key === "Escape") {
      const inp = $("chat-input");
      inp.value = ""; inp.classList.add("hidden"); inp.blur(); G.chatOpen = false;
    }
    return;
  }
  if (G.phase !== "playing") return;

  if (e.key >= "1" && e.key <= "9") { useSkill(parseInt(e.key, 10)); return; }
  switch (e.key) {
    case "Enter": {
      const inp = $("chat-input");
      inp.classList.remove("hidden"); inp.focus(); G.chatOpen = true;
      e.preventDefault();
      break;
    }
    case "i": case "I":
      G.invOpen = !G.invOpen;
      $("inventory").classList.toggle("hidden", !G.invOpen);
      if (G.invOpen) renderInventory();
      break;
    case "Tab":
      G.mapBig = !G.mapBig;
      mini.width = mini.height = G.mapBig ? 280 : 160;
      e.preventDefault();
      break;
    case "Escape":
      G.menuOpen = !G.menuOpen;
      $("menu").classList.toggle("hidden", !G.menuOpen);
      break;
    default:
      break;
  }
});

// ── Câblage des boutons ──────────────────────────────────────────────────────
$("classes").addEventListener("click", (e) => {
  const btn = e.target.closest(".class-btn");
  if (!btn) return;
  document.querySelectorAll(".class-btn").forEach((b) => b.classList.remove("selected"));
  btn.classList.add("selected");
  G.cls = btn.dataset.class;
});
$("login-btn").onclick = () => { G.name = $("name").value.trim() || "Héros"; connect(); };
$("server").addEventListener("keydown", (e) => { if (e.key === "Enter") $("login-btn").click(); });
// Default the server field to the page's own host (works behind any tunnel /
// domain and for local http://localhost:7000), so it never needs editing.
if (location.protocol === "http:" || location.protocol === "https:") {
  $("server").value = location.host;
}

$("refresh-btn").onclick = () => send({ type: "ListGames" });
$("create-btn").onclick = () => $("create-modal").classList.remove("hidden");
$("logout-btn").onclick = () => { if (G.ws) G.ws.close(); toConnect(""); };
$("cg-cancel").onclick = () => $("create-modal").classList.add("hidden");
$("cg-confirm").onclick = () => {
  send({
    type: "CreateGame",
    name: $("cg-name").value.trim() || "Partie",
    max_players: parseInt($("cg-max").value, 10),
    difficulty: parseInt($("cg-diff").value, 10),
    password: null,
  });
  $("create-modal").classList.add("hidden");
};
$("inv-close").onclick = () => { G.invOpen = false; $("inventory").classList.add("hidden"); };
$("menu-resume").onclick = () => { G.menuOpen = false; $("menu").classList.add("hidden"); };
$("menu-leave").onclick = () => { G.menuOpen = false; $("menu").classList.add("hidden"); send({ type: "LeaveGame" }); };

// ── Démarrage ────────────────────────────────────────────────────────────────
buildHotbar();
requestAnimationFrame(loop);
