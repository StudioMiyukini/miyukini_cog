const fs = require('fs');
const path = require('path');
const fse = require('fs-extra');

const initPath = path.resolve(__dirname, './node_modules/');
const outputPath = path.resolve(__dirname, './assets/dist/libs');

// Copy specific files from node_modules to vendor folder
const assets = {
  flatpickr: {
    src: [
      'flatpickr/dist/flatpickr.js',
      'flatpickr/dist/flatpickr.css'
    ]
  },
  flyonui: {
    src: [
      'flyonui/flyonui.js'
    ]
  },
  'tailwindcss-intersect': {
    src: [
      'tailwindcss-intersect/dist/observer.min.js'
    ]
  }
};

function copyAssets() {
  Object.keys(assets).forEach(key => {
    const asset = assets[key];

    asset.src.forEach(file => {
      const src = path.resolve(initPath, file);
      const dest = path.resolve(outputPath, file);

      fse.ensureDirSync(path.dirname(dest));

      fse.copy(src, dest, { overwrite: true }, err => {
        if (err) {
          console.error('Error copying file:', err);
        }
      });
    });
  });
}

copyAssets();
