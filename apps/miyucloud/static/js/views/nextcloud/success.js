/*
 * @id     cog.cloudViewNcSuccess
 * @do     confirmer_liaison_nextcloud
 * @role   ui
 * @layer  ui
 * @human  Page de succès du pont Nextcloud : confirme la liaison et propose de fermer la fenêtre.
 */
document.getElementById('close-window-btn').addEventListener('click', function() {
    window.close();
});
// Auto-close after 3 seconds
setTimeout(function() {
    window.close();
}, 3000);
