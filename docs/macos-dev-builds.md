# macOS dev builds

<!-- doc-covers: none -->

Dev builds are signed with a developer certificate so macOS TCC keeps recognizing
the app across rebuilds and permissions persist.

Config: `apps/MEMORA-app-tauri/src-tauri/tauri.conf.json` â†’
`bundle.macOS.signingIdentity`.

Contributors without the certificate will hit permission prompts on every
rebuild. Onboarding shows a "continue anyway" button after 5s for that case.
