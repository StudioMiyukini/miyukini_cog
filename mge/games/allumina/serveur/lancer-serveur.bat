@echo off
rem ── Allumina — serveur multijoueur + pont web ───────────────────────────────
rem Lance le serveur TCP autoritatif (port 7777, local) ET le pont HTTP+WebSocket
rem sur le port 7000 (la cible du tunnel Cloudflare allumina.miyukini-home.org).
rem Laisse cette fenetre ouverte ; Ctrl-C pour arreter.
title Allumina - serveur (web 7000, tcp 7777)
echo Demarrage du serveur Allumina (web :7000  /  tcp :7777)...
echo Navigateur local : http://localhost:7000/
echo Via le domaine   : https://allumina.miyukini-home.org/
echo.
"C:\Users\Van Jean\Documents\miyukini-cog\mge\target\release\allumina.exe" --server --web-port 7000
echo.
echo [serveur arrete]
pause
