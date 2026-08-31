/*
 * @id     cog.docsComponent
 * @do     neutraliser_liens_apercu
 * @role   ui
 * @layer  doc
 * @human  Script d'aperçu UX/UI (HyperUI) : neutralise les liens et champs des composants montrés en démo.
 */
document.addEventListener('DOMContentLoaded', () => {
  document.querySelectorAll('a, input[type="file"]').forEach((iframeLink) => {
    iframeLink.addEventListener('click', (e) => e.preventDefault())
    iframeLink.addEventListener('keydown', (e) => e.key !== 'Tab' && e.preventDefault())
  })

  document.querySelectorAll('form').forEach((iframeForm) => {
    iframeForm.addEventListener('submit', (e) => e.preventDefault())
  })
})
