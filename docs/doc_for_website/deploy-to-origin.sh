#!/bin/bash
# Deploy doc_for_website to Origin VPS
# Usage: ./deploy-to-origin.sh /path/to/ssh-key-2026-02-12.key

set -e

KEY_PATH="$1"
VPS_IP="46.202.129.65"
VPS_USER="root"
REMOTE_PATH="/var/www/docs"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo -e "\033[36mDeploiement de la documentation vers Origin VPS...\033[0m"
echo "VPS: $VPS_USER@$VPS_IP"
echo "Destination: $REMOTE_PATH"

if [ -z "$KEY_PATH" ]; then
    echo -e "\033[31mERREUR: Chemin de la cle SSH requis\033[0m"
    echo "Usage: $0 /path/to/ssh-key.key"
    exit 1
fi

if [ ! -f "$KEY_PATH" ]; then
    echo -e "\033[31mERREUR: Cle SSH introuvable: $KEY_PATH\033[0m"
    exit 1
fi

# Creer le repertoire distant
echo -e "\n\033[33m[1/3] Creation du repertoire distant...\033[0m"
ssh -i "$KEY_PATH" "$VPS_USER@$VPS_IP" "mkdir -p $REMOTE_PATH && chown -R www-data:www-data /var/www"

# Transfert des fichiers avec rsync (plus efficace que scp)
echo -e "\n\033[33m[2/3] Transfert des fichiers...\033[0m"
rsync -avz --delete -e "ssh -i $KEY_PATH" \
    --exclude 'deploy-to-origin.ps1' \
    --exclude 'deploy-to-origin.sh' \
    "$SCRIPT_DIR/" "$VPS_USER@$VPS_IP:$REMOTE_PATH/"

# Permissions et configuration nginx
echo -e "\n\033[33m[3/3] Permissions et configuration nginx...\033[0m"
ssh -i "$KEY_PATH" "$VPS_USER@$VPS_IP" << 'EOF'
chown -R www-data:www-data /var/www/docs
chmod -R 755 /var/www/docs
find /var/www/docs -type f -exec chmod 644 {} \;

# Verifier si la location /docs existe deja
if ! grep -q 'location /docs' /etc/nginx/sites-available/origin-miyukini.conf 2>/dev/null; then
    echo 'Ajout de la location /docs dans nginx...'
    cp /etc/nginx/sites-available/origin-miyukini.conf /etc/nginx/sites-available/origin-miyukini.conf.bak
    
    # Inserer la location /docs avant la location /
    sed -i '/location \/ {/i \    location /docs {\n        alias /var/www/docs/;\n        index index.html;\n        try_files $uri $uri/ /docs/index.html;\n        add_header Cache-Control "public, max-age=3600";\n    }\n' /etc/nginx/sites-available/origin-miyukini.conf
    
    nginx -t && systemctl reload nginx
    echo 'Nginx recharge avec la nouvelle configuration.'
else
    echo 'Location /docs deja configuree.'
    systemctl reload nginx
fi

echo 'Deploiement termine.'
EOF

echo -e "\n\033[32mDeploiement termine avec succes!\033[0m"
echo -e "\033[36mDocumentation accessible sur: https://$VPS_IP/docs/\033[0m"
echo "(ou https://origin.miyukini.com/docs/ si le domaine est configure)"
