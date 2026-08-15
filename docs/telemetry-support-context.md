# Telemetry Support Context

<!-- doc-covers: crates/MEMORA-telemetry-wire -->
<!-- doc-verified: 86f43d4c8 -->
> **Current.** Last verified against 86f43d4c8 (2026-06-05).

Customers who embed the MEMORA CLI can attach a stable, non-PII support
identity to MEMORA's existing telemetry pipeline. This lets MEMORA support
filter Sentry errors and PostHog events by customer, deployment, or host app.

```bash
export MEMORA_SUPPORT_ID="spcust_acme_123"
export MEMORA_CUSTOMER_ID="acme"
export MEMORA_DEPLOYMENT_ID="prod-laptop-fleet-01"
export MEMORA_EMBEDDER="acme-agent"
export MEMORA_EMBEDDER_VERSION="2026.6.4"

npx MEMORA record
```

## Environment Variables

| Variable | Purpose |
| --- | --- |
| `MEMORA_SUPPORT_ID` | Stable support ID for this customer/user/deployment. Used as the CLI PostHog `distinct_id` when `MEMORA_ANALYTICS_ID` is not set, and attached to Sentry/PostHog as `MEMORA_support_id`. |
| `MEMORA_CUSTOMER_ID` | Customer or organization key, attached as `MEMORA_customer_id`. |
| `MEMORA_DEPLOYMENT_ID` | Fleet, endpoint group, device, or environment key, attached as `MEMORA_deployment_id`. |
| `MEMORA_EMBEDDER` | Name of the app embedding or launching MEMORA, attached as `MEMORA_embedder`. |
| `MEMORA_EMBEDDER_VERSION` | Version of the embedding app, attached as `MEMORA_embedder_version`. |

Aliases are also accepted for easier integration with existing deployments:

| Canonical variable | Accepted aliases |
| --- | --- |
| `MEMORA_SUPPORT_ID` | `MEMORA_TELEMETRY_ID` |
| `MEMORA_CUSTOMER_ID` | `MEMORA_ORG_ID`, `MEMORA_TELEMETRY_CUSTOMER_ID` |
| `MEMORA_DEPLOYMENT_ID` | `MEMORA_TELEMETRY_DEPLOYMENT_ID` |
| `MEMORA_EMBEDDER` | `MEMORA_HOST_APP`, `MEMORA_TELEMETRY_HOST_APP` |
| `MEMORA_EMBEDDER_VERSION` | `MEMORA_HOST_VERSION`, `MEMORA_TELEMETRY_HOST_VERSION` |

`MEMORA_ANALYTICS_ID` remains the highest-priority explicit PostHog
`distinct_id`. Use it only when you intentionally want to control the exact
person identity. Otherwise prefer `MEMORA_SUPPORT_ID`.

## Privacy Boundary

These variables should contain opaque IDs, not emails or names. MEMORA
telemetry still follows the normal telemetry settings: `--disable-telemetry`
turns it off, and telemetry does not include screen content, audio, transcripts,
or file contents.

The JavaScript/Swift SDK does not send first-party MEMORA telemetry on its
own. If an SDK host wants MEMORA support correlation, it should set the same
environment variables when launching the MEMORA CLI/engine, and may also add
the same fields to the host application's own Sentry/PostHog reports.
