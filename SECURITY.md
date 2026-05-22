# Security Policy

## Supported Versions

Pyoway is a personal project under active development. Only the latest release and the `master` branch receive security updates.

| Version | Supported |
|---|---|
| latest release | ✅ |
| `master` | ✅ |
| older releases | ❌ |

## Dependency Management

Pyoway uses [Dependabot](.github/dependabot.yml) to automatically monitor and update dependencies:

- **Rust crate dependencies** are checked weekly (Mondays) for available updates
- **GitHub Actions** are checked weekly on the same schedule
- Minor and patch updates are grouped into single PRs to reduce noise

Security-related advisories from GitHub's Advisory Database are handled outside of the scheduled Dependabot cycle and will trigger alerts as soon as they are published.

## Reporting a Vulnerability

If you discover a security vulnerability in Pyoway, please report it through one of the following channels:

### 1. GitHub Private Vulnerability Report (preferred)

For critical vulnerabilities, use GitHub's [Private Vulnerability Reporting](https://docs.github.com/en/code-security/security-advisories/guidance-on-reporting-and-writing-information-about-vulnerabilities/privately-reporting-a-security-vulnerability) feature on the [Pyoway repository](https://github.com/pyoclaw/pyoway/security/advisories/new).

### 2. Email

Send details to the repository maintainer via the email address associated with the [GitHub profile](https://github.com/pyoclaw).

### What to include

To help us respond quickly, please include:

- A clear description of the vulnerability
- Steps to reproduce (if applicable)
- Affected dependency or component
- Potential impact
- Any suggested fixes (if known)

### Response timeline

| Timeframe | Action |
|---|---|
| Within 48 hours | Acknowledgment of receipt |
| Within 7 days | Initial assessment and triage |
| Within 30 days | Fix deployed or mitigation communicated |

## Scope

The following components are covered by this policy:

- **landing-server** — Axum HTTP server
- **landing-frontend** — Leptos WASM web frontend
- **docs/** — mdBook documentation site
- **CI/CD pipelines** — `.github/workflows/`
- **Build tooling** — Dockerfile, Justfile, Trunk config

## Out of Scope

The following are **not** covered by this policy:

- Third-party websites or services linked from the documentation or landing page
- Issues in upstream dependencies that are already fixed in newer versions (report those to the respective project)
- Theoretical vulnerabilities without a demonstrated exploit path

## Safe Harbor

We believe in responsible disclosure. If you report a vulnerability in good faith, we will not take legal action against you. We ask that you:

- Allow reasonable time for the fix to be deployed before public disclosure
- Do not access or modify user data without explicit permission
- Do not perform destructive testing (e.g., denial of service, data destruction)

## Acknowledgments

We appreciate the security community's help in keeping Pyoway safe. Contributors who report valid vulnerabilities will be acknowledged in the release notes (unless they prefer to remain anonymous).
