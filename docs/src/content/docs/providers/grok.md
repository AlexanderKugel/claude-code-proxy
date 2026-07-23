---
title: Grok
description: Configure grok.com authentication, Grok models, reasoning, function tools, hosted web and X search, citations, and provider overrides.
---

Grok uses the Responses endpoint at `https://cli-chat-proxy.grok.com/v1/responses`.

## Account and authentication

Use a **grok.com account**. Browser login uses S256 PKCE through `auth.x.ai` and an ephemeral loopback callback:

```sh
claude-code-proxy grok auth login
```

For a headless host, use the device-code flow:

```sh
claude-code-proxy grok auth device
claude-code-proxy grok auth status
```

The proxy owns and refreshes its Grok tokens. It does not read `~/.grok/auth.json`.

## Models

The registered IDs are `grok-composer-2.5-fast` and `grok-4.5`. Account and regional access can vary. Use the same concrete Grok ID for `ANTHROPIC_MODEL` and `ANTHROPIC_SMALL_FAST_MODEL`.

```sh
ANTHROPIC_MODEL=grok-4.5 \
ANTHROPIC_SMALL_FAST_MODEL=grok-4.5 \
  claude --model grok-4.5
```

## Reasoning and tools

The proxy translates Claude messages, function tools, tool results, thinking controls, token usage, and streaming events. Grok reasoning text appears as Claude Code thinking blocks.

Claude Code hosted search tools map to Grok-native tools:

- General web queries use hosted web search.
- X queries use hosted `x_search`.
- Citations and search usage return in Anthropic-compatible content and usage fields.

## Multimodal support

Grok support in claude-code-proxy focuses on text, reasoning, function tools, and hosted search. Treat image and other media behavior as unsupported unless the current source and tests explicitly cover it.

## Configuration

- `CCP_GROK_BASE_URL` or `grok.baseUrl` changes the API base URL.
- `CCP_GROK_CLIENT_VERSION` or `grok.clientVersion` changes the client version header.

See [Configuration](/reference/configuration/) for defaults.

## Limitations and troubleshooting

A successful login does not guarantee every model is enabled for the account or region. Model rejection and upstream errors are surfaced to Claude Code. Use `grok auth status` for token state, inspect the failed request in the monitor, and use the structured log or error capture for the full redacted response.
