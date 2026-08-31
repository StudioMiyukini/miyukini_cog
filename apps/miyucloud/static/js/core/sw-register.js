/*
 * @id     cog.cloudSwRegister
 * @do     enregistrer_service_worker
 * @role   orchestration
 * @layer  infra
 * @human  Enregistre le service worker après le chargement de la page.
 */
// Service Worker registration — runs after page load.
if ('serviceWorker' in navigator) {
    window.addEventListener('load', function () {
        navigator.serviceWorker.register('/sw.js')
            .then(function () { /* registered */ })
            .catch(function (err) { console.log('Service Worker registration failed:', err); });
    });
}
