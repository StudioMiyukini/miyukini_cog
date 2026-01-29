/* MiyukiniAdmin — Daynight theme toggle (vanilla JS, localStorage) */
(function () {
  var STORAGE_KEY = 'miyukini-admin-theme';
  var DARK = 'dark';
  var LIGHT = 'light';

  function getStored() {
    try {
      return localStorage.getItem(STORAGE_KEY) || DARK;
    } catch (_) {
      return DARK;
    }
  }

  function setTheme(theme) {
    document.documentElement.setAttribute('data-theme', theme);
    try {
      localStorage.setItem(STORAGE_KEY, theme);
    } catch (_) {}
    var btn = document.getElementById('theme-toggle');
    if (btn) btn.textContent = theme === DARK ? 'Mode clair' : 'Mode nuit';
  }

  function init() {
    var theme = getStored();
    setTheme(theme);
    var btn = document.getElementById('theme-toggle');
    if (btn) {
      btn.addEventListener('click', function () {
        var next = document.documentElement.getAttribute('data-theme') === DARK ? LIGHT : DARK;
        setTheme(next);
      });
    }
  }

  if (document.readyState === 'loading') {
    document.addEventListener('DOMContentLoaded', init);
  } else {
    init();
  }
})();
