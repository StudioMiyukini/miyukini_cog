/**
 * Search view orchestration logic
 */

/*
 * @id     cog.cloudSearchView
 * @do     rechercher_fichiers
 * @role   ui
 * @layer  ui
 * @human  Vue de recherche : interroge le serveur et affiche les fichiers correspondant à la requête.
 */
/*
 * @id     cog.cloudSearchView.performSearch
 * @do     lancer_recherche
 * @role   orchestration
 * @layer  ui
 * @human  Lance une recherche de fichiers et affiche les résultats triés.
 */
async function performSearch(query, sortBy) {
    const app = window.app;

    console.log(`Performing search for: "${query}" (sort: ${sortBy || 'relevance'})`);

    try {
        app.isSearchMode = true;
        window.ui.updateBreadcrumb(`Search: "${query}"`);

        const filesGrid = document.getElementById('files-grid');
        if (filesGrid) {
            filesGrid.innerHTML = `
                <div class="search-results-header">
                    <h3><i class="fas fa-spinner fa-spin search-spinner"></i> Searching for "${query}"...</h3>
                </div>
            `;
        }

        const options = {
            recursive: true,
            limit: 100,
            sort_by: sortBy || 'relevance'
        };

        if (!app.isTrashView) {
            // Ensure we have a valid folder_id before searching
            if (!app.currentPath || app.currentPath === '') {
                await window.resolveHomeFolder();
            }

            // Only set folder_id if we have a valid value
            if (app.currentPath && app.currentPath !== '') {
                options.folder_id = app.currentPath;
            }
            // If still no valid folder_id, search will be global (without folder_id)
        }

        const searchResults = await window.search.searchFiles(query, options);
        window.search.displaySearchResults(searchResults);

    } catch (error) {
        console.error('Search error:', error);
        window.ui.showNotification('Error', 'Error performing search');
    }
}

document.addEventListener('search-resort', (e) => {
    const searchInput = document.querySelector('.search-container input');
    if (searchInput && searchInput.value.trim()) {
        performSearch(searchInput.value.trim(), e.detail.sort_by);
    }
});

window.performSearch = performSearch;
