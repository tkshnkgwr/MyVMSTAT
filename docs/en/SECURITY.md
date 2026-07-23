**English** | [日本語版](../ja/SECURITY.md)

# Security Policy: SECURITY

This document defines the security standards, supported versions, and reporting procedures for security vulnerabilities in **MyVMSTAT**.

---

## 1. Supported Versions

Security updates are actively applied to the following release branches:

| Version | Support Status |
| :--- | :--- |
| **v1.3.x** | 🟢 Actively Supported (Latest release) |
| **< v1.3.0** | 🔴 No Longer Supported |

We release security patches targeting the latest active minor/patch releases. Users on older releases are strongly encouraged to upgrade to the latest version immediately to resolve potential security risks.

---

## 2. Reporting a Vulnerability

If you discover a security vulnerability, please do not post it publicly on GitHub Issues or other open forums. Report it privately using the steps below:

1. **How to Report**:
   Send a private message or email directly to the repository maintainer.
2. **Information to Include**:
   - Detailed description of the vulnerability and its potential impact.
   - Step-by-step reproduction steps, including any proof-of-concept (PoC) code or command-line arguments.
   - Expected risk scenarios (e.g. panic crashes, resource leaks).
3. **Disclosure Window**:
   Once reported, the maintainers will verify the vulnerability and build a patch. We kindly ask you to refrain from disclosing details publicly until the security fix is merged and released.
