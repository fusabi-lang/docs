# Fusabi Lang Documentation Hub

Central documentation hub for the fusabi-lang organization. This repository automatically aggregates and publishes documentation from all public fusabi-lang repositories.

## Overview

This documentation site is built with:
- **Quartz v4**: Static site generator optimized for Obsidian vaults
- **Nushell Scripts**: Automated repository discovery and synchronization
- **GitHub Actions**: Daily automated builds and deployments
- **Git Submodules**: Links to documentation from each repository

## Quick Start

### Prerequisites

- Node.js v22+ and npm v10.9.2+
- Nushell (latest stable version)
- Git with submodule support
- GitHub CLI (gh) for API access
- Obsidian (optional, for local editing)

### Initial Setup

```bash
# Clone repository
git clone https://github.com/fusabi-lang/docs.git
cd docs

# Install Quartz
npx quartz create

# Install dependencies
npm install

# Discover and sync repositories
npm run discover
npm run sync

# Build and preview
npm run dev
```

## Available Scripts

- `npm run discover` - Discover fusabi-lang repositories with /docs directories
- `npm run sync` - Synchronize git submodules with discovered repositories
- `npm run update` - Update documentation from all submodules
- `npm run pipeline` - Run full build pipeline (discover, sync, update, build)
- `npm run dev` - Start local development server
- `npm run build` - Build static site

## Repository Structure

```
docs/
├── index.md                    # Homepage
├── content/                    # Main documentation content
│   ├── projects/              # Aggregated from submodules
│   │   ├── fusabi/           # Submodule: fusabi-lang/fusabi/docs
│   │   └── ...
│   └── guides/               # Local documentation
├── scripts/                   # Nushell automation scripts
│   ├── discover-repos.nu     # GitHub API repo discovery
│   ├── sync-submodules.nu    # Submodule management
│   ├── update-docs.nu        # Documentation updates
│   └── build-site.nu         # Build orchestration
├── config/                    # Configuration files
│   └── ignorelist.json       # Repositories to exclude
├── .github/
│   └── workflows/
│       └── sync-and-deploy.yml  # Automated sync & deploy
├── quartz.config.ts          # Quartz configuration
├── package.json              # Node.js dependencies
└── CLAUDE.md                 # Detailed technical documentation
```

## Configuration

### Ignorelist

Edit `config/ignorelist.json` to exclude specific repositories or patterns:

```json
{
  "repositories": ["docs", "private-repo"],
  "patterns": ["fork-*", "archive-*", "test-*"],
  "exclude_forks": true,
  "exclude_archived": true,
  "exclude_private": true,
  "require_docs_directory": true
}
```

### Quartz Configuration

Edit `quartz.config.ts` to customize:
- Site title and metadata
- Theme colors and typography
- Plugins and transformers
- Analytics and SEO settings

## Automated Workflow

The GitHub Actions workflow runs:
- **Daily at 2 AM UTC**: Automatic sync and deployment
- **On push to main**: When configuration or scripts change
- **Manual dispatch**: Via GitHub Actions UI
- **Repository dispatch**: When triggered by other repositories

## Contributing Documentation

To add or update documentation:

1. Navigate to the relevant project repository
2. Edit markdown files in the `/docs` directory
3. Submit a pull request
4. Changes will be automatically synchronized to the docs site

## Maintenance

### Daily Tasks (Automated)
- Sync latest documentation from all repositories
- Update version information and metadata
- Build and deploy static site to GitHub Pages

### Manual Tasks
- Review new repositories for inclusion
- Update ignorelist configuration
- Customize Quartz theme and plugins

## Documentation

For detailed technical information, see:
- [CLAUDE.md](./CLAUDE.md) - Complete technical documentation
- [Quartz Documentation](https://quartz.jzhao.xyz/)
- [Nushell Book](https://www.nushell.sh/book/)

## Current Repositories

The following repositories are currently tracked:

- **fusabi** - A high-performance, type-safe F# scripting engine for embedded Rust applications (v0.3.0-alpha)

## License

This documentation hub aggregates content from multiple repositories. Each project retains its original license. See individual project directories for specific licensing information.

---

**Note**: This repository is public and only aggregates from public repositories within the fusabi-lang organization.