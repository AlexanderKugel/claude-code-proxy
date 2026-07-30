---
title: How it works
description: Follow authentication, routing, protocol translation, streaming, session state, and diagnostics through claude-code-proxy.
---

claude-code-proxy exposes Anthropic Messages and optional OpenAI-compatible creation APIs. Every route selects a provider from the request model and translates through that provider's native protocol.

<div class="route-rail" aria-label="claude-code-proxy request architecture">
  <div class="route-node"><strong>API client</strong><span>Anthropic Messages<br/>OpenAI Chat or Responses</span></div>
  <div class="route-arrow" aria-hidden="true">→</div>
  <div class="route-node"><strong>Proxy pipeline</strong><span>route model<br/>refresh auth<br/>translate events</span></div>
  <div class="route-arrow" aria-hidden="true">→</div>
  <div class="route-node provider-stack"><code>Codex Responses</code><code>Kimi Chat Completions</code><code>Grok Responses</code><code>Cursor Connect</code></div>
</div>

## Request lifecycle

1. Claude Code sends an Anthropic Messages request to `/v1/messages`.
2. The registry normalizes a trailing `[1m]`, resolves aliases, and selects a provider from the model ID.
3. The provider loads its proxy-owned credential and refreshes an expiring access token when needed.
4. The request translator maps system content, user and assistant messages, images, tools, tool results, thinking controls, and output settings into the upstream shape.
5. The upstream stream is reduced into typed text, thinking, tool, usage, and completion events.
6. The proxy emits Anthropic SSE events or accumulates a non-streaming Anthropic response.
7. The monitor, JSONL logger, and optional traffic capture record operational details.

## OpenAI-compatible lifecycle

`POST /v1/chat/completions` and `POST /v1/responses` use the same model registry as Anthropic Messages. Codex retains dedicated paths: Responses requests pass through natively, while Chat Completions uses the Codex compatibility translator.

Kimi, Grok, and Cursor requests pass through a strict OpenAI ingress adapter into an Anthropic Messages request. The selected provider returns typed buffered or live Anthropic SSE. One egress adapter decodes those events into the requested OpenAI surface, which keeps buffered and streaming text, reasoning, tools, usage, citations, completion status, and errors aligned.

This shared intermediate stream avoids decoding an already-built HTTP response. It also gives monitoring and traffic capture named stages for the OpenAI request, translated request, provider traffic, intermediate events, and downstream output.

## Authentication boundary

Each provider login belongs to claude-code-proxy. The proxy does not read native Codex, Grok, or Cursor Agent credentials. Credentials live in the platform credential store described in [Files and storage](/reference/files-and-storage/). Incoming `ANTHROPIC_AUTH_TOKEN` values are accepted for client compatibility and are not used as upstream credentials.

## Routing boundary

Routing happens per request, not per server process or API surface. Codex IDs, Kimi IDs, Grok IDs, Cursor prefixes, and configured Anthropic-style aliases can share one listener across `/v1/messages`, `/v1/chat/completions`, and `/v1/responses`. Unknown model IDs return HTTP 400 with the supported catalog.

## Session state

Claude Code sends `x-claude-code-session-id`. The proxy uses it for monitor grouping and provider features that need continuity. Cursor conversation IDs, optional Codex `previous_response_id`, and optional Codex server compaction state live in memory. A proxy restart clears that state and portable Claude Code history remains the fallback.

## Count tokens

`POST /v1/messages/count_tokens` performs a local estimate with `gpt-tokenizer` and the `o200k_base` encoding. It supports Claude Code's compaction decisions without an upstream request.

See [HTTP API](/reference/http-api/) for route contracts and [Compatibility and limitations](/reference/compatibility-and-limitations/) for translation boundaries.
