# Security Policy

## Supported Versions

This is an experimental playground repository. Security updates are provided on a best-effort basis.

| Version | Supported          |
| ------- | ------------------ |
| main    | ✅ Active development |
| Other branches | ❌ Experimental only |

## Reporting a Vulnerability

**Please report security vulnerabilities responsibly.**

### Where to Report

1. **GitHub Security Advisories** (Preferred)
   - Navigate to: https://github.com/Hyperpolymath/jtv-playground/security/advisories/new
   - Provide detailed information (see below)
   - Report will be private until fixed

2. **Email** (Alternative)
   - Contact maintainers via GitHub profile
   - Use PGP encryption if possible (key in .well-known/security.txt)

### What to Include

Please provide:

- **Description**: Clear explanation of the vulnerability
- **Impact**: Potential security impact and affected components
- **Reproduction**: Step-by-step instructions to reproduce
- **Affected Versions**: Which versions/branches are vulnerable
- **Suggested Fix**: If you have a patch or mitigation (optional)
- **Disclosure Timeline**: Your intended disclosure timeline

### What NOT to Do

- ❌ Do NOT create a public GitHub issue
- ❌ Do NOT discuss on social media or forums
- ❌ Do NOT exploit the vulnerability beyond proof-of-concept
- ❌ Do NOT share details with third parties before fix

## Response Timeline

We aim to respond within:

| Severity | Acknowledgment | Assessment | Fix Target |
|----------|---------------|------------|------------|
| **Critical** | 24 hours | 48 hours | 48-72 hours |
| **High** | 48 hours | 7 days | 1-2 weeks |
| **Medium** | 1 week | 2 weeks | 2-4 weeks |
| **Low** | 2 weeks | Best effort | Best effort |

### Severity Levels

**Critical** (CVSS 9.0-10.0):
- Remote code execution
- Privilege escalation to admin/root
- Data exfiltration of sensitive information
- Complete system compromise

**High** (CVSS 7.0-8.9):
- Authentication bypass
- SQL injection or command injection
- Cross-site scripting (XSS) with data access
- Denial of service affecting availability

**Medium** (CVSS 4.0-6.9):
- Information disclosure
- CSRF vulnerabilities
- Path traversal
- Weak cryptography

**Low** (CVSS 0.1-3.9):
- Non-security bugs with minor impact
- Configuration issues
- Documentation errors

## Security Best Practices

This project follows:

### 1. Zero Trust Architecture

- **Verify everything**: No implicit trust
- **Least privilege**: Minimal permissions by default
- **Assume breach**: Design for containment
- **Explicit security boundaries**: Clear perimeters

### 2. Offline-First Security

- **Minimal attack surface**: No unnecessary network calls
- **Air-gap capable**: Works without internet
- **No telemetry**: No data exfiltration
- **Local-first**: Data stays on user's machine

### 3. Type & Memory Safety

- **Rust**: Memory safety without garbage collection
- **Julia**: Type stability and bounds checking
- **ReScript**: Sound type system, no `any` types
- **No `unsafe` blocks**: Unless absolutely necessary and audited

### 4. Supply Chain Security

- **Minimal dependencies**: Reduce attack surface
- **Reproducible builds**: Nix flakes for determinism
- **Dependency pinning**: Exact versions, not ranges
- **Audit trail**: SBOM generation (future)

### 5. Secure Development Lifecycle

- **Pre-commit hooks**: Linting, formatting, security scans
- **CI/CD security**: Automated vulnerability scanning
- **Code review**: All changes reviewed (TPCF Perimeter 1)
- **Testing**: 100% code coverage target

## Out of Scope

The following are **out of scope** for security reports:

- Issues in third-party dependencies (report to upstream)
- Vulnerabilities in demo/example code clearly marked as insecure
- Social engineering or phishing attacks
- Physical security (theft, hardware tampering)
- Denial of service on public demo instances
- Issues requiring physical access to the system
- Vulnerabilities in unsupported versions/branches

## Vulnerability Disclosure Policy

### Our Commitments

1. **Acknowledgment**: Confirm receipt within 48 hours
2. **Communication**: Keep you updated on progress
3. **Credit**: Public acknowledgment (if desired)
4. **Coordinated Disclosure**: Agree on disclosure timeline
5. **Transparency**: Public post-mortem after fix

### Your Commitments

1. **Confidentiality**: Keep details private until fixed
2. **Good Faith**: Don't exploit beyond proof-of-concept
3. **Coordination**: Work with us on disclosure timeline
4. **Patience**: Allow reasonable time for fixes

### Disclosure Timeline

- **Critical**: 7-14 days after fix deployed
- **High**: 30 days after fix deployed
- **Medium/Low**: 90 days after fix deployed

We may request extension for complex issues.

## Security Hall of Fame

We gratefully acknowledge security researchers who responsibly disclose vulnerabilities:

<!-- No vulnerabilities disclosed yet -->

_Your name could be here! Report responsibly._

## Security Features

### Current Implementation

- ✅ **Type Safety**: Rust, Julia, ReScript enforce correctness
- ✅ **Memory Safety**: No buffer overflows, use-after-free
- ✅ **Offline-First**: Minimal network attack surface
- ✅ **Principle of Least Privilege**: Minimal permissions
- ✅ **Security Documentation**: This file, .well-known/security.txt
- ✅ **Dual License**: MIT + Palimpsest v0.8 for legal clarity

### Planned Enhancements

- 🔄 **Automated SAST**: Static analysis in CI/CD
- 🔄 **Dependency Scanning**: Automated CVE detection
- 🔄 **Fuzzing**: Automated fuzz testing
- 🔄 **Penetration Testing**: Regular security audits
- 🔄 **SBOM Generation**: Software Bill of Materials
- 🔄 **Signed Releases**: GPG signatures on releases

## Compliance & Standards

This project aims for:

- **OWASP Top 10**: Mitigate common web vulnerabilities
- **CWE Top 25**: Address most dangerous software weaknesses
- **NIST Cybersecurity Framework**: Identify, Protect, Detect, Respond, Recover
- **RFC 9116**: Security.txt standard compliance
- **RSR Framework**: Rhodium Standard Repository security requirements
- **TPCF**: Tri-Perimeter security model

## Security Contacts

- **Primary**: GitHub Security Advisories
- **Secondary**: Maintainer contact via GitHub profile
- **Public**: GitHub Issues (for non-security bugs only)

See `.well-known/security.txt` for canonical contact information (RFC 9116).

## Additional Resources

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [CWE Top 25](https://cwe.mitre.org/top25/)
- [NIST Cybersecurity Framework](https://www.nist.gov/cyberframework)
- [Responsible Disclosure Guidelines](https://cheatsheetseries.owasp.org/cheatsheets/Vulnerability_Disclosure_Cheat_Sheet.html)

---

**Last Updated**: 2025-11-22
**Version**: 1.0
**Maintainer**: JTV (Hyperpolymath)

Thank you for helping keep this project secure! 🔒
