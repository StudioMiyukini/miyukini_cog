# TailwindCSS Production Build

This is the production-ready version of the TailwindCSS project containing generated HTML files, compiled assets, and TailwindCSS output.

## Quick Start

### Development

```bash
pnpm run dev
```

This will watch your CSS changes and compile TailwindCSS automatically.

### Production Build

```bash
pnpm run build:prod
```

This creates optimized and minified assets for production deployment.

## Project Structure

```
src/
├── html/
│   ├── ......
│   ├── ......
│   ├── ......
│   └── ......
├── assets/              # Build assets and outputs
│   ├── css/            # Source CSS files
│   │   └── main.css    # Main TailwindCSS source
│   ├── js/             # JavaScript files
│   ├── img/            # Images and graphics
│   ├── json/           # JSON data files
│   └── dist/           # Built assets (previously vendor)
│       ├── css/        # Compiled CSS
│       │   └── output.css  # Generated TailwindCSS output
│       ├── js/         # Built JavaScript
│       └── libs/       # External libraries
```

## Commands

### Development Commands

- `pnpm run dev` - Start TailwindCSS development mode with file watching and local server
- `pnpm run watch` - Start TailwindCSS in watch mode (CSS compilation only)
- `pnpm run serve` - Start a local HTTP server for preview

### Build Commands

- `pnpm run build` - Build all assets for development
- `pnpm run build:css` - Compile TailwindCSS without minification
- `pnpm run build:js` - Copy JavaScript files to dist directory
- `pnpm run build:libs` - Build external libraries from node_modules
- `pnpm run build:assets` - Build all assets for development
- `pnpm run build:prod` - Build all assets for production with minification

### Production Build Commands

### `pnpm run build:prod`

This is the main production build command that creates optimized, minified assets:

- **Minified CSS**: TailwindCSS compiled with `--minify` flag
- **Minified JavaScript**: All JS files minified using Terser
- **Minified Libraries**: Third-party dependencies are minified
- **Preserves Structure**: Maintains the same directory structure as development

**What gets minified:**

- `assets/js/**/*.js` → `assets/dist/js/**/*.js` (minified)
- `assets/css/main.css` → `assets/dist/css/output.css` (minified)
- All vendor libraries in `assets/dist/libs/` (minified)

**Example file size reduction:**

- CSS: ~25-50% smaller
- JavaScript: ~40-60% smaller
- Overall: Significant reduction in bundle size

### Development vs Production

| Command                 | CSS        | JavaScript | Libs     | Use Case    |
| ----------------------- | ---------- | ---------- | -------- | ----------- |
| `pnpm run build:assets` | Unminified | Copied     | Copied   | Development |
| `pnpm run build:prod`   | Minified   | Minified   | Minified | Production  |

### Utility Commands

- `pnpm run watch` - Watch CSS changes and recompile automatically

## Features

### 🎨 TailwindCSS

- **TailwindCSS 4.x**: Latest version with enhanced features
- **FlyonUI**: Extended UI components for TailwindCSS
- **Live compilation**: Automatic CSS generation during development
- **Optimized output**: Minified and purged CSS for production

### 📱 Responsive Design

- **Mobile-first**: Responsive layouts that work on all devices
- **Modern CSS**: Flexbox and CSS Grid for advanced layouts
- **Icon support**: Tabler icons via Iconify integration

### ⚡ Performance

- **Minification**: Optimized CSS and JavaScript for production
- **Asset optimization**: Efficient asset bundling and processing
- **Modern tooling**: Gulp-based build system for asset management

### 🔧 Development Tools

- **Live reload**: Automatic browser refresh during development
- **File watching**: Automatic rebuilds on file changes
- **HTTP server**: Built-in development server
- **Clean builds**: Automated cleanup of generated files
- **Production builds**: Complete minification and optimization pipeline
- **Asset path management**: Automatic path updates for production

## Getting Started

### Prerequisites

- Node.js 18.0.0 or higher
- pnpm or yarn package manager

### Installation

1. **Install dependencies:**

   ```bash
   pnpm install
   ```

2. **Start development:**

   ```bash
   pnpm run dev
   ```

   This will start TailwindCSS in watch mode and compile changes automatically.

3. **Preview your site:**

   ```bash
   pnpm run serve
   ```

   This will start a local server at `http://localhost:8080`

### Building for Production

1. **Create production build:**

   ```bash
   pnpm run build:prod
   ```

   This command will:

   - Generate all HTML layouts from the core directory
   - Minify CSS with TailwindCSS
   - Minify and copy JavaScript files
   - Copy libraries and other assets
   - Create a complete `dist/` directory ready for deployment

2. **Preview production build:**

   ```bash
   cd dist && npx http-server . -p 8080 -o
   ```

### Production Build Process

The production build system creates a complete `dist/` directory with:

- **Minified CSS**: TailwindCSS compiled with `--minify` flag
- **Minified JavaScript**: All JS files processed through Terser
- **Optimized Assets**: Images, fonts, and JSON files copied efficiently
- **Libraries**: Third-party dependencies organized and copied
- **HTML Files**: All generated templates with updated asset paths

## Configuration

### TailwindCSS Configuration

The main TailwindCSS configuration is defined in the source CSS file at `css/main.css`. This file uses TailwindCSS 4.x syntax with `@import` directives and can include custom CSS.

### Build Configuration

Build settings are managed through:

- `package.json` - NPM scripts and dependencies
- `gulpfile.js` - Gulp task definitions
- `build-config.js` - Build path configurations
- `libs-build.js` - Library building script

## Dependencies

### Production Dependencies

- `@tailwindcss/cli` - TailwindCSS command-line interface
- `flyonui` - Extended UI components for TailwindCSS
- `tailwindcss` - Core TailwindCSS framework
- `gulp` - Build system task runner
- Various UI libraries - DataTables, ApexCharts, jQuery, Flatpickr, etc.

### Development Dependencies

- `@iconify-json/tabler` - Tabler icon set
- `@iconify/tailwind4` - Iconify integration for TailwindCSS 4.x
- `fs-extra` - Enhanced file system utilities
- `http-server` - Static file server for development
- `terser` - JavaScript minification tool
- `prettier` - Code formatting tool
- `clean-css-cli` - CSS minification tool

## Troubleshooting

### Common Issues

**CSS not updating:**

- Make sure `pnpm run dev` is running
- Check that your changes are in the correct CSS source files
- Verify file paths in the TailwindCSS compilation command

**Build fails:**

- Ensure Node.js version is 18.0.0 or higher
- Try running `pnpm run clean` followed by `pnpm install`
- Check for any missing dependencies
