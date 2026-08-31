"use strict";
/*
 * @id     cog.gameAllumina
 * @do     executer_jeu_allumina_legacy
 * @role   ui
 * @layer  outil
 * @human  Client web du jeu Allumina embarqué dans la galerie de jeux (mge) — version solo legacy, balisée au module.
 */
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

  // Carte de la zone (message ZoneMap). Sans elle on ne dessinait qu'un damier
  // infini : ni murs, ni portail visible — le joueur ne pouvait pas sortir du village.
  map: null,                   // { zone_name, width, height, tiles: Uint8Array, portals, level_range }
  // Déplacement : le bouton gauche maintenu déplace en continu vers le curseur.
  holdMove: false,
  lastMoveSent: 0,
  clickMark: null,             // { x, y, t } — retour visuel du point visé
  hoverEntity: null,           // entité sous le curseur (surbrillance + curseur)
  hoverItem: null,             // objet au sol sous le curseur
  target: null,                // cible d'attaque courante
  pickupTarget: null,          // objet vers lequel on marche pour le ramasser
  lastPickupSent: 0,
  // Touches de direction maintenues, par *code physique* (voir stepKeyboardMove).
  keys: new Set(),
  keyMoving: false,
};

// Codes de tuile — doivent suivre `protocol::tile_code` côté Rust.
const TILE = { FLOOR: 0, WALL: 1, VOID: 2, PLAYER_SPAWN: 3, MOB_SPAWN: 4, EXIT: 5 };
// Codes de decor (miroir de `protocol::decor_code`, meme ordre).
const DECOR = {
  FOUNTAIN: 0, TORCH: 1, BENCH: 2, MARKET_STALL: 3, BARREL: 4, CRATE: 5,
  DUMMY: 6, WEAPON_RACK: 7, FLAG: 8, ALTAR: 9, CANDLE: 10, TABLE: 11,
  FIREPLACE: 12, SIGN: 13, TREE: 14, BUSH: 15, FLOWER: 16,
};
// Codes d'obstacles (miroir de `protocol::obstacle_code`).
const OBST = { ROCK: 0, BOULDERS: 1, DEAD_TREE: 2, POND: 3, RUINS: 4, THORNBUSH: 5 };
// Roles de PNJ (miroir de `protocol::npc_code`) : libelle + teinte.
const NPC_ROLE = [
  ["Marchand", "#d8c070"], ["Armes", "#c0c8d8"], ["Armures", "#a8b0c0"],
  ["Apothicaire", "#d06090"], ["Forgeron", "#e08040"], ["Guerisseur", "#70d0b0"],
  ["Quetes", "#f0c850"], ["Recruteur", "#a090e0"],
];
// Intervalle minimal entre deux MoveIntent en maintien (ms) : inutile de saturer
// le serveur à 60 Hz quand il simule les déplacements à 20 Hz.
const MOVE_SEND_MS = 100;
// Rayon de sélection d'un ennemi au clic, en tuiles. 1.6 était trop exigeant :
// il fallait cliquer presque au pixel près sur le monstre.
const PICK_RADIUS = 2.6;
// Deux portées différentes se cumulent côté serveur : `try_pickup_item` refuse
// au-delà de 3 tuiles, mais la simulation elle-même (plugins/player.rs) exige
// ≤ 1.8. C'est la plus stricte qui compte — viser 2.2 faisait échouer tous les
// ramassages en silence. On s'approche donc franchement en deçà.
const PICKUP_RANGE = 1.4;
// Rayon de sélection d'un objet au sol au clic.
const ITEM_PICK_RADIUS = 1.2;
// Anti-rafale : une demande de ramassage au plus toutes les 250 ms.
const PICKUP_SEND_MS = 250;
// Distance visee devant le joueur en deplacement clavier, en tuiles. Le serveur
// ne connait pas les directions : il ne sait qu'aller *a* un point. On lui donne
// donc un point droit devant, reactualise en continu.
const KEY_LOOKAHEAD = 4;

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
    case "ZoneMap":
      applyZoneMap(m.map);
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

  // Pilotage. Hors du bloc `if (p.moving)` ci-dessous : il y etait enferme, si
  // bien qu'une fois a l'arret ni le clic maintenu ni la poursuite d'un objet
  // ne repartaient — il fallait recliquer pour redemarrer.
  if (G.phase === "playing") {
    const byKeyboard = stepKeyboardMove();
    stepPickup();
    // Maintien du clic gauche : on relance l'intention vers le curseur. C'est ce
    // qui manquait le plus — sans cela il fallait cliquer à répétition pour avancer.
    if (!byKeyboard && G.holdMove) {
      sendMove(G.mouse.wx, G.mouse.wy, false);
      G.clickMark = { x: G.mouse.wx, y: G.mouse.wy, t: performance.now() };
    }
  }

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

// ── Carte de zone ────────────────────────────────────────────────────────────

/** Reçoit la carte statique de la zone et l'affiche. */
function applyZoneMap(map) {
  if (!map) return;
  G.map = {
    zone_name: map.zone_name,
    width: map.width,
    height: map.height,
    tiles: Uint8Array.from(map.tiles || []),
    portals: map.portals || [],
    level_range: map.level_range || [0, 0],
    decors: map.decors || [],
    obstacles: map.obstacles || [],
  };
  // Zone d'ou l'on vient : la boussole doit cesser de pointer le portail par
  // lequel on vient d'arriver, sinon elle renvoie le joueur d'ou il sort.
  G.prevZone = G.zoneName || null;
  G.zoneName = map.zone_name;
  const [lo, hi] = G.map.level_range;
  const band = lo === 0 && hi === 0 ? "zone sûre" : `niveaux ${lo}\u2013${hi}`;
  setZoneBanner(`${map.zone_name} — ${band}`);
  const onward = G.map.portals.filter((p) => p.to_name !== G.prevZone);
  const dests = (onward.length ? onward : G.map.portals).map((p) => p.to_name).join(", ");
  pushChat(
    dests ? `Zone : ${map.zone_name}. Sortie vers ${dests}.` : `Zone : ${map.zone_name}.`,
    true,
  );
  if (G.map.level_range[1] === 0) {
    pushChat("Zone sûre : aucun ennemi ici. Suivez la flèche dorée pour aller chasser.", true);
  }
}

/** Type de la tuile (x, y), ou VOID hors carte. */
function tileAt(x, y) {
  const m = G.map;
  if (!m) return TILE.VOID;
  if (x < 0 || y < 0 || x >= m.width || y >= m.height) return TILE.VOID;
  return m.tiles[y * m.width + x];
}

function isWalkable(x, y) {
  const t = tileAt(x, y);
  return t !== TILE.WALL && t !== TILE.VOID;
}

/** Bandeau de nom de zone (créé à la volée pour ne pas toucher à l'HTML). */
function setZoneBanner(text) {
  let el = document.getElementById("zone-banner");
  if (!el) {
    el = document.createElement("div");
    el.id = "zone-banner";
    document.body.appendChild(el);
  }
  el.textContent = text;
  el.classList.remove("fade");
  // Redémarre l'animation d'estompage.
  void el.offsetWidth;
  el.classList.add("fade");
}

// ── Rendu ────────────────────────────────────────────────────────────────────
function render() {
  ctx.fillStyle = "#07070b";
  ctx.fillRect(0, 0, canvas.width, canvas.height);

  drawGround();
  drawClickMark();
  drawGroundItems();
  drawScene();
  drawTargeting();
  drawItemLabel();
  drawParticles();
  drawDamageNumbers();
  drawExitCompass();
  drawMinimap();
}

function drawGround() {
  // Sans carte reçue (ancien serveur), on retombe sur le damier neutre.
  if (!G.map) return drawGroundFallback();

  const m = G.map;
  const cx = Math.round(G.cam.x), cy = Math.round(G.cam.y);
  const now = performance.now();
  // Pulsation des portails : ils doivent sauter aux yeux, c'est la seule sortie.
  const pulse = 0.5 + 0.5 * Math.sin(now / 300);

  for (let gy = -GROUND_RADIUS; gy <= GROUND_RADIUS; gy++) {
    for (let gx = -GROUND_RADIUS; gx <= GROUND_RADIUS; gx++) {
      const wx = cx + gx, wy = cy + gy;
      const s = worldToScreen(wx, wy);
      if (s.x < -TILE_W || s.x > canvas.width + TILE_W || s.y < -TILE_H || s.y > canvas.height + TILE_H) continue;

      const t = tileAt(wx, wy);
      if (t === TILE.VOID) continue;   // hors carte : on ne dessine rien

      if (t === TILE.WALL) {
        drawWallTile(s.x, s.y, wx, wy);
        continue;
      }

      // Sol : damier discret, teinté selon la nature de la tuile.
      const even = ((wx + wy) & 1) === 0;
      let fill = even ? "#1d2129" : "#181c23";
      if (t === TILE.EXIT) {
        fill = mix("#1d2129", "#f0c850", 0.35 + 0.35 * pulse);
      } else if (t === TILE.PLAYER_SPAWN) {
        fill = mix(fill, "#3a7fe0", 0.25);
      }
      ctx.fillStyle = fill;
      diamond(s.x, s.y);
      ctx.fill();
      ctx.strokeStyle = "rgba(70,82,105,0.22)";
      ctx.lineWidth = 1;
      ctx.stroke();

      if (t === TILE.EXIT) {
        ctx.strokeStyle = `rgba(240,200,80,${0.55 + 0.45 * pulse})`;
        ctx.lineWidth = 2;
        diamond(s.x, s.y);
        ctx.stroke();
      }
    }
  }

  drawPortalLabels(pulse);
}

/** Mur : losange assombri surmonté d'un bloc, pour donner du relief. */
function drawWallTile(sx, sy, wx, wy) {
  const H = 26;
  // Face supérieure.
  ctx.fillStyle = "#39414f";
  ctx.beginPath();
  ctx.moveTo(sx, sy - HALF_H - H);
  ctx.lineTo(sx + HALF_W, sy - H);
  ctx.lineTo(sx, sy + HALF_H - H);
  ctx.lineTo(sx - HALF_W, sy - H);
  ctx.closePath();
  ctx.fill();
  // Faces latérales.
  ctx.fillStyle = "#252b36";
  ctx.beginPath();
  ctx.moveTo(sx - HALF_W, sy - H); ctx.lineTo(sx, sy + HALF_H - H);
  ctx.lineTo(sx, sy + HALF_H); ctx.lineTo(sx - HALF_W, sy);
  ctx.closePath(); ctx.fill();
  ctx.fillStyle = "#1b2028";
  ctx.beginPath();
  ctx.moveTo(sx + HALF_W, sy - H); ctx.lineTo(sx, sy + HALF_H - H);
  ctx.lineTo(sx, sy + HALF_H); ctx.lineTo(sx + HALF_W, sy);
  ctx.closePath(); ctx.fill();
}

/** Étiquette flottante au-dessus de chaque portail : où il mène. */
function drawPortalLabels(pulse) {
  if (!G.map) return;
  ctx.textAlign = "center";
  ctx.font = "bold 13px system-ui, sans-serif";
  for (const p of G.map.portals) {
    const s = worldToScreen(p.x, p.y);
    if (s.x < -200 || s.x > canvas.width + 200 || s.y < -200 || s.y > canvas.height + 200) continue;
    const label = "\u27a4 " + p.to_name;
    const w = ctx.measureText(label).width + 14;
    const y = s.y - 44;
    ctx.fillStyle = `rgba(20,18,8,${0.55 + 0.25 * pulse})`;
    roundRect(ctx, s.x - w / 2, y - 15, w, 21, 5);
    ctx.fill();
    ctx.strokeStyle = `rgba(240,200,80,${0.5 + 0.5 * pulse})`;
    ctx.lineWidth = 1;
    ctx.stroke();
    ctx.fillStyle = "#f5dc9a";
    ctx.fillText(label, s.x, y);
  }
  ctx.textAlign = "start";
}

/** Damier neutre — utilisé seulement si le serveur n'envoie pas de carte. */
function drawGroundFallback() {
  const cx = Math.round(G.cam.x), cy = Math.round(G.cam.y);
  for (let gy = -GROUND_RADIUS; gy <= GROUND_RADIUS; gy++) {
    for (let gx = -GROUND_RADIUS; gx <= GROUND_RADIUS; gx++) {
      const wx = cx + gx, wy = cy + gy;
      const s = worldToScreen(wx, wy);
      if (s.x < -TILE_W || s.x > canvas.width + TILE_W || s.y < -TILE_H || s.y > canvas.height + TILE_H) continue;
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

/** Anneau qui se resserre là où le joueur a cliqué : confirme que l'ordre est pris. */
function drawClickMark() {
  if (!G.clickMark) return;
  const age = (performance.now() - G.clickMark.t) / 450;
  if (age >= 1) { G.clickMark = null; return; }
  const s = worldToScreen(G.clickMark.x, G.clickMark.y);
  const r = 20 * (1 - age) + 6;
  ctx.strokeStyle = `rgba(120,200,255,${0.75 * (1 - age)})`;
  ctx.lineWidth = 2;
  ctx.beginPath();
  ctx.ellipse(s.x, s.y, r, r * 0.5, 0, 0, Math.PI * 2);
  ctx.stroke();
}

/** Cercle sous l'ennemi survolé et sous la cible courante. */
/** Étiquette de l'objet au sol survolé — sinon on ignore ce qu'on ramasse. */
function drawItemLabel() {
  const it = G.hoverItem;
  if (!it) return;
  const s = worldToScreen(it.x, it.y);
  const col = RARITY_COLOR[it.rarity] || "#c8c4b8";
  ctx.font = "bold 13px system-ui, sans-serif";
  ctx.textAlign = "center";
  const w = ctx.measureText(it.name).width + 14;
  ctx.fillStyle = "rgba(8,8,12,0.82)";
  roundRect(ctx, s.x - w / 2, s.y - 40, w, 20, 4);
  ctx.fill();
  ctx.strokeStyle = col;
  ctx.lineWidth = 1;
  ctx.stroke();
  ctx.fillStyle = col;
  ctx.fillText(it.name, s.x, s.y - 26);
  ctx.textAlign = "start";
}

function drawTargeting() {
  const ring = (ent, col, w) => {
    const s = worldToScreen(ent.x, ent.y);
    ctx.strokeStyle = col;
    ctx.lineWidth = w;
    ctx.beginPath();
    ctx.ellipse(s.x, s.y, HALF_W * 0.55, HALF_H * 0.55, 0, 0, Math.PI * 2);
    ctx.stroke();
  };
  if (G.hoverEntity && G.entities.has(G.hoverEntity.id)) {
    ring(G.entities.get(G.hoverEntity.id), "rgba(255,220,120,0.85)", 2);
  }
  if (G.target != null && G.entities.has(G.target)) {
    const t = G.entities.get(G.target);
    if (t.hp > 0) ring(t, "rgba(230,70,60,0.95)", 2.5);
    else G.target = null;
  }
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

/**
 * Dessine decor et entites dans un seul passage trie par profondeur.
 *
 * Les props doivent s'intercaler entre les personnages : un joueur au nord de
 * la fontaine passe derriere, au sud il passe devant. Deux passes separees
 * plaquaient tout le decor sous tout le monde.
 */
function drawScene() {
  const list = [];
  for (const e of G.entities.values()) list.push({ d: e.x + e.y, e });
  if (G.map) {
    for (const d of G.map.decors) list.push({ d: d.x + d.y + 1, decor: d });
    for (const o of G.map.obstacles) list.push({ d: o.x + o.y, obst: o });
  }
  list.sort((a, b) => a.d - b.d);
  for (const it of list) {
    if (it.decor) drawDecor(it.decor);
    else if (it.obst) drawObstacle(it.obst);
    else drawEntity(it.e);
  }
}

function drawEntity(e) {
  {
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

    // PNJ : nom authored + role. Sept silhouettes muettes et identiques ne
    // disaient pas qui vendait des potions et qui donnait les quetes.
    if (e.kind === "Npc") {
      const role = NPC_ROLE[e.cls] || null;
      ctx.textAlign = "center";
      ctx.shadowColor = "#000"; ctx.shadowBlur = 3;
      ctx.font = "11px Segoe UI";
      ctx.fillStyle = "#e8e4d8";
      ctx.fillText(e.name, s.x, headY - headR - 16);
      if (role) {
        ctx.font = "10px Segoe UI";
        ctx.fillStyle = role[1];
        ctx.fillText(role[0], s.x, headY - headR - 5);
      }
      ctx.shadowBlur = 0;
      // Losange dore au-dessus du donneur de quetes : le seul PNJ qui fait
      // avancer le joueur, il doit se reperer de loin.
      if (e.cls === 6) {
        const b = 0.5 + 0.5 * Math.sin(performance.now() / 320);
        ctx.fillStyle = "rgba(240,200,80," + (0.55 + 0.45 * b) + ")";
        ctx.beginPath();
        const my = headY - headR - 26 - b * 3;
        ctx.moveTo(s.x, my - 6); ctx.lineTo(s.x + 5, my); ctx.lineTo(s.x, my + 6); ctx.lineTo(s.x - 5, my);
        ctx.closePath(); ctx.fill();
      }
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

/**
 * Un obstacle genere (rocher, mare, ruine) a sa position monde.
 *
 * Le generateur en dispose 15 a 30 par zone de chasse. Ils n'etaient pas
 * repliques : chaque zone arrivait donc dans le navigateur comme un rectangle
 * nu avec des monstres dessus et aucun repere pour s'orienter.
 */
function drawObstacle(o) {
  const s = worldToScreen(o.x, o.y);
  if (s.x < -90 || s.x > canvas.width + 90 || s.y < -110 || s.y > canvas.height + 90) return;
  ctx.save();
  ctx.lineWidth = 1.5;
  const shadow = (rx, ry) => {
    ctx.fillStyle = "rgba(0,0,0,0.32)";
    ctx.beginPath(); ctx.ellipse(s.x, s.y + 2, rx, ry, 0, 0, Math.PI * 2); ctx.fill();
  };
  const rock = (cx, cy, r, tone) => {
    ctx.fillStyle = tone;
    ctx.beginPath();
    ctx.moveTo(cx - r, cy);
    ctx.lineTo(cx - r * 0.55, cy - r * 0.95);
    ctx.lineTo(cx + r * 0.35, cy - r * 1.05);
    ctx.lineTo(cx + r, cy - r * 0.25);
    ctx.lineTo(cx + r * 0.6, cy + r * 0.25);
    ctx.lineTo(cx - r * 0.5, cy + r * 0.28);
    ctx.closePath(); ctx.fill();
    ctx.strokeStyle = "rgba(0,0,0,0.45)"; ctx.stroke();
    // Facette eclairee, pour que ce ne soit pas une tache plate.
    ctx.fillStyle = "rgba(255,255,255,0.10)";
    ctx.beginPath();
    ctx.moveTo(cx - r * 0.55, cy - r * 0.95);
    ctx.lineTo(cx + r * 0.35, cy - r * 1.05);
    ctx.lineTo(cx + r * 0.1, cy - r * 0.4);
    ctx.closePath(); ctx.fill();
  };

  switch (o.kind) {
    case OBST.ROCK:
      shadow(15, 7);
      rock(s.x, s.y, 15, "#767e8c");
      break;
    case OBST.BOULDERS:
      shadow(26, 11);
      rock(s.x - 12, s.y + 3, 13, "#69707d");
      rock(s.x + 13, s.y + 2, 12, "#727a88");
      rock(s.x, s.y - 4, 17, "#7d8695");
      break;
    case OBST.DEAD_TREE: {
      shadow(11, 5);
      ctx.strokeStyle = "#4a3a2c"; ctx.lineCap = "round";
      ctx.lineWidth = 5;
      ctx.beginPath(); ctx.moveTo(s.x, s.y); ctx.lineTo(s.x + 2, s.y - 34); ctx.stroke();
      ctx.lineWidth = 3;
      for (const [dx, dy] of [[-13, -12], [12, -16], [-9, -22]]) {
        ctx.beginPath();
        ctx.moveTo(s.x + 1, s.y - 20);
        ctx.quadraticCurveTo(s.x + dx * 0.5, s.y - 26, s.x + dx, s.y - 26 + dy * 0.4);
        ctx.stroke();
      }
      break;
    }
    case OBST.POND: {
      const t = performance.now();
      ctx.fillStyle = "#2b4a5e";
      ctx.beginPath(); ctx.ellipse(s.x, s.y, 34, 17, 0, 0, Math.PI * 2); ctx.fill();
      ctx.fillStyle = "#356b8a";
      ctx.beginPath(); ctx.ellipse(s.x, s.y - 1, 29, 13, 0, 0, Math.PI * 2); ctx.fill();
      // Rides concentriques : de l'eau immobile ressemble a une tache bleue.
      ctx.strokeStyle = "rgba(180,220,240,0.22)"; ctx.lineWidth = 1.5;
      for (let i = 0; i < 3; i++) {
        const ph = ((t / 2400) + i / 3) % 1;
        ctx.beginPath();
        ctx.ellipse(s.x, s.y - 1, 6 + 22 * ph, 3 + 10 * ph, 0, 0, Math.PI * 2);
        ctx.stroke();
      }
      break;
    }
    case OBST.RUINS: {
      shadow(30, 12);
      // Trois pans de mur brises, de hauteurs inegales.
      const cols = [[-24, 26], [-4, 38], [18, 20]];
      for (const [dx, h] of cols) {
        ctx.fillStyle = "#8a8478";
        ctx.fillRect(s.x + dx, s.y - h, 15, h);
        ctx.strokeStyle = "rgba(0,0,0,0.45)";
        ctx.strokeRect(s.x + dx, s.y - h, 15, h);
        ctx.fillStyle = "rgba(255,255,255,0.10)";
        ctx.fillRect(s.x + dx, s.y - h, 15, 3);
      }
      break;
    }
    case OBST.THORNBUSH:
      shadow(12, 6);
      ctx.fillStyle = "#24451f";
      for (const [ox, oy, r] of [[-7, -6, 9], [7, -6, 9], [0, -13, 10]]) {
        ctx.beginPath(); ctx.arc(s.x + ox, s.y + oy, r, 0, Math.PI * 2); ctx.fill();
      }
      ctx.strokeStyle = "#6a5a2a"; ctx.lineWidth = 1;
      for (let i = 0; i < 7; i++) {
        const a = i * 0.9;
        ctx.beginPath();
        ctx.moveTo(s.x + Math.cos(a) * 7, s.y - 8 + Math.sin(a) * 6);
        ctx.lineTo(s.x + Math.cos(a) * 13, s.y - 8 + Math.sin(a) * 11);
        ctx.stroke();
      }
      break;
    default:
      shadow(12, 6);
      rock(s.x, s.y, 12, "#767e8c");
  }
  ctx.restore();
}

/**
 * Un prop de decor a sa position monde.
 *
 * Purement cosmetique, mais son absence est ce qui faisait passer le village
 * pour un terrain vague : sept silhouettes anonymes sur une grille nue.
 */
function drawDecor(d) {
  const s = worldToScreen(d.x + 0.5, d.y + 0.5);
  if (s.x < -80 || s.x > canvas.width + 80 || s.y < -100 || s.y > canvas.height + 80) return;
  const t = performance.now();
  ctx.save();
  ctx.lineWidth = 1.5;

  const shadow = (rx, ry) => {
    ctx.fillStyle = "rgba(0,0,0,0.30)";
    ctx.beginPath(); ctx.ellipse(s.x, s.y + 2, rx, ry, 0, 0, Math.PI * 2); ctx.fill();
  };
  const box = (w, h, fill, top) => {
    ctx.fillStyle = fill;
    roundRect(ctx, s.x - w / 2, s.y - h, w, h, 2); ctx.fill();
    ctx.strokeStyle = "rgba(0,0,0,0.45)"; ctx.stroke();
    if (top) { ctx.fillStyle = top; ctx.fillRect(s.x - w / 2, s.y - h, w, 3); }
  };
  // Flamme vacillante partagee par torche / bougie / feu.
  const flame = (fy, r) => {
    const f = 0.75 + 0.25 * Math.sin(t / 90 + d.x * 3 + d.y);
    const g = ctx.createRadialGradient(s.x, fy, 0, s.x, fy, r * 4 * f);
    g.addColorStop(0, "rgba(255,190,90,0.55)"); g.addColorStop(1, "rgba(255,150,40,0)");
    ctx.fillStyle = g;
    ctx.beginPath(); ctx.arc(s.x, fy, r * 4 * f, 0, Math.PI * 2); ctx.fill();
    ctx.fillStyle = "#ffd070";
    ctx.beginPath(); ctx.ellipse(s.x, fy, r * f, r * 1.7 * f, 0, 0, Math.PI * 2); ctx.fill();
  };

  switch (d.kind) {
    case DECOR.FOUNTAIN: {
      shadow(26, 12);
      ctx.fillStyle = "#5a6272";
      ctx.beginPath(); ctx.ellipse(s.x, s.y - 4, 24, 11, 0, 0, Math.PI * 2); ctx.fill();
      ctx.strokeStyle = "#3a404c"; ctx.stroke();
      ctx.fillStyle = "#2f6f9a";
      ctx.beginPath(); ctx.ellipse(s.x, s.y - 6, 18, 8, 0, 0, Math.PI * 2); ctx.fill();
      ctx.fillStyle = "#6a7484";
      roundRect(ctx, s.x - 4, s.y - 26, 8, 20, 2); ctx.fill();
      // Jet d'eau.
      ctx.strokeStyle = "rgba(140,200,240,0.75)"; ctx.lineWidth = 2;
      for (let i = 0; i < 5; i++) {
        const ph = ((t / 620) + i / 5) % 1;
        ctx.beginPath();
        ctx.moveTo(s.x, s.y - 26);
        ctx.lineTo(s.x + Math.cos(i * 1.26) * 14 * ph, s.y - 26 + 22 * ph * ph);
        ctx.stroke();
      }
      break;
    }
    case DECOR.TORCH:
      shadow(6, 3);
      ctx.fillStyle = "#5d3f25"; ctx.fillRect(s.x - 2, s.y - 24, 4, 24);
      flame(s.y - 28, 4);
      break;
    case DECOR.CANDLE:
      ctx.fillStyle = "#e0d8c0"; ctx.fillRect(s.x - 1.5, s.y - 8, 3, 8);
      flame(s.y - 10, 2);
      break;
    case DECOR.FIREPLACE:
      shadow(14, 7);
      ctx.fillStyle = "#4a4038";
      ctx.beginPath(); ctx.ellipse(s.x, s.y - 2, 13, 6, 0, 0, Math.PI * 2); ctx.fill();
      flame(s.y - 10, 6);
      break;
    case DECOR.BENCH:
      shadow(16, 6);
      box(28, 6, "#7a5533"); ctx.fillStyle = "#5d3f25";
      ctx.fillRect(s.x - 12, s.y - 6, 3, 6); ctx.fillRect(s.x + 9, s.y - 6, 3, 6);
      break;
    case DECOR.TABLE:
      shadow(16, 7);
      box(30, 5, "#8a6238", "#a67c4a");
      ctx.fillStyle = "#5d3f25";
      ctx.fillRect(s.x - 13, s.y - 5, 3, 5); ctx.fillRect(s.x + 10, s.y - 5, 3, 5);
      break;
    case DECOR.MARKET_STALL: {
      shadow(24, 10);
      ctx.fillStyle = "#6b4a2c";
      ctx.fillRect(s.x - 20, s.y - 16, 40, 16);
      ctx.strokeStyle = "rgba(0,0,0,0.45)"; ctx.strokeRect(s.x - 20, s.y - 16, 40, 16);
      // Auvent raye.
      for (let i = 0; i < 6; i++) {
        ctx.fillStyle = i % 2 ? "#c0503c" : "#e8ddc8";
        ctx.beginPath();
        ctx.moveTo(s.x - 24 + i * 8, s.y - 30);
        ctx.lineTo(s.x - 16 + i * 8, s.y - 30);
        ctx.lineTo(s.x - 18 + i * 8, s.y - 22);
        ctx.lineTo(s.x - 26 + i * 8, s.y - 22);
        ctx.closePath(); ctx.fill();
      }
      ctx.fillStyle = "#5d3f25";
      ctx.fillRect(s.x - 22, s.y - 30, 3, 30); ctx.fillRect(s.x + 19, s.y - 30, 3, 30);
      break;
    }
    case DECOR.BARREL:
      shadow(9, 4);
      box(16, 20, "#8a5a32");
      ctx.strokeStyle = "#5a4028"; ctx.lineWidth = 2;
      ctx.beginPath(); ctx.moveTo(s.x - 8, s.y - 14); ctx.lineTo(s.x + 8, s.y - 14);
      ctx.moveTo(s.x - 8, s.y - 6); ctx.lineTo(s.x + 8, s.y - 6); ctx.stroke();
      break;
    case DECOR.CRATE:
      shadow(9, 4);
      box(18, 16, "#9a7040");
      ctx.strokeStyle = "#6a4a28";
      ctx.beginPath();
      ctx.moveTo(s.x - 9, s.y - 16); ctx.lineTo(s.x + 9, s.y);
      ctx.moveTo(s.x + 9, s.y - 16); ctx.lineTo(s.x - 9, s.y); ctx.stroke();
      break;
    case DECOR.DUMMY:
      shadow(9, 4);
      ctx.fillStyle = "#6b4a2c"; ctx.fillRect(s.x - 2, s.y - 16, 4, 16);
      ctx.fillStyle = "#c8a878";
      ctx.beginPath(); ctx.ellipse(s.x, s.y - 24, 9, 11, 0, 0, Math.PI * 2); ctx.fill();
      ctx.strokeStyle = "#8a6a48"; ctx.stroke();
      ctx.fillStyle = "#8a5a32"; ctx.fillRect(s.x - 13, s.y - 26, 26, 3);
      break;
    case DECOR.WEAPON_RACK:
      shadow(12, 5);
      ctx.fillStyle = "#6b4a2c"; ctx.fillRect(s.x - 14, s.y - 4, 28, 4);
      ctx.fillRect(s.x - 14, s.y - 26, 28, 3);
      ctx.strokeStyle = "#c9d2e0"; ctx.lineWidth = 2.5;
      for (let i = -1; i <= 1; i++) {
        ctx.beginPath(); ctx.moveTo(s.x + i * 9, s.y - 4); ctx.lineTo(s.x + i * 9, s.y - 26); ctx.stroke();
      }
      break;
    case DECOR.FLAG: {
      shadow(5, 3);
      ctx.fillStyle = "#6b6b6b"; ctx.fillRect(s.x - 1.5, s.y - 40, 3, 40);
      const w = 0.5 + 0.5 * Math.sin(t / 260 + d.x);
      ctx.fillStyle = "#b03040";
      ctx.beginPath();
      ctx.moveTo(s.x + 1.5, s.y - 40);
      ctx.quadraticCurveTo(s.x + 14, s.y - 36 - 3 * w, s.x + 22, s.y - 32);
      ctx.lineTo(s.x + 22, s.y - 22);
      ctx.quadraticCurveTo(s.x + 12, s.y - 24 + 3 * w, s.x + 1.5, s.y - 26);
      ctx.closePath(); ctx.fill();
      break;
    }
    case DECOR.ALTAR: {
      shadow(20, 9);
      box(34, 14, "#8a8478", "#a8a294");
      const gl = 0.5 + 0.5 * Math.sin(t / 500);
      const g = ctx.createRadialGradient(s.x, s.y - 22, 0, s.x, s.y - 22, 20);
      g.addColorStop(0, "rgba(240,200,110," + (0.30 + 0.25 * gl) + ")");
      g.addColorStop(1, "rgba(240,200,110,0)");
      ctx.fillStyle = g;
      ctx.beginPath(); ctx.arc(s.x, s.y - 22, 20, 0, Math.PI * 2); ctx.fill();
      break;
    }
    case DECOR.SIGN:
      shadow(6, 3);
      ctx.fillStyle = "#5d3f25"; ctx.fillRect(s.x - 1.5, s.y - 20, 3, 20);
      box(22, 30, "#9a7040");
      ctx.fillStyle = "#6a4a28";
      ctx.fillRect(s.x - 7, s.y - 26, 14, 2); ctx.fillRect(s.x - 7, s.y - 22, 10, 2);
      break;
    case DECOR.TREE: {
      shadow(14, 6);
      ctx.fillStyle = "#5d3f25"; ctx.fillRect(s.x - 3, s.y - 20, 6, 20);
      const sway = Math.sin(t / 900 + d.x) * 1.5;
      ctx.fillStyle = "#2e6b32";
      for (const [ox, oy, r] of [[-8, -26, 12], [8, -26, 12], [0, -36, 14]]) {
        ctx.beginPath(); ctx.arc(s.x + ox + sway, s.y + oy, r, 0, Math.PI * 2); ctx.fill();
      }
      ctx.fillStyle = "rgba(120,200,110,0.30)";
      ctx.beginPath(); ctx.arc(s.x - 4 + sway, s.y - 38, 8, 0, Math.PI * 2); ctx.fill();
      break;
    }
    case DECOR.BUSH:
      shadow(10, 5);
      ctx.fillStyle = "#356b38";
      for (const [ox, oy, r] of [[-6, -5, 8], [6, -5, 8], [0, -11, 9]]) {
        ctx.beginPath(); ctx.arc(s.x + ox, s.y + oy, r, 0, Math.PI * 2); ctx.fill();
      }
      break;
    case DECOR.FLOWER:
      ctx.strokeStyle = "#3f7a3a"; ctx.lineWidth = 1.5;
      ctx.beginPath(); ctx.moveTo(s.x, s.y); ctx.lineTo(s.x, s.y - 7); ctx.stroke();
      ctx.fillStyle = ["#e07090", "#e0c060", "#a080e0"][(d.x + d.y) % 3];
      for (let i = 0; i < 5; i++) {
        const a = i * Math.PI * 0.4;
        ctx.beginPath();
        ctx.arc(s.x + Math.cos(a) * 2.6, s.y - 7 + Math.sin(a) * 2.6, 2.2, 0, Math.PI * 2);
        ctx.fill();
      }
      break;
    default:
      shadow(7, 3);
      box(12, 12, "#6a6a72");
  }
  ctx.restore();
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

/**
 * Fleche de bord d'ecran vers la sortie de zone, avec la distance.
 *
 * Le village est un hub sans le moindre ennemi : sa seule sortie est une tuile
 * unique a 34 cases du point d'apparition. Sans indicateur, on tourne en rond
 * dans une place vide en concluant que le jeu n'a pas de monstres.
 */
function drawExitCompass() {
  if (!G.map || !G.map.portals.length || G.phase !== "playing") return;
  // Portail le plus proche parmi ceux qui menent *en avant*. Sans ce filtre,
  // en arrivant dans une zone on se tient a cote du portail de retour : la
  // boussole designait donc le chemin par lequel on venait d'entrer.
  const onward = G.map.portals.filter((p) => p.to_name !== G.prevZone);
  const candidates = onward.length ? onward : G.map.portals;
  let best = null, bestD = Infinity;
  for (const p of candidates) {
    const d = Math.hypot(p.x + 0.5 - G.predict.x, p.y + 0.5 - G.predict.y);
    if (d < bestD) { bestD = d; best = p; }
  }
  if (!best) return;
  const s = worldToScreen(best.x + 0.5, best.y + 0.5);
  const m = 74;
  const onScreen = s.x > m && s.x < canvas.width - m && s.y > m && s.y < canvas.height - m;
  if (onScreen) return;   // visible : le portail se signale deja tout seul

  // Point d'ancrage sur le bord, dans la direction du portail.
  const cx = canvas.width / 2, cy = canvas.height / 2;
  const ang = Math.atan2(s.y - cy, s.x - cx);
  const rx = canvas.width / 2 - m, ry = canvas.height / 2 - m;
  const k = Math.min(Math.abs(rx / Math.cos(ang)), Math.abs(ry / Math.sin(ang)));
  const ax = cx + Math.cos(ang) * k, ay = cy + Math.sin(ang) * k;

  const pulse = 0.5 + 0.5 * Math.sin(performance.now() / 340);
  ctx.save();
  ctx.translate(ax, ay);
  ctx.rotate(ang);
  ctx.fillStyle = "rgba(240,200,80," + (0.55 + 0.45 * pulse) + ")";
  ctx.strokeStyle = "rgba(30,24,8,0.8)"; ctx.lineWidth = 1.5;
  ctx.beginPath();
  ctx.moveTo(16, 0); ctx.lineTo(-8, -11); ctx.lineTo(-3, 0); ctx.lineTo(-8, 11);
  ctx.closePath(); ctx.fill(); ctx.stroke();
  ctx.restore();

  const label = best.to_name + " \u2014 " + Math.round(bestD) + " m";
  ctx.font = "bold 12px Segoe UI";
  ctx.textAlign = "center";
  const w = ctx.measureText(label).width + 16;
  const ly = ay + (Math.sin(ang) > 0 ? 28 : -30);
  ctx.fillStyle = "rgba(8,8,12,0.82)";
  roundRect(ctx, ax - w / 2, ly - 14, w, 20, 5); ctx.fill();
  ctx.strokeStyle = "rgba(240,200,80,0.6)"; ctx.lineWidth = 1; ctx.stroke();
  ctx.fillStyle = "#f0c850";
  ctx.fillText(label, ax, ly);
  ctx.textAlign = "start";
}

function drawMinimap() {
  const W = mini.width, H = mini.height, scale = G.mapBig ? 6 : 3;
  mctx.clearRect(0, 0, W, H);
  mctx.fillStyle = "rgba(8,8,12,0.85)";
  mctx.fillRect(0, 0, W, H);
  const toMini = (wx, wy) => ({ x: W / 2 + (wx - G.cam.x) * scale, y: H / 2 + (wy - G.cam.y) * scale });

  // Fond : murs et portails de la zone, pour que la minimap serve à s'orienter.
  if (G.map) {
    const span = Math.ceil(Math.max(W, H) / (2 * scale)) + 1;
    const cx = Math.round(G.cam.x), cy = Math.round(G.cam.y);
    for (let y = cy - span; y <= cy + span; y++) {
      for (let x = cx - span; x <= cx + span; x++) {
        const t = tileAt(x, y);
        if (t === TILE.VOID) continue;
        const p = toMini(x, y);
        if (p.x < 0 || p.x > W || p.y < 0 || p.y > H) continue;
        if (t === TILE.WALL) mctx.fillStyle = "#39414f";
        else if (t === TILE.EXIT) mctx.fillStyle = "#f0c850";
        else mctx.fillStyle = "rgba(90,105,130,0.30)";
        mctx.fillRect(p.x, p.y, scale, scale);
      }
    }
  }

  // Decor : points ternes, juste assez pour reconnaitre la place du marche.
  if (G.map) {
    mctx.fillStyle = "rgba(150,140,110,0.55)";
    for (const d of G.map.decors) {
      const p = toMini(d.x + 0.5, d.y + 0.5);
      if (p.x < 0 || p.x > W || p.y < 0 || p.y > H) continue;
      mctx.fillRect(p.x - 1, p.y - 1, 2, 2);
    }
    // Obstacles : plus gros que le decor, ce sont eux qui structurent la zone.
    mctx.fillStyle = "rgba(150,158,175,0.75)";
    for (const o of G.map.obstacles) {
      const q = toMini(o.x, o.y);
      if (q.x < 0 || q.x > W || q.y < 0 || q.y > H) continue;
      mctx.fillRect(q.x - 1.5, q.y - 1.5, 3, 3);
    }
    // Portails : le repere le plus important de la minimap.
    const pulse = 0.5 + 0.5 * Math.sin(performance.now() / 340);
    for (const po of G.map.portals) {
      const p = toMini(po.x + 0.5, po.y + 0.5);
      if (p.x < -6 || p.x > W + 6 || p.y < -6 || p.y > H + 6) continue;
      mctx.fillStyle = "rgba(240,200,80," + (0.5 + 0.5 * pulse) + ")";
      mctx.beginPath(); mctx.arc(p.x, p.y, 3.5, 0, Math.PI * 2); mctx.fill();
    }
  }

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
  // Nouvelle partie : on ne vient de nulle part, la boussole ne doit pas
  // exclure un portail a cause de la zone d'une partie precedente.
  G.zoneName = null; G.prevZone = null;
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
  const w = screenToWorld(G.mouse.sx, G.mouse.sy);
  G.mouse.wx = w.x; G.mouse.wy = w.y;
  // Surbrillance de l'ennemi ou de l'objet survolé + curseur contextuel.
  G.hoverEntity = pickEnemy(w.x, w.y);
  G.hoverItem = G.hoverEntity ? null : pickGroundItem(w.x, w.y, ITEM_PICK_RADIUS);
  canvas.style.cursor = G.hoverEntity ? "crosshair" : (G.hoverItem ? "pointer" : "default");
});

/** Ennemi le plus proche du point monde, dans PICK_RADIUS. */
function pickEnemy(wx, wy) {
  let best = PICK_RADIUS, found = null;
  for (const ent of G.entities.values()) {
    if (ent.kind !== "Mob" && ent.kind !== "Boss") continue;
    if (ent.hp <= 0) continue;
    const d = Math.hypot(ent.x - wx, ent.y - wy);
    if (d < best) { best = d; found = ent; }
  }
  return found;
}

/** Objet au sol le plus proche d'un point monde, dans `radius`. */
function pickGroundItem(wx, wy, radius) {
  let best = radius, found = null;
  for (const it of G.ground.values()) {
    const d = Math.hypot(it.x - wx, it.y - wy);
    if (d < best) { best = d; found = it; }
  }
  return found;
}

/**
 * Poursuit l'objet visé et le ramasse dès qu'on est à portée.
 *
 * Le serveur n'a aucun ramassage automatique par proximité : sans cet envoi
 * explicite de `PickupItem`, le butin restait au sol pour toujours.
 */
function stepPickup() {
  if (G.pickupTarget == null) return;
  const it = G.ground.get(G.pickupTarget);
  if (!it) { G.pickupTarget = null; return; }   // déjà ramassé (par nous ou un autre)

  const d = Math.hypot(it.x - G.predict.x, it.y - G.predict.y);
  if (d > PICKUP_RANGE) {
    sendMove(it.x, it.y, false);
    return;
  }
  const now = performance.now();
  if (now - G.lastPickupSent < PICKUP_SEND_MS) return;
  G.lastPickupSent = now;
  send({ type: "PickupItem", ground_item_id: G.pickupTarget });
}

/** Ramasse l'objet au sol le plus proche du joueur (touche A). */
function grabNearest() {
  const it = pickGroundItem(G.predict.x, G.predict.y, 6.0);
  if (it) {
    G.pickupTarget = it.id;
    G.clickMark = { x: it.x, y: it.y, t: performance.now() };
  } else {
    pushChat("Rien à ramasser à proximité.", true);
  }
}

/**
 * Direction voulue par les touches maintenues, en coordonnees monde.
 *
 * On lit `e.code` (touche *physique*) et non `e.key` (caractere produit) : la
 * meme rangee de touches est WASD en QWERTY et ZQSD en AZERTY. Un seul jeu de
 * codes couvre donc les deux dispositions, sans demander sa langue au joueur.
 *
 * L'ecran est isometrique : « haut » n'est pas -y monde. On inverse la
 * projection (`worldToScreen`) pour que chaque touche pousse le personnage dans
 * la direction ou elle pointe *a l'ecran*, seule lecture intuitive.
 */
function keyboardDir() {
  const k = G.keys;
  let sx = 0, sy = 0;
  if (k.has("KeyW") || k.has("ArrowUp")) sy -= 1;
  if (k.has("KeyS") || k.has("ArrowDown")) sy += 1;
  if (k.has("KeyA") || k.has("ArrowLeft")) sx -= 1;
  if (k.has("KeyD") || k.has("ArrowRight")) sx += 1;
  if (sx === 0 && sy === 0) return null;

  // Inverse de : isoX = (wx-wy)*HALF_W ; isoY = (wx+wy)*HALF_H
  const wx = 0.5 * (sx / HALF_W + sy / HALF_H);
  const wy = 0.5 * (sy / HALF_H - sx / HALF_W);
  const n = Math.hypot(wx, wy);
  return n < 1e-6 ? null : { x: wx / n, y: wy / n };
}

/**
 * Fait avancer le deplacement clavier. Renvoie `true` s'il a la main ce frame.
 *
 * Au relachement on vise sa propre case : le serveur lit cela comme un arret
 * explicite. Sans cela le personnage continuait jusqu'au dernier point envoye,
 * soit quatre tuiles de derapage a chaque fois qu'on lache une touche.
 */
function stepKeyboardMove() {
  const dir = keyboardDir();
  if (!dir) {
    if (G.keyMoving) {
      G.keyMoving = false;
      send({ type: "MoveIntent", target_x: G.predict.x, target_y: G.predict.y });
      G.predict.moving = false;
    }
    return false;
  }
  G.keyMoving = true;
  // Le clavier reprend la main sur les poursuites en cours.
  G.pickupTarget = null;
  G.holdMove = false;

  // Viser le plus loin possible sans franchir de mur : le serveur ignore
  // purement et simplement une intention dont la case d'arrivee est bloquee,
  // et le joueur croirait la touche morte.
  for (let L = KEY_LOOKAHEAD; L >= 1; L--) {
    const tx = G.predict.x + dir.x * L, ty = G.predict.y + dir.y * L;
    if (!G.map || isWalkable(Math.floor(tx), Math.floor(ty))) {
      sendMove(tx, ty, false);
      return true;
    }
  }
  return true;   // face a un mur : on reste sur place, mais le clavier a la main
}

/** Envoie une intention de déplacement, au plus une fois par MOVE_SEND_MS. */
function sendMove(wx, wy, force) {
  const now = performance.now();
  if (!force && now - G.lastMoveSent < MOVE_SEND_MS) return;
  G.lastMoveSent = now;
  G.predict.tx = wx; G.predict.ty = wy;
  G.predict.moving = true; G.predict.init = true;
  send({ type: "MoveIntent", target_x: wx, target_y: wy });
}

canvas.addEventListener("mousedown", (e) => {
  if (G.phase !== "playing") return;
  const r = canvas.getBoundingClientRect();
  const w = screenToWorld(e.clientX - r.left, e.clientY - r.top);

  if (e.button === 0) {
    // Clic gauche sur un ennemi : on ATTAQUE. Auparavant le clic gauche
    // déplaçait toujours, ce qui rendait le combat pratiquement inaccessible.
    const foe = pickEnemy(w.x, w.y);
    if (foe) {
      G.target = foe.id;
      G.pickupTarget = null;
      send({ type: "AttackTarget", entity_id: foe.id });
      return;
    }
    // Clic sur un objet au sol : on va le chercher et on le ramasse.
    const item = pickGroundItem(w.x, w.y, ITEM_PICK_RADIUS);
    if (item) {
      G.pickupTarget = item.id;
      G.clickMark = { x: item.x, y: item.y, t: performance.now() };
      return;
    }
    // Sinon : déplacement, et le maintien poursuit le curseur.
    G.pickupTarget = null;
    G.holdMove = true;
    G.clickMark = { x: w.x, y: w.y, t: performance.now() };
    sendMove(w.x, w.y, true);
  } else if (e.button === 2) {
    // Clic droit : attaque explicite, même geste qu'avant mais plus tolérant.
    const foe = pickEnemy(w.x, w.y);
    if (foe) {
      G.target = foe.id;
      send({ type: "AttackTarget", entity_id: foe.id });
    }
  }
});

window.addEventListener("mouseup", (e) => {
  if (e.button === 0) G.holdMove = false;
});
// Le curseur qui sort de la fenêtre ne doit pas laisser le joueur en marche forcée.
window.addEventListener("blur", () => { G.holdMove = false; });

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

  // Deplacement : touche physique, donc ZQSD en AZERTY et WASD en QWERTY.
  if (MOVE_CODES.has(e.code)) { G.keys.add(e.code); e.preventDefault(); return; }

  // Competences par code physique aussi : en AZERTY la rangee des chiffres
  // produit &, é, ", ' ... sans Maj, et `e.key >= "1"` n'y voyait jamais un
  // chiffre — la barre de competences etait donc inerte sur clavier francais.
  if (/^Digit[1-9]$/.test(e.code)) { useSkill(parseInt(e.code.slice(5), 10)); return; }
  if (e.key >= "1" && e.key <= "9") { useSkill(parseInt(e.key, 10)); return; }

  // `F` (et non `A`) pour ramasser : en QWERTY, `A` est aussi la touche
  // « gauche ». Le code physique reste le meme sur les deux dispositions.
  if (e.code === "KeyF") { grabNearest(); return; }
  if (e.code === "KeyI") {
    G.invOpen = !G.invOpen;
    $("inventory").classList.toggle("hidden", !G.invOpen);
    if (G.invOpen) renderInventory();
    return;
  }

  switch (e.key) {
    case "Enter": {
      const inp = $("chat-input");
      inp.classList.remove("hidden"); inp.focus(); G.chatOpen = true;
      G.keys.clear();   // sinon on continue de marcher en tapant son message
      e.preventDefault();
      break;
    }
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

// Codes des touches de deplacement. `KeyW/A/S/D` sont physiquement Z/Q/S/D
// sur un clavier AZERTY : une seule liste couvre WASD et ZQSD.
const MOVE_CODES = new Set([
  "KeyW", "KeyA", "KeyS", "KeyD",
  "ArrowUp", "ArrowLeft", "ArrowDown", "ArrowRight",
]);

window.addEventListener("keyup", (e) => {
  if (MOVE_CODES.has(e.code)) G.keys.delete(e.code);
});
// Alt-Tab, clic hors fenetre, ouverture du chat : sans cela une touche relachee
// hors focus reste « enfoncee » et le personnage part tout seul.
window.addEventListener("blur", () => G.keys.clear());

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
