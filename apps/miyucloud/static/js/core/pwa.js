// PWA service worker registration + global handlers
if ('serviceWorker' in navigator) {
    navigator.serviceWorker.register('/sw.js').catch(function() {});
}

document.addEventListener('DOMContentLoaded', function() {
    // Nav mobile link
    var navMobile = document.getElementById('nav-mobile');
    if (navMobile) {
        navMobile.addEventListener('click', function() {
            window.location.href = '/mobile';
        });
    }

    // Upload photos button in dropdown menu
    var uploadPhotosBtn = document.getElementById('upload-photos-btn');
    if (uploadPhotosBtn) {
        var photosInput = document.createElement('input');
        photosInput.type = 'file';
        photosInput.multiple = true;
        photosInput.accept = 'image/*,video/*';
        photosInput.style.display = 'none';
        document.body.appendChild(photosInput);

        uploadPhotosBtn.addEventListener('click', function() {
            photosInput.value = '';
            photosInput.click();
        });

        photosInput.addEventListener('change', function() {
            var files = Array.from(photosInput.files);
            if (!files.length) return;

            // Use the existing file-input upload mechanism if available
            var fileInput = document.getElementById('file-input');
            if (fileInput) {
                // Create a new DataTransfer to set files on the existing input
                try {
                    var dt = new DataTransfer();
                    files.forEach(function(f) { dt.items.add(f); });
                    fileInput.files = dt.files;
                    fileInput.dispatchEvent(new Event('change', { bubbles: true }));
                } catch (e) {
                    // Fallback: upload directly via API
                    uploadPhotosDirectly(files);
                }
            } else {
                uploadPhotosDirectly(files);
            }
        });
    }
});

function uploadPhotosDirectly(files) {
    var done = 0, errors = 0, total = files.length;
    var notif = document.createElement('div');
    notif.style.cssText = 'position:fixed;bottom:20px;right:20px;z-index:9999;background:#f97316;color:white;padding:14px 20px;border-radius:12px;font-size:14px;font-weight:600;box-shadow:0 4px 16px rgba(0,0,0,0.2)';
    notif.textContent = '0 / ' + total + ' photos envoy\u00e9es...';
    document.body.appendChild(notif);

    var idx = 0, active = 0;
    function next() {
        while (active < 3 && idx < files.length) {
            (function(i) {
                var file = files[i];
                active++;
                var form = new FormData();
                form.append('file', file);
                var headers = {};
                if (typeof getCsrfHeaders === 'function') Object.assign(headers, getCsrfHeaders());
                fetch('/api/files/upload', {
                    method: 'POST', headers: headers, credentials: 'same-origin', body: form
                }).then(function(r) {
                    if (!r.ok) throw new Error();
                    done++;
                }).catch(function() {
                    errors++; done++;
                }).then(function() {
                    active--;
                    notif.textContent = done + ' / ' + total + ' photos envoy\u00e9es' + (errors ? ' (' + errors + ' erreurs)' : '');
                    if (done >= total) {
                        notif.style.background = errors ? '#ef4444' : '#22c55e';
                        notif.textContent = '\u2713 ' + (done - errors) + ' photos envoy\u00e9es' + (errors ? ', ' + errors + ' erreurs' : '');
                        setTimeout(function() { notif.remove(); }, 5000);
                        // Reload file list
                        if (typeof loadFiles === 'function') loadFiles();
                    }
                    next();
                });
            })(idx);
            idx++;
        }
    }
    next();
}
