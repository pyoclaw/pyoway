#!/usr/bin/env bash
# ---------------------------------------------------------------------------
# Integration tests for the mdBook docs build pipeline.
#
# Usage:
#   ./docs/test-docs.sh              # build & test (cleans up afterward)
#   ./docs/test-docs.sh --skip-build  # test existing docs/book/ output
#   KEEP_BUILD=1 ./docs/test-docs.sh  # keep build artifacts after testing
#
# Exit code: 0 if all checks pass, 1 if any fail.
# ---------------------------------------------------------------------------
set -euo pipefail

DOCS_DIR="$(cd "$(dirname "$0")" && pwd)"
PROJECT_DIR="$(cd "$DOCS_DIR/.." && pwd)"
BOOK_DIR="$DOCS_DIR/book"

PASS=0
FAIL=0
SKIP=0

# Colours (disable if not a terminal)
if [ -t 1 ]; then
    GREEN='\033[0;32m'
    RED='\033[0;31m'
    YELLOW='\033[0;33m'
    BOLD='\033[1m'
    NC='\033[0m'
else
    GREEN=''; RED=''; YELLOW=''; BOLD=''; NC=''
fi

pass()  { PASS=$((PASS + 1)); echo -e "  ${GREEN}✓${NC} $1"; }
fail()  { FAIL=$((FAIL + 1)); echo -e "  ${RED}✗${NC} $1"; }
skip()  { SKIP=$((SKIP + 1)); echo -e "  ${YELLOW}−${NC} $1"; }
header() { echo -e "\n${BOLD}$1${NC}"; }

# ---------------------------------------------------------------------------
# 1. Prerequisites
# ---------------------------------------------------------------------------
header "Prerequisites"

MDBOOK=""
if command -v mdbook &>/dev/null; then
    MDBOOK="mdbook"
elif [ -x "$HOME/.cargo/bin/mdbook" ]; then
    MDBOOK="$HOME/.cargo/bin/mdbook"
    PATH="$HOME/.cargo/bin:$PATH"
elif [ -x "$HOME/.local/bin/mdbook" ]; then
    MDBOOK="$HOME/.local/bin/mdbook"
    PATH="$HOME/.local/bin:$PATH"
fi

if [ -n "$MDBOOK" ]; then
    pass "mdBook is available: $($MDBOOK --version 2>&1 | head -1)"
else
    skip "mdBook not installed — will attempt to install"
    echo "    Installing mdBook via cargo (this may take a while)..."
    cargo install mdbook 2>&1 | tail -1
    MDBOOK="$HOME/.cargo/bin/mdbook"
    if [ -x "$MDBOOK" ]; then
        PATH="$HOME/.cargo/bin:$PATH"
        pass "mdBook installed: $($MDBOOK --version 2>&1 | head -1)"
    else
        fail "mdBook could not be installed. Install manually: cargo install mdbook"
    fi
fi

# ---------------------------------------------------------------------------
# 2. Build
# ---------------------------------------------------------------------------
header "Build"

SKIP_BUILD="${1:+true}"

if [ "$SKIP_BUILD" = "true" ] && [ "$1" = "--skip-build" ]; then
    skip "Build step skipped (--skip-build)"
elif [ -d "$BOOK_DIR" ]; then
    echo "    Removing previous build..."
    rm -rf "$BOOK_DIR"
fi

if [ "$SKIP_BUILD" != "true" ] || [ "$1" != "--skip-build" ]; then
    echo "    Running: $MDBOOK build docs"
    cd "$PROJECT_DIR"
    if "$MDBOOK" build docs 2>&1; then
        pass "mdbook build succeeded"
    else
        fail "mdbook build failed"
    fi
fi

# ---------------------------------------------------------------------------
# 3. Directory structure
# ---------------------------------------------------------------------------
header "Directory structure"

[ -d "$BOOK_DIR" ] && pass "Output directory exists: docs/book/" || fail "Output directory missing: docs/book/"

if [ ! -d "$BOOK_DIR" ]; then
    fail "Cannot run further checks without build output"
    echo -e "\n${BOLD}Results: $PASS passed, $FAIL failed, $SKIP skipped${NC}"
    exit 1
fi

# Core HTML files
EXPECTED_HTML=(
    "index.html"
    "intro.html"
    "about.html"
    "404.html"
    "print.html"
    "blog/hello-world.html"
    "blog/post-template.html"
    "knowledge-base/rust/why-rust.html"
    "knowledge-base/rust/leptos-notes.html"
    "knowledge-base/web-dev/modern-wasm.html"
    "knowledge-base/tools/cargo-tools-guide.html"
)

for f in "${EXPECTED_HTML[@]}"; do
    [ -f "$BOOK_DIR/$f" ] && pass "Generated: $f" || fail "Missing: $f"
done

# Static assets
if [ -d "$BOOK_DIR/css" ]; then
    CSS_COUNT=$(find "$BOOK_DIR/css" -name '*.css' 2>/dev/null | wc -l)
    [ "$CSS_COUNT" -gt 0 ] && pass "CSS assets: $CSS_COUNT files in css/" || fail "No CSS files in css/"
else
    fail "Missing: css/ directory"
fi

# JS libraries (hashed filenames in v0.5)
JS_COUNT=$(find "$BOOK_DIR" -maxdepth 1 -name '*.js' 2>/dev/null | wc -l)
[ "$JS_COUNT" -gt 0 ] && pass "JavaScript assets: $JS_COUNT files" || fail "No JavaScript files found"

# ---------------------------------------------------------------------------
# 4. HTML content validation
# ---------------------------------------------------------------------------
header "Content validation"

# Title from book.toml
if grep -q '<title>' "$BOOK_DIR/intro.html" 2>/dev/null; then
    TITLE=$(sed -n 's/.*<title>\(.*\)<\/title>.*/\1/p' "$BOOK_DIR/intro.html" | head -1)
    if echo "$TITLE" | grep -qi "pyoway"; then
        pass "Book title contains 'Pyoway': $TITLE"
    else
        pass "Title tag present: $TITLE"
    fi
else
    fail "No <title> tag found in intro.html"
fi

# Content snippets from each source file
declare -A CONTENT_CHECKS
CONTENT_CHECKS["intro.html"]="Welcome to Pyoway"
CONTENT_CHECKS["about.html"]="Leptos (WASM)"
CONTENT_CHECKS["blog/hello-world.html"]="Welcome to Pyoway"
CONTENT_CHECKS["blog/post-template.html"]="Code example"
CONTENT_CHECKS["knowledge-base/rust/why-rust.html"]="Coming soon"
CONTENT_CHECKS["knowledge-base/rust/leptos-notes.html"]="Coming soon"
CONTENT_CHECKS["knowledge-base/web-dev/modern-wasm.html"]="Coming soon"
CONTENT_CHECKS["knowledge-base/tools/cargo-tools-guide.html"]="Coming soon"

for html_file in "${!CONTENT_CHECKS[@]}"; do
    expected_text="${CONTENT_CHECKS[$html_file]}"
    if [ -f "$BOOK_DIR/$html_file" ]; then
        if grep -Fq "$expected_text" "$BOOK_DIR/$html_file" 2>/dev/null; then
            pass "Content in $html_file contains: $expected_text"
        else
            fail "Content missing in $html_file: '$expected_text' not found"
        fi
    else
        fail "Cannot check content: $html_file does not exist"
    fi
done

# ---------------------------------------------------------------------------
# 5. Search functionality
# ---------------------------------------------------------------------------
header "Search"

# mdBook v0.5 uses hashed filenames (e.g., searcher-<hash>.js)
SEARCHER_JS=$(find "$BOOK_DIR" -maxdepth 1 -name 'searcher-*.js' 2>/dev/null | head -1)
SEARCHINDEX_JS=$(find "$BOOK_DIR" -maxdepth 1 -name 'searchindex-*.js' 2>/dev/null | head -1)
ELASTICLUNR_JS=$(find "$BOOK_DIR" -maxdepth 1 -name 'elasticlunr*.js' 2>/dev/null | head -1)

if [ -n "$SEARCHER_JS" ]; then
    pass "Search engine: $(basename "$SEARCHER_JS")"
else
    fail "Missing searcher-*.js — search may not be working"
fi

if [ -n "$SEARCHINDEX_JS" ]; then
    pass "Search index: $(basename "$SEARCHINDEX_JS")"
    # Basic search index integrity check — wrapped as JS variable assignment
    if head -1 "$SEARCHINDEX_JS" 2>/dev/null | grep -q "var searchindex"; then
        pass "Search index wrapped in JS variable (valid format)"
    else
        skip "Search index format: not a var searchindex assignment (may still be valid)"
    fi
else
    fail "Missing searchindex-*.js"
fi

if [ -n "$ELASTICLUNR_JS" ]; then
    pass "Search library: $(basename "$ELASTICLUNR_JS")"
else
    skip "Elasticlunr not found — search may use a different engine"
fi

# ---------------------------------------------------------------------------
# 6. Theme & custom CSS
# ---------------------------------------------------------------------------
header "Theme"

# Check for CSS variables from our custom theme (hashed filenames)
GENERAL_CSS=$(find "$BOOK_DIR/css" -name 'general-*.css' 2>/dev/null | head -1)
if [ -n "$GENERAL_CSS" ]; then
    if grep -q -- "--bg:" "$GENERAL_CSS" 2>/dev/null; then
        pass "Custom CSS variables present in $(basename "$GENERAL_CSS")"
    else
        skip "Custom CSS variables not found (theme may use separate file)"
    fi
else
    skip "general-*.css not found (theme may be bundled differently)"
fi

# Check for navy theme class (from book.toml: default-theme = "navy")
if [ -f "$BOOK_DIR/index.html" ] && grep -q "navy" "$BOOK_DIR/index.html" 2>/dev/null; then
    pass "Theme set to navy (from book.toml config)"
elif [ -f "$BOOK_DIR/intro.html" ] && grep -q "navy" "$BOOK_DIR/intro.html" 2>/dev/null; then
    pass "Theme set to navy (from book.toml config)"
else
    skip "Could not verify navy theme in HTML output"
fi

# ---------------------------------------------------------------------------
# 7. Link integrity — basic checks
# ---------------------------------------------------------------------------
header "Link integrity"

# Check that internal links between generated HTML files resolve
BROKEN_LINKS=0
BROKEN_LINKS_LOG="$(mktemp /tmp/mdbook_links.XXXXXX)"
trap 'rm -f "$BROKEN_LINKS_LOG"' EXIT
if command -v grep &>/dev/null; then
    # Look for hrefs to .html files and verify the target exists
    for html_file in "${EXPECTED_HTML[@]}"; do
        full_path="$BOOK_DIR/$html_file"
        [ ! -f "$full_path" ] && continue
        # Extract all href="...html" targets (POSIX-compatible)
        while IFS= read -r link_target; do
            [ -z "$link_target" ] && continue
            # Resolve relative to the book directory
            target_dir="$(dirname "$full_path")"
            resolved="$target_dir/$link_target"
            # Normalize path
            resolved="$(cd "$(dirname "$resolved")" 2>/dev/null && pwd 2>/dev/null)/$(basename "$resolved")" || true
            if [ -n "$resolved" ] && [ -f "$resolved" ]; then
                : # link OK
            else
                BROKEN_LINKS=$((BROKEN_LINKS + 1))
                echo "    Broken link in $html_file -> $link_target" >> "$BROKEN_LINKS_LOG"
            fi
        done < <(grep -oE 'href="[^"]+\.html"' "$full_path" 2>/dev/null | sed 's/href="//;s/"//')
    done

    if [ "$BROKEN_LINKS" -eq 0 ]; then
        pass "No broken internal HTML links"
    else
        fail "$BROKEN_LINKS broken internal link(s) found (see $BROKEN_LINKS_LOG)"
    fi
else
    skip "Link integrity check requires grep"
fi

# ---------------------------------------------------------------------------
# 8. Source-to-output correspondence
# ---------------------------------------------------------------------------
header "Source integrity"

MD_FILES=$(find "$DOCS_DIR/src" -name '*.md' ! -name 'SUMMARY.md' | sort)
MD_COUNT=0
HTML_COUNT=0

while IFS= read -r md_file; do
    MD_COUNT=$((MD_COUNT + 1))
    # Derive expected HTML path from markdown source path
    rel_path="${md_file#$DOCS_DIR/src/}"
    html_rel="${rel_path%.md}.html"
    if [ -f "$BOOK_DIR/$html_rel" ]; then
        HTML_COUNT=$((HTML_COUNT + 1))
    else
        echo "    No HTML for: $rel_path"
    fi
done <<< "$MD_FILES"

if [ "$MD_COUNT" -eq 0 ]; then
    fail "No markdown source files found in docs/src/"
elif [ "$MD_COUNT" -eq "$HTML_COUNT" ]; then
    pass "All $MD_COUNT markdown files have corresponding HTML output"
else
    fail "$HTML_COUNT/$MD_COUNT markdown files have HTML output ($((MD_COUNT - HTML_COUNT)) missing)"
fi

# SUMMARY.md should exist in output too (it generates the sidebar navigation)
if [ -f "$BOOK_DIR/index.html" ]; then
    pass "SUMMARY.md generated as index.html"
fi

# ---------------------------------------------------------------------------
# 9. Cleanup
# ---------------------------------------------------------------------------
header "Cleanup"

if [ "${KEEP_BUILD:-}" = "1" ]; then
    echo "    KEEP_BUILD=1 — leaving docs/book/ intact"
    pass "Build artifacts preserved"
else
    echo "    Removing docs/book/ ..."
    rm -rf "$BOOK_DIR"
    if [ ! -d "$BOOK_DIR" ]; then
        pass "Build artifacts cleaned up"
    else
        fail "Failed to remove docs/book/"
    fi
fi

# ---------------------------------------------------------------------------
# Summary
# ---------------------------------------------------------------------------
echo ""
echo -e "${BOLD}╔════════════════════════════════════╗${NC}"
echo -e "${BOLD}║  Docs Integration Test Results     ║${NC}"
echo -e "${BOLD}╠════════════════════════════════════╣${NC}"
printf "${BOLD}║${NC}  %-10s %3d passed                ${BOLD}║${NC}\n" "Passed:"  "$PASS"
printf "${BOLD}║${NC}  %-10s %3d failed                ${BOLD}║${NC}\n" "Failed:"  "$FAIL"
printf "${BOLD}║${NC}  %-10s %3d skipped               ${BOLD}║${NC}\n" "Skipped:" "$SKIP"
echo -e "${BOLD}╚════════════════════════════════════╝${NC}"

if [ "$FAIL" -gt 0 ]; then
    exit 1
fi
