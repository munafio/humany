# Security Policy

## Supported versions

| Version | Supported |
| ------- | --------- |
| 0.1.x   | Yes       |

Older releases are not monitored. Upgrade to the latest patch when possible.

## Reporting a vulnerability

**Please do not open a public GitHub issue for security problems.**

Report privately using [GitHub Security Advisories](https://github.com/munafio/humany/security/advisories/new) for [munafio/humany](https://github.com/munafio/humany).

Include:

- Description of the issue and impact
- Steps to reproduce
- Affected version(s)
- Suggested fix (optional)

We aim to acknowledge reports within **72 hours** and to provide a fix or mitigation plan within **14 days** for confirmed issues.

## Scope

Humany formats values for display. It does not execute untrusted code, parse network input, or handle secrets. Typical risks are limited to:

- Panics or incorrect output on extreme inputs
- Dependency vulnerabilities (`libm`, optional `chrono` / `time`)

Reports outside this scope may still be accepted if they affect downstream users.

## Safe disclosure

We appreciate coordinated disclosure. We will credit reporters in release notes when they agree.