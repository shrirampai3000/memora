// memora — AI that knows everything you've seen, said, or heard
// https://github.com/shrirampai3000/memora

# Memora Security Policy

## Data Security & Encryption

- **Encryption at Rest**: Sensitive integration tokens and vault secrets are stored encrypted using keys managed by your operating system's native keychain (Windows Credential Manager / macOS Keychain / Linux Secret Service).
- **Local API Authentication**: The local REST API server binds to `127.0.0.1` by default and enforces token authentication for network calls.

## Reporting Security Issues

If you discover a potential security vulnerability in Memora, please report it privately by opening an issue on GitHub marked with the security tag or contacting the maintainer directly.
