/**
 * MiyuCloud -- Logique de la page de partage web.
 *
 * Zero framework, zero CDN (LOI-1).
 * Gere 3 etats : disponible, mot de passe requis, expire/revoque.
 *
 * Les donnees du fichier partage sont injectees par le serveur
 * dans un element <script id="share-data" type="application/json">.
 * Si absent, les donnees sont chargees via fetch depuis l'API.
 */
(function () {
    "use strict";

    // ================================================================
    // DOM references
    // ================================================================

    var stateLoading   = document.getElementById("state-loading");
    var stateAvailable = document.getElementById("state-available");
    var statePassword  = document.getElementById("state-password");
    var stateExpired   = document.getElementById("state-expired");

    var fileName       = document.getElementById("file-name");
    var fileSize       = document.getElementById("file-size");
    var fileExpires    = document.getElementById("file-expires");
    var downloadBtn    = document.getElementById("download-btn");
    var downloadCount  = document.getElementById("download-count");

    var passwordForm   = document.getElementById("password-form");
    var passwordInput  = document.getElementById("password-input");
    var passwordSubmit = document.getElementById("password-submit");
    var passwordError  = document.getElementById("password-error");

    var gateForm       = document.getElementById("gate-password-form");
    var gateInput      = document.getElementById("gate-password-input");
    var gateSubmit     = document.getElementById("gate-password-submit");
    var gateError      = document.getElementById("gate-password-error");

    var expiredMessage = document.getElementById("expired-message");

    // ================================================================
    // Utilities
    // ================================================================

    /**
     * Format a byte count into a human-readable string.
     * @param {number} bytes
     * @returns {string}
     */
    function formatSize(bytes) {
        if (bytes === 0) return "0 o";
        var units = ["o", "Ko", "Mo", "Go", "To"];
        var i = 0;
        var val = bytes;
        while (val >= 1024 && i < units.length - 1) {
            val /= 1024;
            i++;
        }
        if (i === 0) return val + " " + units[i];
        return val.toFixed(1) + " " + units[i];
    }

    /**
     * Format an ISO 8601 date into a French-readable string.
     * @param {string} iso
     * @returns {string}
     */
    function formatDate(iso) {
        if (!iso || iso.length < 16) return iso || "";
        try {
            var d = new Date(iso);
            if (isNaN(d.getTime())) return iso;
            var day   = String(d.getDate()).padStart(2, "0");
            var month = String(d.getMonth() + 1).padStart(2, "0");
            var year  = d.getFullYear();
            var hours = String(d.getHours()).padStart(2, "0");
            var mins  = String(d.getMinutes()).padStart(2, "0");
            return day + "/" + month + "/" + year + " " + hours + ":" + mins;
        } catch (e) {
            return iso;
        }
    }

    /**
     * Extract the share token from the current URL path.
     * Expected format: /share/{token}
     * @returns {string|null}
     */
    function getToken() {
        var parts = window.location.pathname.split("/");
        // /share/{token} => parts = ["", "share", "{token}"]
        for (var i = 0; i < parts.length; i++) {
            if (parts[i] === "share" && i + 1 < parts.length && parts[i + 1].length > 0) {
                return parts[i + 1];
            }
        }
        return null;
    }

    /**
     * Show a specific state and hide all others.
     * @param {"loading"|"available"|"password"|"expired"} state
     */
    function showState(state) {
        stateLoading.classList.toggle("hidden", state !== "loading");
        stateAvailable.classList.toggle("hidden", state !== "available");
        statePassword.classList.toggle("hidden", state !== "password");
        stateExpired.classList.toggle("hidden", state !== "expired");
    }

    // ================================================================
    // Share data management
    // ================================================================

    var shareToken = getToken();
    var shareData = null;
    var authenticated = false;

    /**
     * Try to read inline share data from the page.
     * The server may inject a JSON block with share metadata.
     * @returns {object|null}
     */
    function readInlineData() {
        var el = document.getElementById("share-data");
        if (!el) return null;
        try {
            return JSON.parse(el.textContent);
        } catch (e) {
            return null;
        }
    }

    /**
     * Fetch share metadata from the server.
     */
    function loadShareData() {
        if (!shareToken) {
            showState("expired");
            return;
        }

        // Try inline data first
        var inline = readInlineData();
        if (inline) {
            handleShareData(inline);
            return;
        }

        // Fetch from API
        showState("loading");

        var xhr = new XMLHttpRequest();
        xhr.open("GET", "/share/" + encodeURIComponent(shareToken), true);
        xhr.setRequestHeader("Accept", "application/json");
        xhr.onreadystatechange = function () {
            if (xhr.readyState !== 4) return;

            if (xhr.status === 200) {
                try {
                    var data = JSON.parse(xhr.responseText);
                    handleShareData(data);
                } catch (e) {
                    // The server returned HTML (the page itself), meaning
                    // data is not available via JSON. Show expired state.
                    showState("expired");
                }
            } else if (xhr.status === 401) {
                // Password required
                showState("password");
            } else if (xhr.status === 410) {
                showState("expired");
                if (expiredMessage) {
                    expiredMessage.textContent = "Ce lien de partage a expire ou a ete revoque.";
                }
            } else if (xhr.status === 404) {
                showState("expired");
                if (expiredMessage) {
                    expiredMessage.textContent = "Ce lien de partage n'existe pas.";
                }
            } else {
                showState("expired");
                if (expiredMessage) {
                    expiredMessage.textContent = "Erreur lors du chargement (code " + xhr.status + ").";
                }
            }
        };
        xhr.onerror = function () {
            showState("expired");
            if (expiredMessage) {
                expiredMessage.textContent = "Impossible de contacter le serveur.";
            }
        };
        xhr.send();
    }

    /**
     * Process share data and display the appropriate state.
     * @param {object} data
     */
    function handleShareData(data) {
        shareData = data;

        // Check for expiration/revocation
        if (data.is_revoked) {
            showState("expired");
            if (expiredMessage) {
                expiredMessage.textContent = "Ce lien de partage a ete revoque.";
            }
            return;
        }

        if (data.expires_at) {
            var expiresDate = new Date(data.expires_at);
            var now = new Date();
            if (expiresDate <= now) {
                showState("expired");
                if (expiredMessage) {
                    expiredMessage.textContent = "Ce lien de partage a expire le " + formatDate(data.expires_at) + ".";
                }
                return;
            }
        }

        // Check if password is required and not yet authenticated
        if (data.has_password && !authenticated) {
            showState("password");
            return;
        }

        // Show file info
        showState("available");
        if (fileName) fileName.textContent = data.file_name || "Fichier";
        if (fileSize) fileSize.textContent = formatSize(data.size_bytes || 0);
        if (fileExpires && data.expires_at) {
            fileExpires.textContent = "Expire le " + formatDate(data.expires_at);
        }

        // Show download count if max_downloads is set
        if (data.max_downloads && downloadCount) {
            var remaining = data.max_downloads - (data.download_count || 0);
            if (remaining <= 0) {
                downloadBtn.disabled = true;
                downloadCount.textContent = "Nombre maximum de telechargements atteint.";
                downloadCount.classList.remove("hidden");
            } else {
                downloadCount.textContent = remaining + " telechargement(s) restant(s)";
                downloadCount.classList.remove("hidden");
            }
        }

        // If password required, show the inline password form
        if (data.has_password && !authenticated) {
            passwordForm.classList.remove("hidden");
            downloadBtn.classList.add("hidden");
        } else {
            passwordForm.classList.add("hidden");
            downloadBtn.classList.remove("hidden");
        }
    }

    // ================================================================
    // Password authentication
    // ================================================================

    /**
     * Submit password for authentication.
     * @param {string} password
     * @param {HTMLElement} errorEl
     * @param {Function} onSuccess
     */
    function submitPassword(password, errorEl, onSuccess) {
        if (!shareToken || !password) {
            showError(errorEl, "Veuillez entrer un mot de passe.");
            return;
        }

        var xhr = new XMLHttpRequest();
        xhr.open("POST", "/share/" + encodeURIComponent(shareToken) + "/auth", true);
        xhr.setRequestHeader("Content-Type", "application/json");
        xhr.onreadystatechange = function () {
            if (xhr.readyState !== 4) return;

            if (xhr.status === 200) {
                authenticated = true;
                hideError(errorEl);
                if (onSuccess) onSuccess();
            } else if (xhr.status === 401 || xhr.status === 403) {
                showError(errorEl, "Mot de passe incorrect.");
            } else if (xhr.status === 429) {
                showError(errorEl, "Trop de tentatives. Reessayez dans quelques minutes.");
            } else {
                showError(errorEl, "Erreur d'authentification (code " + xhr.status + ").");
            }
        };
        xhr.onerror = function () {
            showError(errorEl, "Impossible de contacter le serveur.");
        };
        xhr.send(JSON.stringify({ password: password }));
    }

    function showError(el, msg) {
        if (el) {
            el.textContent = msg;
            el.classList.remove("hidden");
        }
    }

    function hideError(el) {
        if (el) {
            el.textContent = "";
            el.classList.add("hidden");
        }
    }

    // ================================================================
    // Download
    // ================================================================

    /**
     * Trigger file download.
     */
    function startDownload() {
        if (!shareToken) return;

        downloadBtn.disabled = true;
        downloadBtn.textContent = "Telechargement...";

        // Use a hidden link to trigger the download
        var url = "/share/" + encodeURIComponent(shareToken) + "/download";
        var link = document.createElement("a");
        link.href = url;
        link.style.display = "none";

        // Set download attribute with filename if available
        if (shareData && shareData.file_name) {
            link.download = shareData.file_name;
        }

        document.body.appendChild(link);
        link.click();

        // Re-enable after a short delay
        setTimeout(function () {
            document.body.removeChild(link);
            downloadBtn.disabled = false;
            downloadBtn.innerHTML =
                '<svg width="18" height="18" viewBox="0 0 18 18" fill="none" aria-hidden="true">' +
                '<path d="M9 2v10M9 12l-4-4M9 12l4-4" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"/>' +
                '<path d="M3 14h12" stroke="currentColor" stroke-width="2" stroke-linecap="round"/>' +
                '</svg> Telecharger';

            // Update download count display
            if (shareData && shareData.max_downloads && downloadCount) {
                var current = (shareData.download_count || 0) + 1;
                shareData.download_count = current;
                var remaining = shareData.max_downloads - current;
                if (remaining <= 0) {
                    downloadBtn.disabled = true;
                    downloadCount.textContent = "Nombre maximum de telechargements atteint.";
                } else {
                    downloadCount.textContent = remaining + " telechargement(s) restant(s)";
                }
                downloadCount.classList.remove("hidden");
            }
        }, 2000);
    }

    // ================================================================
    // Event listeners
    // ================================================================

    if (downloadBtn) {
        downloadBtn.addEventListener("click", function (e) {
            e.preventDefault();
            startDownload();
        });
    }

    if (gateForm) {
        gateForm.addEventListener("submit", function (e) {
            e.preventDefault();
            var pwd = gateInput ? gateInput.value : "";
            submitPassword(pwd, gateError, function () {
                // Re-load the share data now that we are authenticated
                loadShareData();
            });
        });
    }

    if (passwordForm) {
        passwordForm.addEventListener("submit", function (e) {
            e.preventDefault();
            var pwd = passwordInput ? passwordInput.value : "";
            submitPassword(pwd, passwordError, function () {
                passwordForm.classList.add("hidden");
                downloadBtn.classList.remove("hidden");
            });
        });
    }

    // ================================================================
    // Init
    // ================================================================

    loadShareData();

})();
