<!-- @id cert.index -->
<!-- @do route_certification_lookup_by_agent -->
<!-- @role index -->
<!-- @layer reference -->
<!-- @human Index certifications par agent â€” chargement selectif -->

# Certifications â€” Index

> **Chargement sÃ©lectif** : `load-map.json` (tÃ¢che â†’ cert_ids). RÃ©solution : `registry.json` (cert_id â†’ folder). Ne jamais charger tous les rÃ©fÃ©rentiels.

## Flux de chargement

```
TÃ¢che (ex: francois.unit_test) â†’ load-map.json â†’ [cert.francois.istqb]
                                              â†’ registry.json â†’ istqb/
                                              â†’ .mip/certifications/istqb/REFERENCE.md
```

## Routage par agent

| Agent | Cert IDs | Dossiers |
|-------|----------|----------|
| Maria | cert.maria.{pmp,prince2,psm,itil4} | pmp, prince2, psm, itil4 |
| Fabrice | cert.fabrice.{pspo,lean_startup} | pspo, lean_startup |
| Denis | cert.denis.{togaf,iso_25010,iso_12207} | togaf, iso_25010, iso_12207 |
| FranÃ§ois | cert.francois.{istqb,openapi} | istqb, openapi |
| Lise | cert.lise.{wcag,iso_9241} | wcag, iso_9241 |
| Arianne | cert.arianne.{iso_9001,six_sigma,iso_33001} | iso_9001, six_sigma, iso_33001 |
| George | cert.george.{iso_19011,cisa,rgpd} | iso_19011, cisa, rgpd |
| Victor | cert.victor.{iso_27001,vp2,hds,iso_20000,nf461,nf203,nf525,cmmi,cissp,ceh} | iso-iec_27001, vp2, hds, iso-iec_20000-1, nf461, nf203, nf525, cmmi, cissp, ceh |
| Hugo | cert.hugo.{devops,aws,cka,terraform,docker} | DevOPS, aws, cka, terraform, docker |
| Jean | cert.jean.{finops,prompt_eng,mlops} | finops, prompt_eng, mlops |

## ClÃ©s load-map (exemples)

| ClÃ© | Certs |
|-----|-------|
| francois.unit_test | cert.francois.istqb |
| francois.api_design | cert.francois.openapi |
| lise.ui_component | cert.lise.wcag, cert.lise.iso_9241 |
| victor.threat_model | cert.victor.cissp, cert.victor.iso_27001 |
| george.audit_conformite | cert.george.iso_19011, cert.george.cisa |
| hugo.docker | cert.hugo.docker |

Voir `load-map.json` pour la liste complÃ¨te.

## Obligations lÃ©gales

Voir `legal/OBLIGATIONS.md` â€” HDS, NF525, NF203, RGPD.


## Dossier critique cyber

- Index dedie: `.mip/certifications/critical_cyber/INDEX.md`
- Usage: verification de controle par certification critique.

## Decoupage monolithiques

- Protocole agent (dedie): `.mip/certifications/agent-certification-protocol/INDEX.md`
- Index dedie: `.mip/certifications/monolithiques_decoupes/INDEX.md`
- Usage: navigation par sections pour les gros fichiers certifications.


