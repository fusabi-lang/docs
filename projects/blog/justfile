# Fusabi Blog Publishing
# Automated blog post publishing to fusabi-lang/docs

# Show available commands
default:
    @just --list

# ============================================================================
# Publishing
# ============================================================================

# Publish today's daily update to fusabi-lang/docs blog
publish-daily:
    nu scripts/publish.nu daily

# Publish this week's report to fusabi-lang/docs blog
publish-weekly:
    nu scripts/publish.nu weekly

# Publish this month's review to fusabi-lang/docs blog
publish-monthly:
    nu scripts/publish.nu monthly

# Publish all unpublished posts to fusabi-lang/docs blog
publish-all:
    nu scripts/publish.nu all

# Dry run - show what would be published without actually publishing
publish-dry-run MODE:
    nu scripts/publish.nu {{MODE}} --dry-run

# ============================================================================
# Content Management
# ============================================================================

# Create a new daily post for today
new-daily:
    @echo "Creating daily post structure..."
    @mkdir -p output/daily
    @echo "---\ntitle: \"Fusabi Daily Update - $(date +%Y-%m-%d)\"\n---\n\n# Daily Update\n\nWrite your content here..." > output/daily/$(date +%Y-%m-%d).md
    @echo "✅ Created output/daily/$(date +%Y-%m-%d).md"

# Create a new weekly post
new-weekly:
    @echo "Creating weekly post structure..."
    @mkdir -p output/weekly
    @echo "---\ntitle: \"This Week in Fusabi - $(date +%Y-W%V)\"\n---\n\n# Weekly Report\n\nWrite your content here..." > output/weekly/$(date +%Y-W%V).md
    @echo "✅ Created output/weekly/$(date +%Y-W%V).md"

# Create a new monthly post
new-monthly:
    @echo "Creating monthly post structure..."
    @mkdir -p output/monthly
    @echo "---\ntitle: \"Fusabi Monthly Review - $(date +%B\ %Y)\"\n---\n\n# Monthly Review\n\nWrite your content here..." > output/monthly/$(date +%Y-%m).md
    @echo "✅ Created output/monthly/$(date +%Y-%m).md"

# ============================================================================
# Utilities
# ============================================================================

# Clean temporary docs directory
clean:
    rm -rf .fusabi-docs-temp

# Check for unpublished posts
status:
    @echo "📊 Blog Post Status"
    @echo "==================="
    @echo ""
    @echo "Daily posts:"
    @ls -1 output/daily/*.md 2>/dev/null | wc -l || echo "0"
    @echo ""
    @echo "Weekly posts:"
    @ls -1 output/weekly/*.md 2>/dev/null | wc -l || echo "0"
    @echo ""
    @echo "Monthly posts:"
    @ls -1 output/monthly/*.md 2>/dev/null | wc -l || echo "0"

# Install GitHub CLI (required for publishing)
install-gh:
    @echo "Installing GitHub CLI..."
    @echo "See: https://cli.github.com/manual/installation"
    @echo "Then run: gh auth login"

# Setup blog directory structure
setup:
    mkdir -p output/daily
    mkdir -p output/weekly
    mkdir -p output/monthly
    @echo "✅ Blog directory structure created"

# ============================================================================
# Help
# ============================================================================

# Show help for a specific command
help COMMAND:
    just --show {{COMMAND}}

# Show version information
version:
    @echo "Fusabi Blog Publishing"
    @nu --version
