# Deploy doc_for_website to Origin VPS
# Usage: .\deploy-to-origin.ps1 -KeyPath "C:\path\to\ssh-key-2026-02-12.key"

param(
    [Parameter(Mandatory=$true)]
    [string]$KeyPath
)

$VPS_IP = "46.202.129.65"
$VPS_USER = "root"
$REMOTE_PATH = "/var/www/docs"
$LOCAL_PATH = $PSScriptRoot

Write-Host "Deploiement de la documentation vers Origin VPS..." -ForegroundColor Cyan
Write-Host "VPS: $VPS_USER@$VPS_IP" -ForegroundColor Gray
Write-Host "Destination: $REMOTE_PATH" -ForegroundColor Gray

# Verification de la cle SSH
if (-not (Test-Path $KeyPath)) {
    Write-Host "ERREUR: Cle SSH introuvable: $KeyPath" -ForegroundColor Red
    exit 1
}

# Creer le repertoire distant et definir les permissions
Write-Host "`n[1/3] Creation du repertoire distant..." -ForegroundColor Yellow
ssh -i $KeyPath "$VPS_USER@$VPS_IP" "mkdir -p $REMOTE_PATH && chown -R www-data:www-data /var/www"

if ($LASTEXITCODE -ne 0) {
    Write-Host "ERREUR: Impossible de creer le repertoire distant" -ForegroundColor Red
    exit 1
}

# Transfert des fichiers avec scp (recursif)
Write-Host "`n[2/3] Transfert des fichiers..." -ForegroundColor Yellow
scp -i $KeyPath -r "$LOCAL_PATH\*" "$VPS_USER@$VPS_IP`:$REMOTE_PATH/"

if ($LASTEXITCODE -ne 0) {
    Write-Host "ERREUR: Echec du transfert" -ForegroundColor Red
    exit 1
}

# Definir les permissions et recharger nginx si necessaire
Write-Host "`n[3/3] Permissions et configuration nginx..." -ForegroundColor Yellow
ssh -i $KeyPath "$VPS_USER@$VPS_IP" @"
chown -R www-data:www-data $REMOTE_PATH
chmod -R 755 $REMOTE_PATH
find $REMOTE_PATH -type f -exec chmod 644 {} \;

# Verifier si la location /docs existe deja dans nginx
if ! grep -q 'location /docs' /etc/nginx/sites-available/origin-miyukini.conf 2>/dev/null; then
    echo 'Ajout de la location /docs dans nginx...'
    # Backup
    cp /etc/nginx/sites-available/origin-miyukini.conf /etc/nginx/sites-available/origin-miyukini.conf.bak
    
    # Inserer la location /docs avant la location / 
    sed -i '/location \/ {/i \    location /docs {\n        alias /var/www/docs/;\n        index index.html;\n        try_files \$uri \$uri/ /docs/index.html;\n        add_header Cache-Control \"public, max-age=3600\";\n    }\n' /etc/nginx/sites-available/origin-miyukini.conf
    
    nginx -t && systemctl reload nginx
    echo 'Nginx recharge avec la nouvelle configuration.'
else
    echo 'Location /docs deja configuree.'
    systemctl reload nginx
fi

echo 'Deploiement termine.'
"@

if ($LASTEXITCODE -ne 0) {
    Write-Host "AVERTISSEMENT: Probleme lors de la configuration nginx" -ForegroundColor Yellow
} else {
    Write-Host "`nDeploiement termine avec succes!" -ForegroundColor Green
    Write-Host "Documentation accessible sur: https://$VPS_IP/docs/" -ForegroundColor Cyan
    Write-Host "(ou https://origin.miyukini.com/docs/ si le domaine est configure)" -ForegroundColor Gray
}
