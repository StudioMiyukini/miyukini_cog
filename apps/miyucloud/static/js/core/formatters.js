/**
 * Miyukini Cloud - Shared format and escaping utilities
 * Centralized global helpers for date/size/text formatting and XSS-safe escaping.
 */

/*
 * @id     cog.cloudFormatters
 * @do     formater_et_echapper
 * @role   securite
 * @layer  outil
 * @human  Utilitaires partagés de Miyukini Cloud : mise en forme des dates, tailles et textes, et échappement anti-XSS.
 */
/*
 * @id     cog.cloudFormatters.escapeHtml
 * @do     echapper_html
 * @role   securite
 * @layer  outil
 * @human  Échappe une chaîne pour l'insérer sans risque dans du HTML.
 */
function escapeHtml(str) {
    if (typeof str !== 'string') return '';
    return str
        .replace(/&/g, '&amp;')
        .replace(/</g, '&lt;')
        .replace(/>/g, '&gt;')
        .replace(/\"/g, '&quot;')
        .replace(/'/g, '&#039;');
}

/*
 * @id     cog.cloudFormatters.formatFileSize
 * @do     formater_taille_fichier
 * @role   ui
 * @layer  outil
 * @human  Met en forme une taille en octets en unité lisible (Ko, Mo, Go…).
 */
function formatFileSize(bytes) {
    if (bytes === 0) return '0 Bytes';

    const k = 1024;
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));

    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

/// Formats a byte count for quota display. When bytes is 0, returns "∞" (unlimited).
/*
 * @id     cog.cloudFormatters.formatQuotaSize
 * @do     formater_quota
 * @role   ui
 * @layer  outil
 * @human  Met en forme une taille de quota de stockage pour l'affichage.
 */
function formatQuotaSize(bytes) {
    if (bytes === 0) return '∞';
    return formatFileSize(bytes);
}

/*
 * @id     cog.cloudFormatters.formatDateTime
 * @do     formater_date_heure
 * @role   ui
 * @layer  outil
 * @human  Met en forme une date avec l'heure.
 */
function formatDateTime(value) {
    if (!value) return '';
    let dateValue;
    if (value instanceof Date) {
        dateValue = value;
    } else if (typeof value === 'number') {
        dateValue = new Date(value < 1e12 ? value * 1000 : value);
    } else {
        dateValue = new Date(value);
    }
    if (isNaN(dateValue.getTime())) return String(value);
    return dateValue.toLocaleDateString() + ' ' +
        dateValue.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
}

/*
 * @id     cog.cloudFormatters.formatDateShort
 * @do     formater_date_courte
 * @role   ui
 * @layer  outil
 * @human  Met en forme une date en version courte.
 */
function formatDateShort(value) {
    if (!value) return 'N/A';
    const dateValue = typeof value === 'number' ? new Date(value * 1000) : new Date(value);
    if (isNaN(dateValue.getTime())) return String(value);
    return dateValue.toLocaleDateString(undefined, { year: 'numeric', month: 'short', day: 'numeric' });
}

/*
 * @id     cog.cloudFormatters.isTextViewable
 * @do     tester_texte_affichable
 * @role   rule
 * @layer  outil
 * @human  Dit si un type MIME peut être affiché comme texte dans l'aperçu.
 */
function isTextViewable(mimeType) {
    if (!mimeType) return false;
    if (mimeType.startsWith('text/')) return true;
    const textTypes = [
        'application/json', 'application/xml', 'application/javascript',
        'application/x-sh', 'application/x-yaml', 'application/toml',
        'application/x-toml', 'application/sql',
    ];
    return textTypes.includes(mimeType);
}

window.escapeHtml = escapeHtml;
window.formatFileSize = formatFileSize;
window.formatQuotaSize = formatQuotaSize;
window.formatDateTime = formatDateTime;
window.formatDateShort = formatDateShort;
window.isTextViewable = isTextViewable;
