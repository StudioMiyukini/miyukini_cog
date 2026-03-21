// Miyukini Cloud — Mobile page JS
// Handles OS detection, batch upload, and camera roll sync
// Optimized for large photo libraries (10k+ files)

// ── OS Detection ──
(function() {
    var ua = navigator.userAgent;
    var isIOS = /iPad|iPhone|iPod/.test(ua) || (navigator.platform === 'MacIntel' && navigator.maxTouchPoints > 1);
    var isAndroid = /Android/.test(ua);
    var detected = document.getElementById('detected-os');
    if (isIOS) {
        detected.innerHTML = '<i class="fab fa-apple"></i> iOS d\u00e9tect\u00e9';
        document.getElementById('card-ios').classList.add('active');
    } else if (isAndroid) {
        detected.innerHTML = '<i class="fab fa-android"></i> Android d\u00e9tect\u00e9';
        document.getElementById('card-android').classList.add('active');
    } else {
        detected.innerHTML = '<i class="fas fa-desktop"></i> Desktop d\u00e9tect\u00e9';
        document.getElementById('card-desktop').classList.add('active');
    }
    var url = location.hostname;
    ['ios-url', 'android-url', 'desktop-url'].forEach(function(id) {
        var el = document.getElementById(id);
        if (el) el.textContent = url;
    });

    // Adapt upload button text for mobile
    var label = document.getElementById('auto-upload-label');
    var hint = document.getElementById('auto-upload-hint');
    if (isIOS || isAndroid) {
        if (label) label.textContent = 'S\u00e9lectionner les photos \u00e0 envoyer';
        if (hint) hint.textContent = 'Ouvrez votre galerie, s\u00e9lectionnez tout, seules les nouvelles seront envoy\u00e9es';
    }
})();

// ── DOM Elements ──
var batchInput = document.getElementById('batch-file-input');
var batchBtn = document.getElementById('batch-upload-btn');
var uploadList = document.getElementById('upload-list');
var uploadSummary = document.getElementById('upload-summary');
var autoUploadBtn = document.getElementById('auto-upload-btn');
var cameraTakeBtn = document.getElementById('camera-take-btn');
var cameraTakeInput = document.getElementById('camera-take-input');
var cameraFolderInput = document.getElementById('camera-folder-input');
var syncStatus = document.getElementById('sync-status');
var syncLabel = document.getElementById('sync-label');
var syncCount = document.getElementById('sync-count');
var syncProgress = document.getElementById('sync-progress');
var syncList = document.getElementById('sync-list');
var syncSummaryEl = document.getElementById('sync-summary');

// ── Config ──
var CONCURRENT_UPLOADS = 3;
var BATCH_DISPLAY_LIMIT = 200;
var MODAL_FILES_LIMIT = 50; // Recent files shown in modal
var MEDIA_EXTENSIONS = /\.(jpe?g|png|gif|webp|heic|heif|avif|bmp|tiff?|mp4|mov|avi|mkv|webm|3gp|m4v)$/i;

// ── Upload Modal ──
var modal = document.getElementById('upload-modal');
var modalTitle = document.getElementById('modal-title');
var modalSubtitle = document.getElementById('modal-subtitle');
var modalPct = document.getElementById('modal-pct');
var modalProgressBar = document.getElementById('modal-progress-bar');
var modalScanned = document.getElementById('modal-scanned');
var modalNew = document.getElementById('modal-new');
var modalDone = document.getElementById('modal-done');
var modalErrors = document.getElementById('modal-errors');
var modalFiles = document.getElementById('modal-files');
var modalClose = document.getElementById('modal-close');
var modalHandle = document.getElementById('modal-handle');
var modalExpanded = true;

function showModal(title) {
    modal.classList.remove('hidden');
    modal.style.display = 'flex';
    modalTitle.textContent = title || 'Upload en cours';
    modalSubtitle.textContent = 'Initialisation...';
    modalPct.textContent = '0%';
    modalProgressBar.style.width = '0%';
    modalScanned.textContent = '0';
    modalNew.textContent = '0';
    modalDone.textContent = '0';
    modalErrors.textContent = '0';
    modalFiles.innerHTML = '';
}

function hideModal() {
    modal.classList.add('hidden');
    modal.style.display = 'none';
}

function updateModal(stats) {
    var pct = stats.total > 0 ? Math.round((stats.done / stats.total) * 100) : 0;
    modalPct.textContent = pct + '%';
    modalProgressBar.style.width = pct + '%';
    modalScanned.textContent = stats.scanned || 0;
    modalNew.textContent = stats.queued || stats.total || 0;
    modalDone.textContent = stats.done || 0;
    modalErrors.textContent = stats.errors || 0;

    if (stats.phase === 'scanning') {
        modalSubtitle.textContent = 'Scan en cours... ' + stats.scanned + ' fichiers';
        modalTitle.textContent = 'Analyse de la pellicule';
    } else if (stats.phase === 'comparing') {
        modalSubtitle.textContent = 'Comparaison avec le cloud...';
    } else if (stats.phase === 'uploading') {
        modalSubtitle.textContent = stats.done + ' / ' + stats.total + ' envoy\u00e9s';
        modalTitle.textContent = 'Upload en cours';
    } else if (stats.phase === 'done') {
        modalSubtitle.textContent = 'Termin\u00e9 !';
        modalTitle.textContent = 'Synchronisation termin\u00e9e';
        modalPct.textContent = '\u2713';
        modalPct.style.color = '#22c55e';
        modalProgressBar.style.width = '100%';
        modalProgressBar.style.background = '#22c55e';
    }
}

function addModalFile(name, status) {
    // Keep only recent files
    while (modalFiles.children.length >= MODAL_FILES_LIMIT) {
        modalFiles.removeChild(modalFiles.firstChild);
    }
    var row = document.createElement('div');
    row.style.cssText = 'display:flex;align-items:center;gap:8px;padding:6px 0;font-size:12px;border-bottom:1px solid var(--border-color,#f1f5f9)';
    var icon = status === 'ok' ? '<i class="fas fa-check" style="color:#22c55e"></i>'
             : status === 'err' ? '<i class="fas fa-times" style="color:#ef4444"></i>'
             : '<i class="fas fa-spinner fa-spin" style="color:#f97316"></i>';
    row.innerHTML = icon + '<span style="flex:1;overflow:hidden;text-overflow:ellipsis;white-space:nowrap;color:var(--text-primary,#0f172a)">' + escapeHtml(name) + '</span>';
    modalFiles.appendChild(row);
    modalFiles.scrollTop = modalFiles.scrollHeight;
}

// Toggle expand/collapse
if (modalHandle) {
    modalHandle.addEventListener('click', function() {
        modalExpanded = !modalExpanded;
        var filesEl = document.getElementById('modal-files');
        var statsEl = modalFiles.previousElementSibling;
        if (modalExpanded) {
            if (filesEl) filesEl.style.display = '';
        } else {
            if (filesEl) filesEl.style.display = 'none';
        }
    });
}

if (modalClose) {
    modalClose.addEventListener('click', hideModal);
}

// ── Manual file picker ──
batchBtn.addEventListener('click', function() { batchInput.click(); });
batchInput.addEventListener('change', function() {
    var files = Array.from(batchInput.files);
    if (!files.length) return;
    startBatchUpload(files, 'st', uploadList, uploadSummary);
});

// ── Take photo ──
cameraTakeBtn.addEventListener('click', function() { cameraTakeInput.click(); });
cameraTakeInput.addEventListener('change', function() {
    if (cameraTakeInput.files.length) {
        var file = cameraTakeInput.files[0];
        syncList.innerHTML = '';
        addUploadItem(file, 0, 'ss', syncList);
        uploadFile(file).then(function() { markDone('ss-0', true); }).catch(function() { markDone('ss-0', false); });
    }
});

// ── Auto-upload (camera roll sync) ──
// On mobile (Android/iOS), showDirectoryPicker and webkitdirectory don't work
// reliably for DCIM/Camera. Use the native media picker instead.
var isMobile = /Android|iPhone|iPad|iPod/i.test(navigator.userAgent);

// Hidden input for mobile gallery picker (no webkitdirectory, just multi-select media)
var galleryInput = document.createElement('input');
galleryInput.type = 'file';
galleryInput.multiple = true;
galleryInput.accept = 'image/*,video/*';
galleryInput.style.display = 'none';
document.body.appendChild(galleryInput);

galleryInput.addEventListener('change', function() {
    var files = Array.from(galleryInput.files);
    if (!files.length) { resetAutoBtn(); return; }
    syncWithCloudComparison(files);
});

autoUploadBtn.addEventListener('click', function() {
    // Disable button during scan
    autoUploadBtn.disabled = true;
    autoUploadBtn.style.opacity = '0.6';
    syncStatus.classList.remove('hidden');
    syncLabel.textContent = 'Pr\u00e9paration...';
    syncCount.textContent = '';
    syncProgress.style.width = '0%';
    syncList.innerHTML = '';
    syncSummaryEl.classList.add('hidden');

    if (isMobile) {
        // Mobile: use native gallery picker (most reliable on Android/iOS)
        galleryInput.value = '';
        galleryInput.click();
        // If user cancels, re-enable button after a delay
        setTimeout(function() {
            if (!galleryInput.files || !galleryInput.files.length) resetAutoBtn();
        }, 1000);
    } else if ('showDirectoryPicker' in window) {
        // Desktop: use File System Access API for directory scanning
        window.showDirectoryPicker({ mode: 'read' }).then(function(dirHandle) {
            streamSyncFromDirectory(dirHandle);
        }).catch(function(e) {
            if (e.name === 'AbortError') { resetAutoBtn(); return; }
            console.warn('Directory picker failed, falling back:', e);
            cameraFolderInput.click();
        });
    } else {
        // Fallback: webkitdirectory (desktop browsers without showDirectoryPicker)
        cameraFolderInput.click();
    }
});

cameraFolderInput.addEventListener('change', function() {
    var allFiles = Array.from(cameraFolderInput.files);
    var mediaFiles = allFiles.filter(isMediaFile);
    if (!mediaFiles.length) {
        alert('Aucune photo ou vid\u00e9o trouv\u00e9e.');
        resetAutoBtn();
        return;
    }
    syncWithCloudComparison(mediaFiles);
});

function resetAutoBtn() {
    autoUploadBtn.disabled = false;
    autoUploadBtn.style.opacity = '1';
}

// ═══════════════════════════════════════════════════════════════
// STREAMING SYNC — scans directory progressively, uploads as it goes
// ═══════════════════════════════════════════════════════════════
function streamSyncFromDirectory(dirHandle) {
    showModal('Synchronisation photos');
    updateModal({ phase: 'comparing', scanned: 0, queued: 0, done: 0, errors: 0, total: 0 });
    syncLabel.textContent = 'Chargement de la liste des fichiers cloud...';

    // 1. Fetch existing file names from cloud FIRST
    fetchExistingFileNames().then(function(existingNames) {
        syncLabel.textContent = existingNames.size + ' fichiers d\u00e9j\u00e0 sur le cloud. Scan du dossier...';

        // 2. Scan directory and upload new files as we find them
        var stats = { phase: 'scanning', scanned: 0, skipped: 0, queued: 0, done: 0, errors: 0, total: 0 };
        var uploadQueue = [];
        var uploading = false;

        function processFile(file) {
            stats.scanned++;
            stats.phase = 'scanning';
            if (stats.scanned % 50 === 0) {
                syncLabel.textContent = stats.scanned + ' fichiers analys\u00e9s, ' + stats.queued + ' nouveaux...';
                updateModal(stats);
            }

            if (existingNames.has(file.name.toLowerCase())) {
                stats.skipped++;
                return;
            }

            stats.queued++;
            stats.total = stats.queued;
            uploadQueue.push(file);

            // Show in UI (limit DOM nodes)
            if (stats.queued <= BATCH_DISPLAY_LIMIT) {
                addUploadItem(file, stats.queued - 1, 'ss', syncList);
            }

            // Start uploading if not already
            if (!uploading) {
                uploading = true;
                startQueueProcessing(uploadQueue, stats);
            }
        }

        scanDirectory(dirHandle, processFile, 0).then(function() {
            // Scan complete
            syncLabel.textContent = 'Scan termin\u00e9 : ' + stats.scanned + ' fichiers, ' + stats.queued + ' nouveaux, ' + stats.skipped + ' d\u00e9j\u00e0 pr\u00e9sents';
            syncCount.textContent = stats.skipped + ' d\u00e9j\u00e0 pr\u00e9sents';

            if (stats.queued === 0) {
                syncLabel.textContent = 'Tout est synchronis\u00e9 ! (' + stats.scanned + ' fichiers analys\u00e9s)';
                syncProgress.style.width = '100%';
                syncSummaryEl.classList.remove('hidden');
                syncSummaryEl.textContent = '\u2713 ' + stats.skipped + ' fichiers d\u00e9j\u00e0 sur le cloud.';
                stats.phase = 'done';
                updateModal(stats);
                resetAutoBtn();
            }
            // If there are still uploads pending, the queue processor will finish them
        }).catch(function(e) {
            console.error('Scan error:', e);
            syncLabel.textContent = 'Erreur pendant le scan : ' + e.message;
            resetAutoBtn();
        });
    });
}

// ── Scan directory recursively, call onFile for each media file ──
function scanDirectory(dirHandle, onFile, depth) {
    if (depth > 4) return Promise.resolve();

    return (async function() {
        for await (var entry of dirHandle.values()) {
            if (entry.kind === 'file') {
                try {
                    var file = await entry.getFile();
                    if (isMediaFile(file)) {
                        onFile(file);
                    }
                } catch (e) {
                    // Permission denied or file inaccessible — skip
                }
            } else if (entry.kind === 'directory' && depth < 3) {
                // Skip system directories
                var name = entry.name.toLowerCase();
                if (name === '.thumbnails' || name === '.trash' || name === 'thumbs.db' || name.startsWith('.')) continue;
                await scanDirectory(entry, onFile, depth + 1);
            }
        }
    })();
}

// ── Process upload queue with concurrency control ──
function startQueueProcessing(queue, stats) {
    var activeWorkers = 0;
    var queueIdx = 0;

    function processNext() {
        if (queueIdx >= queue.length) {
            // Check if more items might be added (scan still running)
            setTimeout(function() {
                if (queueIdx < queue.length) {
                    processNext();
                } else if (activeWorkers === 0) {
                    // All done
                    onAllDone();
                }
            }, 500);
            return;
        }

        while (activeWorkers < CONCURRENT_UPLOADS && queueIdx < queue.length) {
            (function(idx) {
                var file = queue[idx];
                activeWorkers++;
                uploadFile(file).then(function() {
                    stats.done++;
                    stats.phase = 'uploading';
                    if (idx < BATCH_DISPLAY_LIMIT) markDone('ss-' + idx, true);
                    addModalFile(file.name, 'ok');
                }).catch(function() {
                    stats.errors++;
                    stats.done++;
                    stats.phase = 'uploading';
                    if (idx < BATCH_DISPLAY_LIMIT) markDone('ss-' + idx, false);
                    addModalFile(file.name, 'err');
                }).then(function() {
                    activeWorkers--;
                    updateProgress(stats);
                    updateModal(stats);
                    processNext();
                });
            })(queueIdx);
            queueIdx++;
        }
    }

    function onAllDone() {
        syncSummaryEl.classList.remove('hidden');
        syncSummaryEl.textContent = '\u2713 ' + (stats.done - stats.errors) + ' envoy\u00e9s, ' +
            stats.skipped + ' d\u00e9j\u00e0 pr\u00e9sents' +
            (stats.errors ? ', ' + stats.errors + ' erreurs' : '');
        syncLabel.textContent = 'Synchronisation termin\u00e9e';
        syncProgress.style.width = '100%';
        stats.phase = 'done';
        updateModal(stats);
        resetAutoBtn();
    }

    processNext();
}

function updateProgress(stats) {
    if (stats.total > 0) {
        var pct = Math.round((stats.done / stats.total) * 100);
        syncProgress.style.width = pct + '%';
        syncLabel.textContent = stats.done + ' / ' + stats.total + ' envoy\u00e9s' +
            (stats.errors ? ' (' + stats.errors + ' erreurs)' : '');
        syncCount.textContent = stats.skipped + ' d\u00e9j\u00e0 pr\u00e9sents';
    }
}

// ═══════════════════════════════════════════════════════════════
// FALLBACK SYNC — for webkitdirectory input (iOS, older browsers)
// ═══════════════════════════════════════════════════════════════
function syncWithCloudComparison(localFiles) {
    syncStatus.classList.remove('hidden');
    syncLabel.textContent = 'V\u00e9rification de ' + localFiles.length + ' fichiers...';

    fetchExistingFileNames().then(function(existingNames) {
        var newFiles = localFiles.filter(function(f) { return !existingNames.has(f.name.toLowerCase()); });
        var skipped = localFiles.length - newFiles.length;

        if (!newFiles.length) {
            syncLabel.textContent = 'Tout est synchronis\u00e9 !';
            syncCount.textContent = skipped + ' d\u00e9j\u00e0 pr\u00e9sents';
            syncProgress.style.width = '100%';
            syncSummaryEl.classList.remove('hidden');
            syncSummaryEl.textContent = '\u2713 ' + skipped + ' fichiers d\u00e9j\u00e0 sur le cloud.';
            resetAutoBtn();
            return;
        }

        var stats = { scanned: localFiles.length, skipped: skipped, queued: newFiles.length, done: 0, errors: 0, total: newFiles.length };
        syncLabel.textContent = 'Envoi de ' + newFiles.length + ' nouveaux fichiers...';
        syncCount.textContent = skipped + ' d\u00e9j\u00e0 pr\u00e9sents';

        newFiles.slice(0, BATCH_DISPLAY_LIMIT).forEach(function(file, i) {
            addUploadItem(file, i, 'ss', syncList);
        });

        startQueueProcessing(newFiles, stats);
    });
}

// ═══════════════════════════════════════════════════════════════
// BATCH UPLOAD — simple multi-file upload (no cloud comparison)
// ═══════════════════════════════════════════════════════════════
function startBatchUpload(files, prefix, listEl, summaryEl) {
    listEl.innerHTML = '';
    summaryEl.classList.remove('hidden');
    summaryEl.textContent = '0 / ' + files.length + ' envoy\u00e9s...';
    showModal('Upload de ' + files.length + ' fichiers');
    var batchStats = { phase: 'uploading', scanned: files.length, queued: files.length, done: 0, errors: 0, total: files.length };
    updateModal(batchStats);
    var done = 0, errors = 0;
    var queue = files.slice();
    var idx = 0;
    var active = 0;

    function next() {
        while (active < CONCURRENT_UPLOADS && idx < queue.length) {
            (function(i) {
                var file = queue[i];
                active++;
                if (i < BATCH_DISPLAY_LIMIT) addUploadItem(file, i, prefix, listEl);
                uploadFile(file).then(function() {
                    done++;
                    if (i < BATCH_DISPLAY_LIMIT) markDone(prefix + '-' + i, true);
                    addModalFile(file.name, 'ok');
                }).catch(function() {
                    errors++; done++;
                    if (i < BATCH_DISPLAY_LIMIT) markDone(prefix + '-' + i, false);
                    addModalFile(file.name, 'err');
                }).then(function() {
                    active--;
                    batchStats.done = done; batchStats.errors = errors;
                    if (done >= files.length) batchStats.phase = 'done';
                    updateModal(batchStats);
                    summaryEl.textContent = done + ' / ' + files.length + ' envoy\u00e9s' + (errors ? ' (' + errors + ' erreurs)' : '');
                    next();
                });
            })(idx);
            idx++;
        }
    }
    next();
}

// ═══════════════════════════════════════════════════════════════
// SHARED HELPERS
// ═══════════════════════════════════════════════════════════════

function fetchExistingFileNames() {
    var headers = {};
    if (typeof getCsrfHeaders === 'function') Object.assign(headers, getCsrfHeaders());

    // Fetch in pages to handle large file counts
    var allNames = new Set();
    var limit = 1000;

    function fetchPage(offset) {
        return fetch('/api/files?limit=' + limit + '&offset=' + offset, {
            credentials: 'same-origin', headers: headers
        }).then(function(r) { return r.ok ? r.json() : { files: [] }; })
        .then(function(data) {
            var files = data.files || data || [];
            files.forEach(function(f) { if (f.name) allNames.add(f.name.toLowerCase()); });
            // If we got a full page, there might be more
            if (files.length >= limit) {
                return fetchPage(offset + limit);
            }
            return allNames;
        });
    }

    return fetchPage(0).catch(function() { return allNames; });
}

function uploadFile(file) {
    var form = new FormData();
    form.append('file', file);
    var headers = {};
    if (typeof getCsrfHeaders === 'function') Object.assign(headers, getCsrfHeaders());
    return fetch('/api/files/upload', {
        method: 'POST',
        headers: headers,
        credentials: 'same-origin',
        body: form,
    }).then(function(resp) {
        if (!resp.ok) throw new Error(resp.statusText);
        return resp.json();
    });
}

function isMediaFile(file) {
    if (file.type && (file.type.startsWith('image/') || file.type.startsWith('video/'))) return true;
    return MEDIA_EXTENSIONS.test(file.name);
}

function addUploadItem(file, i, prefix, listEl) {
    var item = document.createElement('div');
    item.className = 'upload-item';
    item.innerHTML =
        '<div class="status pending" id="' + prefix + '-' + i + '"><i class="fas fa-spinner fa-spin"></i></div>' +
        '<div class="name">' + escapeHtml(file.name) + '</div>' +
        '<div class="size">' + formatSize(file.size) + '</div>';
    listEl.appendChild(item);
}

function markDone(id, ok) {
    var el = document.getElementById(id);
    if (!el) return;
    el.className = 'status ' + (ok ? 'ok' : 'err');
    el.innerHTML = '<i class="fas fa-' + (ok ? 'check' : 'times') + '"></i>';
}

function formatSize(bytes) {
    if (bytes === 0) return '0 B';
    var k = 1024, s = ['B', 'KB', 'MB', 'GB'];
    var i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + s[i];
}

function escapeHtml(str) {
    var d = document.createElement('div');
    d.textContent = str;
    return d.innerHTML;
}

// ── PWA ──
if ('serviceWorker' in navigator) {
    navigator.serviceWorker.register('/sw.js').catch(function() {});
}
