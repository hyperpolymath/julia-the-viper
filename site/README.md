# Julia the Viper - Static Site

This directory contains the static site for Julia the Viper, built with [ddraig-ssg](https://github.com/hyperpolymath/ddraig-ssg).

## Structure

```
site/
├── src/
│   └── Ddraig.idr      # ddraig-ssg source (Idris 2)
├── content/            # Markdown content files
│   ├── index.md
│   └── quick-start.md
├── templates/          # Custom templates (optional)
├── static/             # Static assets (CSS, images)
├── manifest.json       # Site configuration
└── README.md
```

## Requirements

- **Idris 2** (0.6.0 or later)
- Or use the Docker container: `ghcr.io/idris-community/idris2:latest`

## Building

### Local Build (requires Idris 2)

```bash
cd site/src
idris2 Ddraig.idr -o ddraig
./build/exec/ddraig build
```

### Docker Build

```bash
docker run --rm -v $(pwd):/site ghcr.io/idris-community/idris2:latest \
  sh -c "cd /site/src && idris2 Ddraig.idr -o ddraig"
```

### GitHub Actions

The site is automatically built via GitHub Actions on push to main.
See `.github/workflows/ddraig-build.yml`.

## Testing

```bash
./build/exec/ddraig test-markdown
./build/exec/ddraig test-frontmatter
./build/exec/ddraig test-full
```

## About ddraig-ssg

ddraig-ssg is a dependently-typed static site generator written in Idris 2.
It provides:

- **Type-safe templates** - Templates carry their structure in their types
- **Totality checking** - Prove templates handle all inputs
- **Schema enforcement** - Frontmatter schemas are types
- **Compile-time link verification** - Internal links verified at build time

Part of the [poly-ssg](https://github.com/hyperpolymath/poly-ssg) family.
