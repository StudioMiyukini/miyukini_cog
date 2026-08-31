/**
 * CSRF double-submit cookie utility.
 *
 * Reads the `miyucloud_csrf` cookie (which is NOT HttpOnly) and provides
 * its value as the `X-CSRF-Token` header on mutating requests.
 *
 * Usage:
 *   // In any fetch call that changes state:
 *   fetch(url, { method: 'POST', headers: { ...getCsrfHeaders(), 'Content-Type': 'application/json' } })
 *
 * The server-side `csrf_middleware` validates that the header value matches
 * the cookie for every POST/PUT/DELETE/PATCH request authenticated via
 * HttpOnly cookies.
 */

// eslint-disable-next-line no-unused-vars
/*
 * @id     cog.cloudCsrf
 * @do     proteger_csrf
 * @role   securite
 * @layer  infra
 * @human  Protection CSRF de Miyukini Cloud : lit le jeton et compose les en-têtes à joindre aux requêtes qui modifient l'état.
 */
/*
 * @id     cog.cloudCsrf.getCsrfToken
 * @do     lire_jeton_csrf
 * @role   securite
 * @layer  infra
 * @human  Lit le jeton CSRF courant (cookie/méta).
 */
function getCsrfToken() {
    const match = document.cookie
        .split('; ')
        .find(row => row.startsWith('miyucloud_csrf='));
    return match ? match.split('=')[1] : '';
}

// eslint-disable-next-line no-unused-vars
/*
 * @id     cog.cloudCsrf.getCsrfHeaders
 * @do     composer_entetes_csrf
 * @role   securite
 * @layer  infra
 * @human  Compose les en-têtes HTTP portant le jeton CSRF.
 */
function getCsrfHeaders() {
    const token = getCsrfToken();
    return token ? { 'X-CSRF-Token': token } : {};
}
