---
title: Codex
description: Configure ChatGPT Codex authentication, models, reasoning, tools, images, transports, continuation, compaction, and native Responses passthrough.
---

Codex uses the ChatGPT subscription Responses endpoint at `https://chatgpt.com/backend-api/codex/responses`.

## Account and authentication

Sign in with a **ChatGPT Plus or Pro account**, not OpenAI API credentials.

```sh
claude-code-proxy codex auth login
# Headless device-code flow
claude-code-proxy codex auth device
claude-code-proxy codex auth status
```

The proxy owns its tokens and does not read native Codex CLI credentials. It refreshes expiring access tokens with a single-flight guard. See [Files and storage](/reference/files-and-storage/) for credential locations.

## Models and fast mode

Use `claude-code-proxy models` as the current catalog. Model access depends on your ChatGPT account. A model rejected by the subscription produces the upstream error verbatim.

Append `-fast` to any registered Codex model to request `service_tier: "priority"`. For example, `gpt-5.6-sol-fast` selects `gpt-5.6-sol` with fast service. `CCP_CODEX_SERVICE_TIER` or `codex.serviceTier` takes precedence.

## Reasoning

Claude Code's `/effort` value maps to Codex `reasoning.effort`: `low`, `medium`, `high`, `xhigh`, or `max`. A proxy override can also force `none`.

When reasoning is enabled, the proxy requests an automatic reasoning summary and translates summary deltas into Claude Code thinking blocks. Codex may omit a summary for a simple prompt. `CCP_CODEX_REASONING_SUMMARY=off` suppresses summaries while preserving effort and encrypted continuation content.

Claude Code summary compaction requests are capped at low effort by default because they perform extraction over a large transcript. `CCP_COMPACT_EFFORT=off` disables the cap, `none` removes reasoning, and another valid effort sets a different maximum. The cap never raises effort.

## Tools and multimodal input

- Claude function tools and tool results map to Responses API function calls and outputs.
- Claude Code hosted web search maps to Codex `web_search`, including supported domain filters and forced tool choice.
- Top-level base64 user images map to `input_image`.
- Supported base64 images nested in tool results also map to `input_image`.
- Remote image URLs, malformed images, and unsupported tool-result image forms remain textual placeholders.
- Strict JSON schema output maps to Responses `text.format`.

## Transport and continuation

WebSocket is the default transport. Set `CCP_CODEX_TRANSPORT=http` for HTTP SSE, or `auto` to use WebSocket with HTTP fallback only when setup fails before a request is sent.

`CCP_CODEX_PREVIOUS_RESPONSE_ID=1` enables append-only WebSocket continuation. It reuses a session connection and sends `previous_response_id` only when the translated request shape and transcript extension are safe. State is in memory, keyed by Claude Code session ID.

## Server compaction

Codex server compaction is opt-in:

```sh
CCP_CODEX_SERVER_COMPACTION=1 claude-code-proxy serve
```

At a Claude Code compaction boundary, the proxy requests native Codex compaction, keeps the encrypted item in memory for the matching session and model, and anchors replay to Claude Code's portable summary. Branches, restarts, model changes, malformed responses, expiry, or memory limits fall back to portable history. State can remain in memory for up to 30 minutes.

The boundary adds one Codex request. Structured events named `server_compaction_triggered`, `server_compaction_completed`, and `server_compaction_failed` report its outcome.

## Native Responses API

`CCP_CODEX_RESPONSES_API=1` enables `POST /v1/responses`. The proxy replaces incoming credentials with stored Codex auth and preserves native JSON or SSE response bodies. This route covers registered Codex models. Images API, response retrieval or deletion, and WebSocket ingress are outside its scope.

See [Configuration](/reference/configuration/) for every Codex setting and [Troubleshooting](/using/troubleshooting/) for auth, model, and transport failures.
