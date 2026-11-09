//! @id     origin_web_presentation
//! @do     generer_pages_presentation
//! @role   ui
//! @layer  app
//! @human  origin — pages « Fonctionnement » (schémas conceptuels) et « Souveraineté & conformité ».

//! Pages de présentation non commerciales du site Origin.
//!
//! Le contenu suit la doc conceptuelle gelée et `docs/conformite/` : on y écrit ce
//! que COG fait, ce qu'il vise, et ce qui manque encore — jamais une conformité
//! acquise qu'aucun document ne prouve.

use super::pages::layout;

/// Styles propres aux deux pages : schémas, tableaux d'état, encadrés.
const STYLES: &str = r#"
<style>
  .pr-lead { max-width: 760px; margin: 0 auto; color: var(--text-muted); font-size: 1.1rem; }
  .pr-toc { display: flex; flex-wrap: wrap; gap: .5rem; justify-content: center; margin-top: 1.5rem; }
  .pr-toc a { border: 1px solid var(--border); border-radius: 999px; padding: .35rem .9rem; font-size: .9rem; color: var(--text); }
  .pr-toc a:hover { border-color: var(--primary); }
  .diag { margin: 1.5rem 0 0; background: var(--bg-surface); border: 1px solid var(--border); border-radius: 1rem; padding: 1.25rem; }
  .diag svg { width: 100%; height: auto; display: block; }
  .diag figcaption { margin-top: .75rem; color: var(--text-muted); font-size: .9rem; text-align: center; }
  .pr-points { list-style: none; margin-top: 1.25rem; display: grid; gap: .6rem; }
  .pr-points li { padding-left: 1.4rem; position: relative; }
  .pr-points li::before { content: "›"; position: absolute; left: .2rem; color: var(--secondary); font-weight: 700; }
  .pr-table { width: 100%; border-collapse: collapse; margin-top: 1rem; font-size: .95rem; }
  .pr-table th, .pr-table td { padding: .7rem .8rem; border-bottom: 1px solid var(--border); text-align: left; vertical-align: top; }
  .pr-table th { color: var(--text-muted); font-weight: 600; font-size: .8rem; text-transform: uppercase; letter-spacing: .05em; }
  .pr-scroll { overflow-x: auto; }
  .st { display: inline-block; border-radius: 999px; padding: .1rem .6rem; font-size: .8rem; font-weight: 600; white-space: nowrap; }
  .st-ok { background: rgba(16,185,129,.15); color: var(--success); }
  .st-part { background: rgba(245,158,11,.15); color: var(--warning); }
  .st-no { background: rgba(239,68,68,.15); color: var(--error); }
  .st-na { background: rgba(156,163,175,.15); color: var(--text-muted); }
  .pr-note { border-left: 3px solid var(--warning); background: rgba(245,158,11,.06); padding: 1rem 1.25rem; border-radius: .5rem; margin-top: 1.5rem; color: var(--text-muted); }
  .pr-note strong { color: var(--text); }
  .pr-specs { display: grid; gap: 1rem; grid-template-columns: repeat(auto-fit, minmax(320px, 1fr)); margin-top: 1.5rem; }
  .pr-specs.pr-4 { grid-template-columns: repeat(auto-fit, minmax(230px, 1fr)); }
  .pr-spec { background: var(--bg-surface); border: 1px solid var(--border); border-radius: .75rem; padding: 1.25rem; }
  .pr-spec h3 { font-size: 1.05rem; margin-bottom: .4rem; }
  .pr-spec p { color: var(--text-muted); font-size: .95rem; }
  .pr-next { display: flex; flex-wrap: wrap; gap: 1rem; justify-content: center; margin-top: 2rem; }
</style>
"#;

/// Page « Fonctionnement » : les particularités de COG et les schémas conceptuels.
pub fn fonctionnement_page() -> String {
    let content = format!(
        r##"{STYLES}
<section class="hero" style="padding: 3rem 0 2rem;">
  <h1>Comment fonctionne Miyukini COG</h1>
  <p class="pr-lead">Un seul système, installé chez vous, où chaque action passe par des règles explicites.
  Cette page présente ce qui le distingue d'une suite logicielle classique, schémas à l'appui.</p>
  <nav class="pr-toc" aria-label="Sommaire">
    <a href="#particularites">Particularités</a>
    <a href="#strates">Strates</a>
    <a href="#action">Le chemin d'une action</a>
    <a href="#cores">Les 10 Cores</a>
    <a href="#zones">Zones réseau</a>
    <a href="#identite">Identité</a>
    <a href="#federation">Fédération</a>
    <a href="#maia">MAIA</a>
  </nav>
</section>

<section class="section" id="particularites">
  <h2 class="section-title">Ce qui rend COG particulier</h2>
  <p class="section-subtitle">Six choix de conception, fixés avant d'écrire le code</p>
  <div class="pr-specs">
    <div class="pr-spec"><h3>Un système, pas une collection d'apps</h3><p>Agenda, comptabilité, documents, cloud, bureautique, domotique et IA partagent la même identité, les mêmes règles et le même journal.</p></div>
    <div class="pr-spec"><h3>Gouverné, refus par défaut</h3><p>Aucun service n'écrit directement. Il émet une intention ; un Core décide, un autre est seul à écrire. Rien n'est implicite.</p></div>
    <div class="pr-spec"><h3>Local d'abord</h3><p>Les données vivent sur votre machine, chiffrées, une clé par base. L'isolement hors ligne est un état normal, pas une panne.</p></div>
    <div class="pr-spec"><h3>Une zone qui ne sort jamais</h3><p>Le cœur du COG n'ouvre aucune connexion vers l'extérieur : il tire ce dont il a besoin par des voies déclarées.</p></div>
    <div class="pr-spec"><h3>Des signatures, pas des secrets partagés</h3><p>Personnes, appareils et services sont identifiés par des titres signés (passeports, visas), révocables publiquement.</p></div>
    <div class="pr-spec"><h3>Fédérable sans centre</h3><p>Plusieurs COG peuvent coopérer comme des pays souverains ; la fédération fonctionne même sans Origin.</p></div>
  </div>
</section>

<section class="section" id="strates">
  <h2 class="section-title">Une architecture en strates</h2>
  <p class="section-subtitle">Chaque strate ne parle qu'à ses voisines ; les règles sont en bas, les usages en haut</p>
  <figure class="diag">
    <svg viewBox="0 0 720 330" role="img" aria-labelledby="d1t">
      <title id="d1t">Strates de COG : Kernel, Cores, Toolkits, Services, Administration</title>
      <g font-family="Inter, sans-serif" text-anchor="middle">
        <rect x="250" y="10" width="220" height="50" rx="8" fill="#1a1a25" stroke="#9ca3af"/>
        <text x="360" y="32" fill="#f0f0f5" font-size="15" font-weight="600">Administration</text>
        <text x="360" y="50" fill="#9ca3af" font-size="11">pilotage humain, cadré par TAMR</text>
        <rect x="190" y="72" width="340" height="50" rx="8" fill="#1a1a25" stroke="#06b6d4"/>
        <text x="360" y="94" fill="#f0f0f5" font-size="15" font-weight="600">Services</text>
        <text x="360" y="112" fill="#9ca3af" font-size="11">JayKoa, JayKonta, MiyuCloud, Jay Bureau, MAIA, Alicia…</text>
        <rect x="130" y="134" width="460" height="50" rx="8" fill="#1a1a25" stroke="#10b981"/>
        <text x="360" y="156" fill="#f0f0f5" font-size="15" font-weight="600">Toolkits</text>
        <text x="360" y="174" fill="#9ca3af" font-size="11">briques réutilisables qui exécutent : calcul, export, protocoles</text>
        <rect x="70" y="196" width="580" height="54" rx="8" fill="rgba(139,92,246,.15)" stroke="#8b5cf6" stroke-width="2"/>
        <text x="360" y="219" fill="#f0f0f5" font-size="16" font-weight="700">10 Cores de gouvernance</text>
        <text x="360" y="238" fill="#c4b5fd" font-size="11">décident, persistent, surveillent, tracent — aucun service ne les contourne</text>
        <rect x="10" y="262" width="700" height="50" rx="8" fill="#12121a" stroke="#9ca3af" stroke-dasharray="5 4"/>
        <text x="360" y="284" fill="#f0f0f5" font-size="15" font-weight="600">Kernel</text>
        <text x="360" y="302" fill="#9ca3af" font-size="11">système, stockage chiffré (SQLCipher), profils AppArmor</text>
      </g>
    </svg>
    <figcaption>Plus on descend, moins il y a de code et plus il est contrôlé : les Cores forment la couche de règles commune à tous les services.</figcaption>
  </figure>
</section>

<section class="section" id="action">
  <h2 class="section-title">Le chemin d'une action</h2>
  <p class="section-subtitle">Proposer, décider et écrire sont trois rôles distincts, tenus par des composants différents</p>
  <figure class="diag">
    <svg viewBox="0 0 760 330" role="img" aria-labelledby="d2t">
      <title id="d2t">Une intention passe par StrongFather qui décide, puis par KindMother, seul écrivain</title>
      <defs>
        <marker id="ar" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#9ca3af"/></marker>
        <marker id="arg" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#10b981"/></marker>
        <marker id="arr" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#ef4444"/></marker>
      </defs>
      <g font-family="Inter, sans-serif" text-anchor="middle">
        <rect x="10" y="120" width="140" height="70" rx="10" fill="#1a1a25" stroke="#06b6d4"/>
        <text x="80" y="150" fill="#f0f0f5" font-size="14" font-weight="600">Service ou MAIA</text>
        <text x="80" y="170" fill="#9ca3af" font-size="11">propose</text>
        <line x1="150" y1="155" x2="208" y2="155" stroke="#9ca3af" stroke-width="2" marker-end="url(#ar)"/>
        <text x="179" y="145" fill="#9ca3af" font-size="11">intention</text>
        <rect x="212" y="120" width="150" height="70" rx="10" fill="rgba(139,92,246,.15)" stroke="#8b5cf6" stroke-width="2"/>
        <text x="287" y="150" fill="#f0f0f5" font-size="14" font-weight="700">StrongFather</text>
        <text x="287" y="170" fill="#c4b5fd" font-size="11">décide · refus par défaut</text>
        <line x1="362" y1="155" x2="430" y2="155" stroke="#10b981" stroke-width="2" marker-end="url(#arg)"/>
        <text x="396" y="145" fill="#10b981" font-size="11">autorisé</text>
        <rect x="434" y="120" width="150" height="70" rx="10" fill="rgba(139,92,246,.15)" stroke="#8b5cf6" stroke-width="2"/>
        <text x="509" y="150" fill="#f0f0f5" font-size="14" font-weight="700">KindMother</text>
        <text x="509" y="170" fill="#c4b5fd" font-size="11">seul écrivain</text>
        <line x1="584" y1="155" x2="628" y2="155" stroke="#9ca3af" stroke-width="2" marker-end="url(#ar)"/>
        <path d="M640,130 a55,12 0 0 1 110,0 v52 a55,12 0 0 1 -110,0 z" fill="#12121a" stroke="#10b981"/>
        <ellipse cx="695" cy="130" rx="55" ry="12" fill="#1a1a25" stroke="#10b981"/>
        <text x="695" y="162" fill="#f0f0f5" font-size="12" font-weight="600">bases chiffrées</text>
        <text x="695" y="178" fill="#9ca3af" font-size="11">+ journal chaîné</text>
        <line x1="287" y1="190" x2="287" y2="250" stroke="#ef4444" stroke-width="2" marker-end="url(#arr)"/>
        <text x="330" y="226" fill="#ef4444" font-size="11">refusé</text>
        <rect x="212" y="254" width="150" height="50" rx="10" fill="rgba(239,68,68,.08)" stroke="#ef4444"/>
        <text x="287" y="276" fill="#f0f0f5" font-size="13" font-weight="600">Rien n'est écrit</text>
        <text x="287" y="293" fill="#9ca3af" font-size="11">le refus est tracé</text>
        <rect x="212" y="12" width="150" height="56" rx="10" fill="#1a1a25" stroke="#f59e0b" stroke-dasharray="5 4"/>
        <text x="287" y="36" fill="#f0f0f5" font-size="13" font-weight="600">TAMR</text>
        <text x="287" y="54" fill="#9ca3af" font-size="11">validation humaine</text>
        <line x1="287" y1="68" x2="287" y2="116" stroke="#f59e0b" stroke-width="2" stroke-dasharray="5 4" marker-end="url(#ar)"/>
        <text x="300" y="96" fill="#f59e0b" font-size="11" text-anchor="start">décisions sensibles</text>
        <rect x="434" y="254" width="150" height="50" rx="10" fill="#1a1a25" stroke="#9ca3af" stroke-dasharray="5 4"/>
        <text x="509" y="276" fill="#f0f0f5" font-size="13" font-weight="600">CaringNanny</text>
        <text x="509" y="293" fill="#9ca3af" font-size="11">observe, ne modifie rien</text>
        <line x1="584" y1="279" x2="660" y2="200" stroke="#9ca3af" stroke-dasharray="4 4"/>
      </g>
    </svg>
    <figcaption>Ni un service ni l'IA n'écrivent eux-mêmes : ils demandent. Ce qui n'est pas explicitement autorisé est refusé.</figcaption>
  </figure>
</section>

<section class="section" id="cores">
  <h2 class="section-title">Les 10 Cores</h2>
  <p class="section-subtitle">Chacun a un seul rôle, et l'interdiction explicite de faire celui des autres</p>
  <div class="pr-scroll">
  <table class="pr-table">
    <thead><tr><th>Core</th><th>Rôle</th></tr></thead>
    <tbody>
      <tr><td><strong>StrongFather</strong></td><td>Décide (autorise ou refuse) — n'exécute ni ne persiste</td></tr>
      <tr><td><strong>KindMother</strong></td><td>Persiste — seul validateur des écritures, ne décide jamais</td></tr>
      <tr><td><strong>BondingBrother</strong></td><td>Passerelle — traduit, filtre, transmet</td></tr>
      <tr><td><strong>BorderGuard</strong></td><td>Définit les frontières et les niveaux de confiance</td></tr>
      <tr><td><strong>WorrySentinel</strong></td><td>Gouverne la sécurité (états T0 à T4)</td></tr>
      <tr><td><strong>CaringNanny</strong></td><td>Observe l'état, ne modifie rien</td></tr>
      <tr><td><strong>MasterButler</strong></td><td>Registre des capacités et permissions</td></tr>
      <tr><td><strong>LogisticsSteward</strong></td><td>Arbitre ressources et quotas</td></tr>
      <tr><td><strong>EverBuddy</strong></td><td>Cycle de vie, historique immuable</td></tr>
      <tr><td><strong>TAMR</strong></td><td>Cadre de l'intervention humaine, traçabilité absolue</td></tr>
    </tbody>
  </table>
  </div>
</section>

<section class="section" id="zones">
  <h2 class="section-title">Les zones réseau</h2>
  <p class="section-subtitle">Rien n'entre sans contrôle, rien ne sort de la zone COG</p>
  <figure class="diag">
    <svg viewBox="0 0 760 340" role="img" aria-labelledby="d3t">
      <title id="d3t">Extérieur, Douane, Zone franche, Portail, Zone COG</title>
      <defs>
        <marker id="az" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#9ca3af"/></marker>
        <marker id="azc" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#8b5cf6"/></marker>
      </defs>
      <g font-family="Inter, sans-serif" text-anchor="middle">
        <text x="75" y="24" fill="#9ca3af" font-size="12" font-weight="600">EXTÉRIEUR</text>
        <rect x="10" y="40" width="130" height="44" rx="8" fill="#1a1a25" stroke="#9ca3af"/><text x="75" y="67" fill="#f0f0f5" font-size="13">Internet</text>
        <rect x="10" y="100" width="130" height="44" rx="8" fill="#1a1a25" stroke="#9ca3af"/><text x="75" y="120" fill="#f0f0f5" font-size="13">Fédération</text><text x="75" y="136" fill="#9ca3af" font-size="10">autres COG</text>
        <rect x="10" y="160" width="130" height="44" rx="8" fill="#1a1a25" stroke="#9ca3af"/><text x="75" y="180" fill="#f0f0f5" font-size="13">Origin</text><text x="75" y="196" fill="#9ca3af" font-size="10">canal officiel</text>
        <line x1="140" y1="62" x2="186" y2="120" stroke="#9ca3af" marker-end="url(#az)"/>
        <line x1="140" y1="122" x2="186" y2="122" stroke="#9ca3af" marker-end="url(#az)"/>
        <line x1="140" y1="182" x2="186" y2="126" stroke="#9ca3af" marker-end="url(#az)"/>
        <rect x="190" y="30" width="44" height="190" rx="8" fill="rgba(239,68,68,.12)" stroke="#ef4444"/>
        <text x="212" y="128" fill="#f0f0f5" font-size="13" font-weight="700" transform="rotate(-90 212 128)">Douane</text>
        <text x="212" y="244" fill="#9ca3af" font-size="10">pare-feu, IDS</text>
        <line x1="234" y1="125" x2="262" y2="125" stroke="#9ca3af" marker-end="url(#az)"/>
        <rect x="266" y="30" width="190" height="190" rx="10" fill="rgba(245,158,11,.06)" stroke="#f59e0b" stroke-dasharray="6 4"/>
        <text x="361" y="52" fill="#f59e0b" font-size="12" font-weight="600">ZONE FRANCHE</text>
        <rect x="286" y="68" width="150" height="56" rx="8" fill="#1a1a25" stroke="#f59e0b"/><text x="361" y="92" fill="#f0f0f5" font-size="13">Façades web</text><text x="361" y="110" fill="#9ca3af" font-size="10">base de session seule</text>
        <rect x="286" y="140" width="150" height="56" rx="8" fill="#1a1a25" stroke="#f59e0b"/><text x="361" y="164" fill="#f0f0f5" font-size="13">Quarantaine</text><text x="361" y="182" fill="#9ca3af" font-size="10">fichiers désarmés (CDR)</text>
        <rect x="484" y="30" width="44" height="190" rx="8" fill="rgba(6,182,212,.12)" stroke="#06b6d4"/>
        <text x="506" y="128" fill="#f0f0f5" font-size="13" font-weight="700" transform="rotate(-90 506 128)">Portail</text>
        <text x="506" y="244" fill="#9ca3af" font-size="10">voies déclarées</text>
        <line x1="556" y1="125" x2="532" y2="125" stroke="#8b5cf6" stroke-width="2" marker-end="url(#azc)"/>
        <text x="544" y="112" fill="#c4b5fd" font-size="10">tire</text>
        <rect x="556" y="30" width="194" height="190" rx="10" fill="rgba(139,92,246,.10)" stroke="#8b5cf6" stroke-width="2"/>
        <text x="653" y="52" fill="#c4b5fd" font-size="12" font-weight="600">ZONE COG</text>
        <rect x="574" y="64" width="158" height="38" rx="8" fill="#1a1a25" stroke="#8b5cf6"/><text x="653" y="88" fill="#f0f0f5" font-size="13">Cores</text>
        <rect x="574" y="110" width="158" height="38" rx="8" fill="#1a1a25" stroke="#8b5cf6"/><text x="653" y="134" fill="#f0f0f5" font-size="13">Services, MAIA</text>
        <rect x="574" y="156" width="158" height="44" rx="8" fill="#1a1a25" stroke="#10b981"/><text x="653" y="176" fill="#f0f0f5" font-size="13">Bases principales</text><text x="653" y="192" fill="#9ca3af" font-size="10">chiffrées</text>
        <rect x="150" y="268" width="600" height="56" rx="10" fill="#12121a" stroke="#ef4444" stroke-dasharray="5 4"/>
        <text x="450" y="292" fill="#f0f0f5" font-size="13" font-weight="600">Aucune connexion sortante depuis la zone COG</text>
        <text x="450" y="311" fill="#9ca3af" font-size="11">une sortie bloquée est un incident critique · les Vigies coupent une voie anormale ou muette</text>
      </g>
    </svg>
    <figcaption>Le cœur ne s'expose jamais : ce qui vient de l'extérieur est filtré, désarmé, puis récupéré à l'initiative de la zone COG.</figcaption>
  </figure>
</section>

<section class="section" id="identite">
  <h2 class="section-title">Identité : des titres signés</h2>
  <p class="section-subtitle">Comme une administration : une racine, des passeports, des visas</p>
  <figure class="diag">
    <svg viewBox="0 0 760 230" role="img" aria-labelledby="d4t">
      <title id="d4t">La racine du COG délivre des passeports, qui reçoivent des visas</title>
      <defs><marker id="ai" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#9ca3af"/></marker></defs>
      <g font-family="Inter, sans-serif" text-anchor="middle">
        <rect x="10" y="30" width="200" height="80" rx="10" fill="rgba(139,92,246,.15)" stroke="#8b5cf6" stroke-width="2"/>
        <text x="110" y="62" fill="#f0f0f5" font-size="15" font-weight="700">Racine du COG</text>
        <text x="110" y="84" fill="#c4b5fd" font-size="11">clé hors ligne</text>
        <line x1="210" y1="70" x2="276" y2="70" stroke="#9ca3af" stroke-width="2" marker-end="url(#ai)"/>
        <text x="243" y="60" fill="#9ca3af" font-size="11">délivre</text>
        <rect x="280" y="30" width="200" height="80" rx="10" fill="#1a1a25" stroke="#06b6d4"/>
        <text x="380" y="62" fill="#f0f0f5" font-size="15" font-weight="700">Passeport</text>
        <text x="380" y="84" fill="#9ca3af" font-size="11">une personne ou un appareil</text>
        <line x1="480" y1="70" x2="546" y2="70" stroke="#9ca3af" stroke-width="2" marker-end="url(#ai)"/>
        <text x="513" y="60" fill="#9ca3af" font-size="11">obtient</text>
        <rect x="550" y="30" width="200" height="80" rx="10" fill="#1a1a25" stroke="#10b981"/>
        <text x="650" y="62" fill="#f0f0f5" font-size="15" font-weight="700">Visa</text>
        <text x="650" y="84" fill="#9ca3af" font-size="11">l'accès à un service précis</text>
        <rect x="10" y="140" width="360" height="70" rx="10" fill="#12121a" stroke="#9ca3af" stroke-dasharray="5 4"/>
        <text x="190" y="168" fill="#f0f0f5" font-size="13" font-weight="600">Chaque titre est un Sceau signé</text>
        <text x="190" y="190" fill="#9ca3af" font-size="11">Ed25519, multi-signature possible, aucun mot de passe partagé</text>
        <rect x="390" y="140" width="360" height="70" rx="10" fill="#12121a" stroke="#9ca3af" stroke-dasharray="5 4"/>
        <text x="570" y="168" fill="#f0f0f5" font-size="13" font-weight="600">Révocations publiques et vérifiables</text>
        <text x="570" y="190" fill="#9ca3af" font-size="11">registres Merkle · un identifiant différent par COG visité</text>
      </g>
    </svg>
    <figcaption>Les mises à jour suivent la même logique : un canal officiel signé deux fois sur deux, relevé par vous, jamais poussé de force.</figcaption>
  </figure>
</section>

<section class="section" id="federation">
  <h2 class="section-title">La fédération</h2>
  <p class="section-subtitle">Des COG qui se reconnaissent entre eux, sans autorité centrale</p>
  <figure class="diag">
    <svg viewBox="0 0 760 250" role="img" aria-labelledby="d5t">
      <title id="d5t">Cycle d'un membre de la fédération : candidat, invité, membre, suspendu, révoqué</title>
      <defs>
        <marker id="af" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#9ca3af"/></marker>
        <marker id="afg" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#10b981"/></marker>
      </defs>
      <g font-family="Inter, sans-serif" text-anchor="middle">
        <rect x="10" y="90" width="120" height="50" rx="25" fill="#1a1a25" stroke="#9ca3af"/><text x="70" y="120" fill="#f0f0f5" font-size="14" font-weight="600">Candidat</text>
        <line x1="130" y1="115" x2="196" y2="115" stroke="#9ca3af" stroke-width="2" marker-end="url(#af)"/>
        <text x="163" y="106" fill="#9ca3af" font-size="10">≥ 2 membres</text>
        <rect x="200" y="90" width="120" height="50" rx="25" fill="#1a1a25" stroke="#06b6d4"/><text x="260" y="120" fill="#f0f0f5" font-size="14" font-weight="600">Invité</text>
        <line x1="320" y1="115" x2="396" y2="115" stroke="#9ca3af" stroke-width="2" marker-end="url(#af)"/>
        <text x="358" y="100" fill="#9ca3af" font-size="10">≥ 4 membres</text>
        <text x="358" y="134" fill="#9ca3af" font-size="10">pendant 7 jours</text>
        <rect x="400" y="90" width="120" height="50" rx="25" fill="rgba(16,185,129,.12)" stroke="#10b981" stroke-width="2"/><text x="460" y="120" fill="#f0f0f5" font-size="14" font-weight="700">Membre</text>
        <line x1="520" y1="115" x2="596" y2="115" stroke="#9ca3af" stroke-width="2" marker-end="url(#af)"/>
        <text x="558" y="100" fill="#9ca3af" font-size="10">accusation</text>
        <text x="558" y="134" fill="#9ca3af" font-size="10">quorum de 6</text>
        <rect x="600" y="90" width="150" height="50" rx="25" fill="rgba(245,158,11,.12)" stroke="#f59e0b"/><text x="675" y="120" fill="#f0f0f5" font-size="14" font-weight="600">Suspendu</text>
        <path d="M675,90 C675,30 460,30 460,86" fill="none" stroke="#10b981" stroke-width="2" marker-end="url(#afg)"/>
        <text x="568" y="40" fill="#10b981" font-size="11">recours accepté (72 h)</text>
        <line x1="675" y1="140" x2="675" y2="186" stroke="#9ca3af" stroke-width="2" marker-end="url(#af)"/>
        <text x="610" y="168" fill="#9ca3af" font-size="10">72 h sans recours</text>
        <rect x="600" y="190" width="150" height="46" rx="23" fill="rgba(239,68,68,.12)" stroke="#ef4444"/><text x="675" y="218" fill="#f0f0f5" font-size="14" font-weight="600">Révoqué</text>
        <text x="250" y="200" fill="#9ca3af" font-size="12">Une faute se prouve par des preuves signées,</text>
        <text x="250" y="218" fill="#9ca3af" font-size="12">jamais par simple réputation.</text>
      </g>
    </svg>
    <figcaption>Origin aide à se trouver (relais, catalogue) mais n'est pas indispensable : la fédération tient sans lui, et un COG peut rester seul.</figcaption>
  </figure>
  <ul class="pr-points">
    <li>Pour rejoindre un relais Origin, un COG passe une vérification en trois phases : clé de conformité des Cores, blocs de code des services, santé de l'environnement. Il reçoit alors un permis de circulation.</li>
    <li>Origin ne détient ni vos données ni vos clés : il ne peut ni lire un COG, ni lui imposer une mise à jour.</li>
  </ul>
</section>

<section class="section" id="maia">
  <h2 class="section-title">MAIA, une IA locale qui propose</h2>
  <p class="section-subtitle">Elle passe par les mêmes Cores que n'importe quel service</p>
  <div class="pr-specs pr-4">
    <div class="pr-spec"><h3>Sur votre matériel</h3><p>Modèles locaux adaptés à la machine. Aucune requête n'est envoyée à un fournisseur d'IA en ligne.</p></div>
    <div class="pr-spec"><h3>Mémoire chiffrée</h3><p>Par utilisateur, jamais publiée ni synchronisée, avec un oubli réel à la demande (effacement physique vérifié).</p></div>
    <div class="pr-spec"><h3>Bornée</h3><p>MAIA émet des intentions : elle ne décide jamais seule, et les décisions sensibles passent par une validation humaine.</p></div>
    <div class="pr-spec"><h3>Fiches fédérées</h3><p>Elle peut lire des connaissances d'autres COG, marquées étrangères et corroborées ; elle ne publie que ce que vous avez étiqueté et signé.</p></div>
  </div>
  <div class="pr-note"><strong>État du projet :</strong> version 0.9.0-beta.1, dite de recette — à installer sur une machine dédiée, jamais en production. Ni certification ni audit externe à ce jour.</div>
  <div class="pr-next">
    <a href="/souverainete" class="btn btn-primary">Souveraineté &amp; conformité →</a>
    <a href="/services" class="btn btn-secondary">Les services</a>
  </div>
</section>
"##
    );
    layout("Fonctionnement", &content, "fonctionnement")
}

/// Page « Souveraineté & conformité » : où vont les données, qui en répond, et l'état réglementaire.
pub fn souverainete_page() -> String {
    let content = format!(
        r##"{STYLES}
<section class="hero" style="padding: 3rem 0 2rem;">
  <h1>Souveraineté &amp; conformité</h1>
  <p class="pr-lead">Où sont vos données, qui peut y accéder, quels textes s'appliquent — et où en est COG sur chacun.
  On y distingue ce qui est en place, ce qui est partiel et ce qui manque encore.</p>
  <nav class="pr-toc" aria-label="Sommaire">
    <a href="#donnees">Où sont les données</a>
    <a href="#extraterritorial">Droit extraterritorial</a>
    <a href="#rgpd">RGPD</a>
    <a href="#ia">IA et voix</a>
    <a href="#textes">Textes applicables</a>
    <a href="#dependances">Dépendances résiduelles</a>
    <a href="#referentiels">Référentiels visés</a>
  </nav>
  <div class="pr-note" style="max-width: 760px; margin: 1.5rem auto 0; text-align: left;"><strong>À lire d'abord :</strong> cette page résume une recherche documentaire datée du 24 octobre 2026. Ce n'est pas un avis juridique ; les points marqués « à vérifier » doivent être confirmés par un juriste, l'ANSSI ou les organismes concernés.</div>
</section>

<section class="section" id="donnees">
  <h2 class="section-title">Où sont vos données, et qui détient les clés</h2>
  <p class="section-subtitle">La souveraineté commence par une question simple : qui a la garde des données ?</p>
  <figure class="diag">
    <svg viewBox="0 0 760 300" role="img" aria-labelledby="s1t">
      <title id="s1t">Les données et les clés restent chez l'utilisateur ; Origin ne voit que des métadonnées de relais</title>
      <defs><marker id="sa" viewBox="0 0 10 10" refX="9" refY="5" markerWidth="7" markerHeight="7" orient="auto-start-reverse"><path d="M0,0 L10,5 L0,10 z" fill="#9ca3af"/></marker></defs>
      <g font-family="Inter, sans-serif" text-anchor="middle">
        <rect x="10" y="20" width="370" height="260" rx="14" fill="rgba(139,92,246,.10)" stroke="#8b5cf6" stroke-width="2"/>
        <text x="195" y="46" fill="#c4b5fd" font-size="13" font-weight="700">CHEZ VOUS — votre COG</text>
        <rect x="35" y="64" width="320" height="56" rx="8" fill="#1a1a25" stroke="#10b981"/>
        <text x="195" y="88" fill="#f0f0f5" font-size="14" font-weight="600">Vos données</text>
        <text x="195" y="106" fill="#9ca3af" font-size="11">agenda, compta, documents, voix, mémoire MAIA</text>
        <rect x="35" y="132" width="320" height="56" rx="8" fill="#1a1a25" stroke="#10b981"/>
        <text x="195" y="156" fill="#f0f0f5" font-size="14" font-weight="600">Vos clés</text>
        <text x="195" y="174" fill="#9ca3af" font-size="11">une clé par base (SQLCipher), racine hors ligne</text>
        <rect x="35" y="200" width="320" height="56" rx="8" fill="#1a1a25" stroke="#10b981"/>
        <text x="195" y="224" fill="#f0f0f5" font-size="14" font-weight="600">Le journal d'audit</text>
        <text x="195" y="242" fill="#9ca3af" font-size="11">chaîné, écrit par KindMother seul</text>
        <line x1="380" y1="150" x2="456" y2="150" stroke="#9ca3af" stroke-width="2" stroke-dasharray="6 4" marker-end="url(#sa)"/>
        <text x="418" y="138" fill="#9ca3af" font-size="10">optionnel</text>
        <rect x="460" y="20" width="290" height="260" rx="14" fill="#12121a" stroke="#9ca3af" stroke-dasharray="6 4"/>
        <text x="605" y="46" fill="#9ca3af" font-size="13" font-weight="700">ORIGIN — si vous vous fédérez</text>
        <text x="605" y="84" fill="#f0f0f5" font-size="13" font-weight="600">Voit</text>
        <text x="605" y="104" fill="#9ca3af" font-size="11">comptes, identités fédérales,</text>
        <text x="605" y="120" fill="#9ca3af" font-size="11">métadonnées de relais, journaux</text>
        <text x="605" y="164" fill="#f0f0f5" font-size="13" font-weight="600">Ne détient pas</text>
        <text x="605" y="184" fill="#ef4444" font-size="11">vos données ✕ vos clés ✕</text>
        <text x="605" y="226" fill="#f0f0f5" font-size="13" font-weight="600">Ne peut pas</text>
        <text x="605" y="246" fill="#9ca3af" font-size="11">lire un COG ni pousser une mise à jour</text>
      </g>
    </svg>
    <figcaption>Un COG qui ne se fédère pas n'échange rien avec Origin. Les mises à jour exigent deux signatures, dont une hors ligne.</figcaption>
  </figure>
</section>

<section class="section" id="extraterritorial">
  <h2 class="section-title">Droit extraterritorial : CLOUD Act, FISA 702</h2>
  <p class="section-subtitle">Ce qu'un fournisseur ne détient pas, il ne peut pas le remettre</p>
  <ul class="pr-points">
    <li>Le <strong>CLOUD Act</strong> (2018) oblige un fournisseur soumis au droit américain à remettre les données qu'il a « en sa possession, sous sa garde ou son contrôle », où qu'elles soient stockées. <strong>FISA §702</strong> permet la surveillance de non-Américains via les fournisseurs de communications électroniques.</li>
    <li>Avec COG, les données sont au domicile de l'utilisateur, chiffrées, et les clés sont en sa possession : aucun fournisseur tiers n'en a la garde. C'est ce qui les place <strong>hors de portée directe</strong> de ces textes.</li>
    <li>Une contrainte légale exercée sur l'hébergeur d'Origin ne permet ni de lire les données d'un COG, ni de pousser une mise à jour : la signature hors ligne manque.</li>
    <li>Le cadre transatlantique (<em>Data Privacy Framework</em>) a été validé par le Tribunal de l'UE en septembre 2025 ; un pourvoi est pendant devant la CJUE. COG ne repose pas dessus.</li>
  </ul>
</section>

<section class="section" id="rgpd">
  <h2 class="section-title">RGPD : qui est responsable de quoi</h2>
  <p class="section-subtitle">Héberger chez soi ne dispense pas du RGPD ; cela change qui en répond</p>
  <div class="pr-scroll">
  <table class="pr-table">
    <thead><tr><th>Acteur</th><th>Rôle au sens du RGPD</th></tr></thead>
    <tbody>
      <tr><td><strong>Particulier</strong></td><td>Exemption domestique (art. 2(2)(c)) pour un usage personnel.</td></tr>
      <tr><td><strong>Organisation</strong> (TPE, association, cabinet)</td><td><strong>Responsable de traitement</strong>, y compris pour les données de tiers : clients dans JayKonta, rendez-vous dans JayRDV, courriels.</td></tr>
      <tr><td><strong>Éditeur</strong> (Studio Miyukini)</td><td>Pas responsable des données locales auxquelles il n'a pas accès, mais tenu de fournir un produit qui <strong>permet</strong> la conformité (art. 25, protection dès la conception).</td></tr>
      <tr><td><strong>Origin</strong></td><td>Responsable de traitement pour ses propres données (comptes, identités fédérales, registres, journaux) ; possiblement sous-traitant (art. 28) pour les données relayées — <em>à vérifier selon les flux</em>.</td></tr>
    </tbody>
  </table>
  </div>
  <h3 style="margin-top: 2rem;">Ce que l'architecture apporte</h3>
  <ul class="pr-points">
    <li><strong>Sécurité (art. 32)</strong> : chiffrement au repos (AES-256), une clé par base, segmentation par service, profils AppArmor.</li>
    <li><strong>Traçabilité</strong> : toute écriture passe par KindMother et laisse une trace dans un journal chaîné.</li>
    <li><strong>Minimisation et non-chaînabilité</strong> : un identifiant de passeport différent pour chaque COG visité.</li>
    <li><strong>Droit à l'effacement</strong> : « oubli réel » de la mémoire MAIA, avec effacement physique vérifié.</li>
    <li><strong>Pas de télémétrie</strong> vers l'éditeur par défaut ; toute métrique fédérale doit être agrégée, signée et acceptée par l'utilisateur.</li>
  </ul>
  <h3 style="margin-top: 2rem;">Ce qui reste à la charge d'une organisation</h3>
  <ul class="pr-points">
    <li>Tenir le registre des traitements et informer les personnes concernées.</li>
    <li>Mener une analyse d'impact (AIPD) quand elle s'impose, notamment pour la voix (Alicia) ou la mémoire MAIA.</li>
    <li>Fixer les durées de conservation ; pour les journaux, la CNIL recommande en général de 6 mois à 1 an.</li>
    <li>Notifier une violation de données sous 72 heures.</li>
  </ul>
  <div class="pr-note"><strong>Encore à faire côté COG :</strong> les AIPD d'Origin et de la fédération (la notation des pairs et les sanctions s'apparentent à du profilage), un modèle d'AIPD fourni aux organisations, et la conciliation des registres Merkle — immuables — avec le droit à l'effacement (stocker des empreintes, garder les données hors chaîne).</div>
</section>

<section class="section" id="ia">
  <h2 class="section-title">IA et voix</h2>
  <p class="section-subtitle">MAIA et Alicia relèvent de l'AI Act et, pour la voix, de règles RGPD renforcées</p>
  <ul class="pr-points">
    <li><strong>AI Act, article 50</strong> (en vigueur depuis le 2 août 2026) : MAIA et Alicia doivent se déclarer comme IA et marquer les contenus générés — <span class="st st-no">absent</span> à ce jour.</li>
    <li><strong>Voix</strong> : reconnaître <em>qui</em> parle est un traitement biométrique (art. 9 RGPD), qui exige un consentement explicite. Traiter une simple commande vocale ne l'est pas.</li>
    <li>Suivant le livre blanc CNIL sur les assistants vocaux : traitement local, pas de stockage du tampon du mot d'éveil, indicateur d'écoute visible.</li>
    <li>La reconnaissance des émotions au travail ou à l'école est interdite (AI Act, art. 5).</li>
    <li>Pas d'apprentissage sur des données étrangères au COG.</li>
  </ul>
</section>

<section class="section" id="textes">
  <h2 class="section-title">Textes applicables et état de COG</h2>
  <p class="section-subtitle">Échéances légales, et où en est le projet sur chacune (au 26 octobre 2026)</p>
  <div class="pr-scroll">
  <table class="pr-table">
    <thead><tr><th>Texte</th><th>Échéance</th><th>Ce qu'il impose</th><th>État</th></tr></thead>
    <tbody>
      <tr><td><strong>Cyber Resilience Act</strong> — signalement</td><td>en vigueur (11/09/2026)</td><td>Signaler à l'ENISA les vulnérabilités activement exploitées et incidents graves (24 h / 72 h / rapport final)</td><td><span class="st st-part">partiel</span><br><small>politique et processus rédigés ; contact, astreinte et compte ENISA manquants</small></td></tr>
      <tr><td><strong>Cyber Resilience Act</strong> — complet</td><td>11/12/2027</td><td>Exigences essentielles, SBOM, support 5 ans, marquage CE, évaluation de conformité</td><td><span class="st st-part">partiel</span></td></tr>
      <tr><td><strong>RGPD</strong></td><td>en vigueur</td><td>Protection dès la conception, registre, AIPD</td><td><span class="st st-part">partiel</span></td></tr>
      <tr><td><strong>AI Act</strong>, art. 50</td><td>en vigueur (02/08/2026)</td><td>Transparence des systèmes d'IA</td><td><span class="st st-no">absent</span></td></tr>
      <tr><td><strong>LCEN / DSA</strong></td><td>en vigueur</td><td>Obligations d'hébergeur pour Origin et le Market (signalement, retrait)</td><td><span class="st st-no">absent</span></td></tr>
      <tr><td><strong>Cryptologie</strong> (France)</td><td>avant diffusion</td><td>Déclaration des moyens de cryptologie à l'ANSSI ; double usage hors UE</td><td><span class="st st-no">absent</span><br><small>exemption open source à vérifier</small></td></tr>
      <tr><td><strong>Facturation électronique</strong></td><td>01/09/2026 · 01/09/2027</td><td>Réception puis émission via plateforme agréée, FEC, clôture inaltérable</td><td><span class="st st-no">absent</span><br><small>concerne JayKonta</small></td></tr>
      <tr><td><strong>Responsabilité du fait des produits</strong></td><td>09/12/2026</td><td>Le logiciel devient un produit au sens de la directive</td><td><span class="st st-part">à anticiper</span></td></tr>
      <tr><td><strong>NIS2</strong></td><td>non transposée en France</td><td>Touche COG via la chaîne d'approvisionnement des clients</td><td><span class="st st-na">à suivre</span></td></tr>
      <tr><td><strong>Data Act</strong></td><td>12/09/2026 (art. 3(1))</td><td>Accès aux données des produits connectés</td><td><span class="st st-na">probablement limité</span><br><small>logiciel seul ; à vérifier si un boîtier est vendu</small></td></tr>
      <tr><td><strong>Post-quantique</strong> (ANSSI)</td><td>2027 · 2030</td><td>Cryptographie hybride attendue pour la certification et les usages au-delà de 2030</td><td><span class="st st-no">absent</span><br><small>feuille de route : X25519 + ML-KEM, Ed25519 + ML-DSA</small></td></tr>
    </tbody>
  </table>
  </div>
</section>

<section class="section" id="dependances">
  <h2 class="section-title">Dépendances résiduelles hors UE</h2>
  <p class="section-subtitle">Ce qui échappe encore au contrôle européen, et comment c'est atténué</p>
  <div class="pr-scroll">
  <table class="pr-table">
    <thead><tr><th>Dépendance</th><th>Risque</th><th>Atténuation</th></tr></thead>
    <tbody>
      <tr><td>GitHub et GitHub Actions</td><td>Code et intégration continue</td><td>Builds reproductibles ; miroir européen ou auto-hébergé (Forgejo) prévu</td></tr>
      <tr><td>crates.io</td><td>Dépendances Rust</td><td>Versions figées par <code>Cargo.lock</code> ; miroir prévu</td></tr>
      <tr><td>Hugging Face</td><td>Modèles d'IA</td><td>Miroir des modèles prévu</td></tr>
      <tr><td>Google et Apple</td><td>Magasins et notifications du mobile (métadonnées)</td><td>Notifications de réveil sans contenu ni identifiant stable</td></tr>
      <tr><td>Hébergement d'Origin</td><td>VPS non qualifié ; un anti-DDoS américain pourrait déchiffrer le trafic</td><td>Décision à prendre : hébergeur UE (SecNumCloud pour les marchés publics), pas d'anti-DDoS américain</td></tr>
      <tr><td>Fournisseurs d'IA en ligne</td><td>—</td><td>Aucun : l'IA est locale</td></tr>
    </tbody>
  </table>
  </div>
</section>

<section class="section" id="referentiels">
  <h2 class="section-title">Référentiels visés</h2>
  <p class="section-subtitle">Des cibles, pas des certifications obtenues</p>
  <ul class="pr-points">
    <li><strong>ANSSI</strong> : guide de développement Rust, recommandations TLS, journalisation et cryptographie ; première certification visée : <strong>CSPN</strong> sur le canal de mise à jour et le vérificateur de Sceaux.</li>
    <li><strong>Applicatif</strong> : OWASP ASVS 5.0 niveau 2 (Central, Origin, portail, MiyuCloud) et niveau 3 (authentification, KindMother, Portail) ; OWASP MASVS pour le mobile.</li>
    <li><strong>IA</strong> : OWASP Top 10 LLM 2025 et agentique, ETSI EN 304 223.</li>
    <li><strong>Domotique</strong> : ETSI EN 303 645, IEC 62443-4-1 / 4-2.</li>
    <li><strong>Chaîne d'approvisionnement</strong> : SLSA niveau 2 puis 3, builds reproductibles, SBOM CycloneDX signé.</li>
    <li><strong>Vulnérabilités</strong> : ISO/IEC 29147 et 30111, RFC 9116 (<code>security.txt</code>), CVSS 4.0.</li>
    <li><strong>Organisation</strong> : ISO 27001 pour Origin ; 27701 et 42001 en alignement.</li>
  </ul>
  <div class="pr-note"><strong>Aujourd'hui :</strong> aucune certification, aucun audit externe, aucun déploiement en production. COG 0.9.0-beta.1 est une version de recette.</div>
  <div class="pr-next">
    <a href="/fonctionnement" class="btn btn-primary">← Comment fonctionne COG</a>
  </div>
</section>
"##
    );
    layout("Souveraineté & conformité", &content, "souverainete")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fonctionnement_porte_les_schemas_sans_contenu_commercial() {
        let page = fonctionnement_page();
        assert_eq!(page.matches("<svg").count(), 5);
        for interdit in ["€", "&euro;", "Tarifs", "devis"] {
            assert!(!page.contains(interdit), "mention commerciale : {interdit}");
        }
    }

    #[test]
    fn souverainete_affiche_l_avertissement_et_l_etat_reel() {
        let page = souverainete_page();
        assert!(page.contains("pas un avis juridique"));
        assert!(page.contains("st-no"), "les manques doivent rester visibles");
        assert!(!page.contains("€"));
    }
}
