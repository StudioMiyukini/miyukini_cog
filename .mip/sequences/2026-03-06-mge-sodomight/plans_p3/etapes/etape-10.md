# Etape 10 - Parite MVP, equilibrage, tests et freeze documentaire

## Objectif

Fermer P3 avec un build coherent, testable et transmissible vers audit, validation utilisateur et suite de production.

## Taches

1. Geler la feature matrix MVP et lister les ecarts encore ouverts.
2. Executer les tests unitaires moteur et gameplay.
3. Executer les tests integration runtime, contenu, save/load et packaging.
4. Executer les scenarios de parcours complets camp -> boss -> retour camp.
5. Executer les scenarios local host vs dedicated sim sur les features reseau/party critiques.
6. Corriger les ecarts de parite D2 encore ouverts sur les systemes cibles.
7. Corriger les regressions de contenu et de quetes detectees par la campagne de test.
8. Produire une passe d'equilibrage camp + Acte 1 basee sur telemetrie locale et scripts de test.
9. Produire une passe de verification perf render/GPU sur les scenes town et boss.
10. Relire et geler la documentation de soutien qui sert directement a l'implementation.
11. Classer le backlog residuel en `bloquant`, `important`, `post-MVP`.
12. Preparer le dossier de transfert vers P4/P5 avec preuves de couverture et risques restants.

## Documentation de soutien

1. Finaliser la matrice de tests MVP et les preuves de couverture.
2. Finaliser les notes d'equilibrage et les ecarts connus.
3. Finaliser le dossier de transfert vers P4/P5 avec backlog residuel trace.

## Criteres de sortie

1. Le MVP est jouable de bout en bout sur le perimetre camp + Acte 1.
2. Les systemes D2 cibles sont couverts par tests, harness ou scenarios verifies.
3. Le dossier P3 est suffisant pour entrer en audit sans zone floue majeure.
