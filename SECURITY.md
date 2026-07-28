# Security Policy

## Supported Versions

| Version | Supported |
| ------- | --------- |
| 1.0.x   | ✅ Active |
| < 1.0   | ❌        |

## Reporting a Vulnerability

If you discover a security vulnerability in MAR 1.0, please report it
privately **before** disclosing it publicly. **Do not open a public
GitHub issue for security vulnerabilities.**

### How to report

Open a draft [GitHub Security Advisory](https://github.com/your-org/MAR-1.0/security/advisories)
or email maintainers directly.

Please include:
- A description of the vulnerability
- Steps to reproduce
- Potential impact
- Suggested fix (if any)

We will acknowledge receipt within 48 hours.

## Disclosure Policy

We follow coordinated disclosure:
1. Report received → acknowledged within 48h
2. Investigation and fix → typically 7–14 days
3. Release with fix and advisory → public disclosure

## Security-Related Configuration

**Do not commit secrets to the repository.**
- Always use environment variables for secrets
- The .env file is gitignored — use .env.example as a template
- Rotate any secrets accidentally committed
