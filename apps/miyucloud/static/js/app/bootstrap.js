/**
 * Miyukini Cloud - App bootstrap
 * Isolated startup trigger for the main application initializer.
 */

/*
 * @id     cog.cloudBootstrap
 * @do     amorcer_application
 * @role   orchestration
 * @layer  infra
 * @human  Amorçage isolé : déclenche l'initialisation de l'application une fois le DOM prêt.
 */
if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', () => {
        if (typeof window.initApp === 'function') {
            window.initApp();
        }
    });
} else if (typeof window.initApp === 'function') {
    window.initApp();
}
