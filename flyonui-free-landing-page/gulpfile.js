const fs = require('fs');
const path = require('path');
const gulp = require('gulp');
const { exec } = require('child_process');

function buildCss(done) {
  exec('npx @tailwindcss/cli -i ./assets/css/main.css -o ./assets/dist/css/output.css', (err, stdout, stderr) => {
    if (err) return done(err);
    done();
  });
}

function minifyCSS(done) {
  exec(
    'npx @tailwindcss/cli -i ./assets/css/main.css -o ./assets/dist/css/output.css --minify',
    (err, stdout, stderr) => {
      if (err) return done(err);
      done();
    }
  );
}

function watchCSS(done) {
  exec(
    'npx @tailwindcss/cli -i ./assets/css/main.css -o ./assets/dist/css/output.css --watch',
    (err, stdout, stderr) => {
      if (err) console.error(`Error watching CSS: ${err}`);
    }
  );
  done();
}

function copyJS(done) {
  const distJsDir = './assets/dist/js';
  if (!fs.existsSync(distJsDir)) {
    fs.mkdirSync(distJsDir, { recursive: true });
  }

  function copyFiles(srcDir, destDir) {
    const files = fs.readdirSync(srcDir);
    files.forEach(file => {
      const fullPath = path.join(srcDir, file);
      const destPath = path.join(destDir, file);

      if (fs.statSync(fullPath).isDirectory()) {
        if (!fs.existsSync(destPath)) {
          fs.mkdirSync(destPath, { recursive: true });
        }
        copyFiles(fullPath, destPath);
      } else if (file.endsWith('.js')) {
        fs.copyFileSync(fullPath, destPath);
      }
    });
  }

  copyFiles('./assets/js', distJsDir);
  done();
}

function minifyJS(done) {
  const distJsDir = './assets/dist/js';
  if (!fs.existsSync(distJsDir)) {
    fs.mkdirSync(distJsDir, { recursive: true });
  }

  function minifyJsFiles(srcDir, destDir) {
    if (!fs.existsSync(srcDir)) return Promise.resolve();

    const files = fs.readdirSync(srcDir);
    const promises = [];

    files.forEach(file => {
      const fullPath = path.join(srcDir, file);
      const destPath = path.join(destDir, file);

      if (fs.statSync(fullPath).isDirectory()) {
        if (!fs.existsSync(destPath)) {
          fs.mkdirSync(destPath, { recursive: true });
        }
        promises.push(minifyJsFiles(fullPath, destPath));
      } else if (file.endsWith('.js')) {
        const promise = new Promise(resolve => {
          const cmd = `npx terser "${fullPath}" -o "${destPath}" -c -m`;
          exec(cmd, (err, stdout, stderr) => {
            if (err) {
              fs.copyFileSync(fullPath, destPath);
            }
            resolve();
          });
        });
        promises.push(promise);
      }
    });

    return Promise.all(promises);
  }

  minifyJsFiles('./assets/js', distJsDir)
    .then(() => done())
    .catch(err => done(err));
}

function copyLibs(done) {
  exec('node libs-build.js', (err, stdout, stderr) => {
    if (err) return done(err);
    done();
  });
}

function minifyLibs(done) {
  exec('node libs-build.js', (err, stdout, stderr) => {
    if (err) return done(err);

    const distLibsDir = './assets/dist/libs';
    if (fs.existsSync(distLibsDir)) {
      minifyJsFilesInDir(distLibsDir)
        .then(() => done())
        .catch(err => done(err));
    } else {
      done();
    }
  });
}

function minifyJsFilesInDir(dir) {
  const files = fs.readdirSync(dir);
  const promises = [];

  files.forEach(file => {
    const fullPath = path.join(dir, file);

    if (fs.statSync(fullPath).isDirectory()) {
      promises.push(minifyJsFilesInDir(fullPath));
    } else if (file.endsWith('.js') && !file.endsWith('.min.js')) {
      const promise = new Promise(resolve => {
        const cmd = `npx terser "${fullPath}" -o "${fullPath}" -c -m`;
        exec(cmd, (err, stdout, stderr) => {
          resolve();
        });
      });
      promises.push(promise);
    }
  });

  return Promise.all(promises);
}

const buildAssets = gulp.parallel(buildCss, copyJS, copyLibs);
const buildProd = gulp.series(minifyCSS, gulp.parallel(minifyJS, minifyLibs));

exports.buildCss = buildCss;
exports.minifyCSS = minifyCSS;
exports.watchCSS = watchCSS;
exports.copyJS = copyJS;
exports.minifyJS = minifyJS;
exports.copyLibs = copyLibs;
exports.minifyLibs = minifyLibs;
exports.buildAssets = buildAssets;
exports.buildProd = buildProd;
exports.default = buildAssets;
