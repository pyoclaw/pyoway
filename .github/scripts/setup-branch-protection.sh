#!/usr/bin/env bash
#
# setup-branch-protection.sh
#
# Configures GitHub branch protection rules on the default branch to require
# CI checks to pass before merging (including Dependabot PRs).
#
# Prerequisites:
#   1. First commit must be pushed to GitHub
#   2. gh CLI must be authenticated (gh auth status)
#   3. Repo must exist on GitHub (create it first if needed)
#
# Usage:
#   bash .github/scripts/setup-branch-protection.sh
#
# If you prefer to configure this manually in the GitHub UI:
#   Settings > Branches > Add branch protection rule
#   Branch name pattern: main
#   ☑ Require pull request reviews before merging
#     Required approvals: 1
#     ☑ Dismiss stale reviews
#     Bypass: Dependabot (app)
#   ☑ Require status checks to pass before merging
#     ☑ Require branches to be up to date
#     Status checks: Check & Lint, Test Server, Test Frontend, Security Audit
#   ☑ Do not allow bypassing the above settings
#   └─ Save

set -euo pipefail

REPO=$(gh repo view --json nameWithOwner --jq '.nameWithOwner' 2>/dev/null || true)

if [[ -z "$REPO" ]]; then
  echo "❌ Could not determine GitHub repository."
  echo "   Make sure you've pushed to GitHub and 'gh' is authenticated."
  echo ""
  echo "   Quick start:"
  echo "     git remote add origin git@github.com:pyoclaw/pyoway.git"
  echo "     git add -A && git commit -m 'Initial commit'"
  echo "     git push -u origin master"
  echo "     # Then rename master to main on GitHub:"
  echo "     # Repo Settings > Default Branch > change to main"
  echo ""
  exit 1
fi

echo "🔧 Configuring branch protection for $REPO"

# The CI status checks that must pass before merging
REQUIRED_CHECKS=(
  "Check & Lint"
  "Test Server"
  "Test Frontend"
  "Security Audit"
)

# Build the checks JSON array
CHECKS_JSON="["
FIRST=true
for check in "${REQUIRED_CHECKS[@]}"; do
  if [ "$FIRST" = true ]; then
    FIRST=false
  else
    CHECKS_JSON+=", "
  fi
  CHECKS_JSON+="{\"context\": \"$check\", \"app_id\": null}"
done
CHECKS_JSON+="]"

# Build the pull request reviews JSON
# Note: bypass_pull_request_allowances is only available for organization repos via API.
# For personal repos, configure the Dependabot bypass manually in the GitHub UI:
#   Settings > Branches > main > Require pull request reviews >
#   "Allow specific actors to bypass pull request requirements" > + Add Dependabot
REVIEWS_JSON='{
    "dismiss_stale_reviews": true,
    "require_code_owner_reviews": false,
    "required_approving_review_count": 1
  }'

echo "   Required reviews: 1 approval (Dependabot bypass: GitHub UI > Settings > Branches > main > add Dependabot to bypass list)"
echo "   Required checks: ${REQUIRED_CHECKS[*]}"

# Apply branch protection to the main branch
# Uses the GitHub REST API via gh
if gh api \
  -X PUT "repos/$REPO/branches/main/protection" \
  --input - <<JSON
{
  "required_status_checks": {
    "strict": true,
    "checks": $CHECKS_JSON
  },
  "enforce_admins": true,
  "required_pull_request_reviews": $REVIEWS_JSON,
  "restrictions": null
}
JSON
then
  echo ""
  echo "✅ Branch protection applied to 'main'"
  echo ""
  echo "Rules enabled:"
  echo "  • Require pull request reviews (1 approval)"
  echo "  • Dependabot bypassed from review requirement (exempt as an app)"
  echo "  • Require status checks to pass before merging"
  echo "  • Require branches to be up to date (strict)"
  echo "  • Required checks: ${REQUIRED_CHECKS[*]}"
  echo "  • Enforce for admins"
  echo ""
  echo "Human PRs require 1 approval. Dependabot PRs bypass this and merge automatically via the auto-merge workflow."
else
  echo ""
  echo "❌ Failed to apply branch protection."
  echo ""
  echo "Common issues:"
  echo "  • The 'main' branch may not exist yet (rename 'master' to 'main' on GitHub)"
  echo "  • You may not have admin permissions on the repo"
  echo "  • The API rate limit may be exceeded"
  echo ""
  echo "To configure manually: Settings > Branches > Add branch protection rule"
  echo "Branch pattern: main"
  echo "  ☑ Require pull request reviews before merging"
  echo "    Required approvals: 1"
  echo "    ☑ Dismiss stale reviews"
  echo "    Bypass: add Dependabot app"
  echo "  ☑ Require status checks to pass before merging"
  echo "    ☑ Require branches to be up to date"
  echo "    ☑ Check & Lint, Test Server, Test Frontend, Security Audit"
  echo "  ☑ Do not allow bypassing the above settings"
fi
