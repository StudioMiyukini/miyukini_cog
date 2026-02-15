#!/bin/bash
# Setup nginx for docs

set -e

NGINX_CONF="/etc/nginx/sites-available/origin-miyukini.conf"
NGINX_DEFAULT="/etc/nginx/sites-enabled/default"

# Check if location /docs already exists
if grep -q 'location /docs' "$NGINX_CONF" 2>/dev/null; then
    echo "Location /docs already configured in nginx"
else
    echo "Adding /docs location to nginx config..."
    
    # Backup
    cp "$NGINX_CONF" "${NGINX_CONF}.bak.$(date +%Y%m%d%H%M%S)"
    
    # Insert docs location before "location / {" in the 443 server block
    # Using a temp file approach for reliability
    cat > /tmp/docs_location.txt << 'LOCATION_BLOCK'

    location /docs {
        alias /var/www/docs/;
        index index.html;
        try_files $uri $uri/ /docs/index.html;
        add_header Cache-Control "public, max-age=3600";
        add_header X-Content-Type-Options nosniff always;
        add_header X-Frame-Options SAMEORIGIN always;
    }

LOCATION_BLOCK
    
    # Find line number of "location / {" after listen 443 and insert before it
    # This is a simple approach - insert before the first "location / {"
    awk '
    /location \/ \{/ && !inserted {
        system("cat /tmp/docs_location.txt")
        inserted=1
    }
    { print }
    ' "$NGINX_CONF" > /tmp/nginx_new.conf
    
    mv /tmp/nginx_new.conf "$NGINX_CONF"
    echo "Location /docs added"
fi

# Test and reload nginx
echo "Testing nginx configuration..."
nginx -t

echo "Reloading nginx..."
systemctl reload nginx

echo ""
echo "=== Setup complete ==="
echo "Documentation available at: https://46.202.129.65/docs/"
echo "Or: https://origin.miyukini.com/docs/ (if domain configured)"
