/**
 * Miyukini Cloud - App state container
 * Centralized mutable state for app and cached DOM references.
 */

/*
 * @id     cog.cloudState
 * @do     detenir_etat_app
 * @role   donnee
 * @layer  domain
 * @human  État global de Miyukini Cloud : vue et dossier courants, sélections, et références DOM mises en cache.
 */
window.app = {
    currentView: 'grid',
    currentPath: '',
    currentFolder: null,
    contextMenuTargetFolder: null,
    contextMenuTargetFile: null,
    selectedTargetFolderId: '',
    moveDialogMode: 'file',
    isFilesView: true,
    isTrashView: false,
    isSharedView: false,
    isFavoritesView: false,
    isRecentView: false,
    isPhotosView: false,
    currentSection: 'files',
    isSearchMode: false,
    shareDialogItem: null,
    shareDialogItemType: null,
    notificationShareUrl: null,
    userHomeFolderId: null,
    userHomeFolderName: null,
    breadcrumbPath: [] // Array of {id, name} tracking folder navigation hierarchy
};

window.appElements = {
};
