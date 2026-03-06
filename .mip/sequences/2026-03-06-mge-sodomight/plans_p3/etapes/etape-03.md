# Etape 03 - Renderer, animation, audio et pipeline assets

## Objectif

Livrer le socle de presentation temps reel et le pipeline interne pour produire les placeholders et assets du MVP.

## Taches

1. Initialiser `winit`, `wgpu`, surface, device, queue et swapchain.
2. Poser la boucle frame et la separation simulation / rendu.
3. Implementer la camera isometrique et les conversions monde -> ecran.
4. Poser le registre de textures, atlases, materials et handles GPU.
5. Implementer le pass terrain/props opaques.
6. Implementer le pass entites/projectiles avec tri stable et `sort_key`.
7. Implementer le pass VFX opaque puis le pass VFX alpha.
8. Ajouter brouillard, obscurite, emissive locale et color grading de base.
9. Implementer le lecteur de clips d'animation, directions et notify events.
10. Poser les quality tiers `low`, `standard`, `high`.
11. Implementer la recovery `device lost`, resize et recreation de swapchain.
12. Poser le streaming d'atlas, les overlays perf et la telemetrie draw calls/memoire.
13. Poser le pipeline audio, bus, cues et liaison events gameplay -> audio.
14. Construire l'asset baker: import, trim, atlas, metadata, hash, validation.
15. Produire la bible visuelle et la premiere liste d'assets originaux D2-like.

## Documentation de soutien

1. Documenter l'architecture du renderer et les conventions visuelles.
2. Documenter la nomenclature assets, les formats et le pipeline de generation.
3. Documenter les budgets de performances et les garde-fous renderer.
4. Documenter la bible d'assets et les regles de non-copie.

## Criteres de sortie

1. Scene isometrique rendue de maniere stable.
2. Assets internes chargeables et regenerables.
3. Pipeline suffisamment documente pour soutenir la production de contenu.
4. Les fondations de scalabilite GPU sont posees et mesurables.
