#!/bin/bash
# Forcer le français sur tout le forum

# 1. Langue par défaut du forum
mysql miyukinibb -e "UPDATE phpbb_config SET config_value = 'fr' WHERE config_name = 'default_lang';"

# 2. Forcer la langue pour tous les utilisateurs (override préférence user)
mysql miyukinibb -e "UPDATE phpbb_config SET config_value = '1' WHERE config_name = 'override_user_lang';" 2>/dev/null || true

# 3. Mettre à jour la langue de tous les utilisateurs existants
mysql miyukinibb -e "UPDATE phpbb_users SET user_lang = 'fr' WHERE user_lang != 'fr';"

# 4. Vérifier que fr existe dans phpbb_lang
EXISTS=$(mysql -N -B miyukinibb -e "SELECT COUNT(*) FROM phpbb_lang WHERE lang_iso='fr';")
if [ "$EXISTS" = "0" ]; then
  mysql miyukinibb -e "INSERT INTO phpbb_lang (lang_iso, lang_dir, lang_english_name, lang_local_name, lang_author) VALUES ('fr', 'ltr', 'French', 'Français', 'phpBB-fr');"
fi

echo "Config:"
mysql miyukinibb -e "SELECT config_name, config_value FROM phpbb_config WHERE config_name IN ('default_lang', 'override_user_lang');"

cd /var/www/forum
php bin/phpbbcli.php cache:purge

echo DONE
