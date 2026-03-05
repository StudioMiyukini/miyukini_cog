# Analyse PR et comparative concurrence -- Miyukini Whisper

## TL;DR

Miyukini Whisper doit se positionner comme l option "local-first souveraine" pour la productivite clavier.
Le message central: "dictee rapide, privee, FR/EN, controle total local, fallback uniquement si vous le decidez."
La concurrence directe montre une forte traction sur UX/hotkeys, mais souvent avec limites gratuites, gating Pro, ou dependance cloud partielle.
L opportunite COG: unifier STT/TTS + presets hardware + API inter-services (Alicia, futurs services), avec une UI centralisee dans Central.

---

## 1) Analyse PR (positionnement)

## 1.1 Cible

1. Dev: minimiser friction clavier/souris, dicter dans IDE/terminal/chat.
2. Auteur: dictee longue + correction rapide.
3. Etudiant: prise de notes, reformulation et relecture vocale.

## 1.2 Message coeur

`Miyukini Whisper`:
- fonctionne localement par defaut
- transcrit FR/EN avec presets hardware adaptes
- s integre dans toutes les zones texte du poste
- expose une API stable pour les autres services COG

Claim PR recommande:
"Parle. Le texte apparait. Tes donnees restent chez toi."

## 1.3 Piliers de preuve

1. Local-first actif en configuration par defaut.
2. STT/TTS independants de tout LLM cloud.
3. Fallback distant uniquement opt-in, visible dans UI.
4. Presets hardware mesurables (`compact`, `balanced`, `precision`).

## 1.4 Objections previsibles et reponses

| Objection | Reponse PR |
|---|---|
| "Le local est moins bon que le cloud" | Presets + fallback bridge/cloud opt-in si besoin |
| "Je veux que ca marche vite partout" | hotkey global + insertion directe champ actif |
| "Je ne veux pas configurer des modeles" | auto-detection hardware + profil par defaut |
| "Je veux de la privacy stricte" | mode local-only verrouillable |

---

## 2) Analyse comparative concurrence (snapshot 2026-03-05)

## 2.1 Concurrents directs

| Produit | Positionnement | Prix / gating | Local | Points forts | Limites |
|---|---|---|---|---|---|
| Wispr Flow | dictation grand public/team | Basic gratuit limite, Pro payant | mixte | UX mature, large couverture langue | quotas free, mode premium |
| Superwhisper | dictation + modes IA | Free + Pro (mensuel/annuel/lifetime) | local + cloud | docs modele detaillees, multi-device | local models et fonctions avancees cote Pro |
| MacWhisper | transcription locale macOS | free + Pro | local | simplicite locale, ecosysteme macOS | scope surtout macOS |
| OpenWhispr / open-wispr | alternatives open-source | gratuit | local | transparence code, cout nul | maturite variable, support limite |

## 2.2 Sources primaires utilisees

- Wispr Flow pricing (plans, quotas, trial): https://wisprflow.ai/pricing
- Wispr Flow docs plans/discounts:
  - https://docs.wisprflow.ai/articles/9559327591-flow-plans-and-what-s-included
  - https://docs.wisprflow.ai/articles/1128761434-flow-discounts
- Superwhisper Pro (features/prix/licence multi-device):
  - https://superwhisper.com/docs/get-started/sw-pro
- Superwhisper modeles/modes:
  - https://superwhisper.com/docs/models/language
  - https://superwhisper.com/docs/modes/voice
- MacWhisper presskit:
  - https://macwhisper.pressdeck.io/
- OpenWhispr:
  - https://openwhispr.com/
- open-wispr:
  - https://open-wispr.com/

Note: ce tableau est un snapshot produit/date. Les prix/features peuvent evoluer.

---

## 3) Gap concurrentiel et angle Miyukini

## 3.1 Ce que la concurrence fait bien

1. UX "push-to-talk" immediate.
2. Insertion cross-app fluide.
3. Messaging productivite tres clair.

## 3.2 Ce que Miyukini peut faire mieux

1. API native inter-services COG (Alicia et autres) des la V1.
2. Presets hardware standardises avec auto-selection.
3. Gouvernance fallback locale/host/cloud explicite.
4. Integrer STT + TTS + diagnostics dans Central.

---

## 4) Recommandations PR actionnables

1. Lancer avec 3 promesses simples:
   - "4x plus rapide que taper" (a valider bench interne)
   - "Mode local-only par defaut"
   - "Compatible FR/EN"
2. Publier une demo courte: IDE + document + chat (3 cas usage).
3. Exposer un tableau de latence par preset hardware.
4. Mettre en avant `fallback opt-in` comme argument confiance.
5. Preparer un comparatif public "Miyukini Whisper vs alternatives" apres P5.

---

## 5) KPIs PR/Produit

| KPI | Cible V1 |
|---|---|
| Temps premier transcript apres install | < 5 min |
| Taux activation hebdo feature dictee | > 35% utilisateurs cibles |
| Part usage mode local-only | > 70% |
| Satisfaction qualite transcription | >= 4/5 |
| Taux retention 30 jours | > 40% |
