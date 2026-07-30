---
title: HTTP API
description: Canonical local HTTP routes for liveness, Anthropic Messages, token counting, model discovery, routed OpenAI APIs, and Codex images.
---

The server speaks the Anthropic and OpenAI protocol subsets needed by Claude Code and Codex-backed OpenAI-compatible clients.

<div class="security-callout">
<strong>No client authentication.</strong> The listener accepts requests without validating `Authorization` or `x-api-key`. Loopback is the default. Protect every non-loopback listener with a firewall or authenticating reverse proxy.
</div>

## `GET /healthz`

Liveness check:

```json
{"ok":true}
```

It does not verify provider credentials or upstream availability.

## `POST /v1/messages`

Accepts an Anthropic Messages request in streaming or non-streaming mode. `POST /v1/messages?beta=true` reaches the same route.

The request `model` selects the provider. The proxy translates supported message content, system prompts, thinking settings, tool definitions, tool choice, tool calls, tool results, images, output configuration, metadata, and streaming behavior according to the provider.

Streaming responses use Anthropic SSE events such as `message_start`, `content_block_start`, `content_block_delta`, `content_block_stop`, `message_delta`, and `message_stop`. Non-streaming requests are accumulated from the provider's stream.

Unknown models return HTTP 400 with the supported catalog. Missing provider auth returns HTTP 401.

## `POST /v1/messages/count_tokens`

Accepts the same basic Anthropic request shape and returns:

```json
{"input_tokens":1234}
```

Codex, Kimi, and Grok use a local `gpt-tokenizer` estimate with `o200k_base`. Cursor estimates the rendered prompt from its character length. Counts support Claude Code compaction behavior and are estimates rather than provider billing values.

## `GET /v1/models`

Returns Anthropic-shaped model discovery:

```json
{
  "data": [
    {
      "type": "model",
      "id": "gpt-5.6-sol",
      "display_name": "gpt-5.6-sol (codex)"
    }
  ],
  "has_more": false,
  "first_id": "gpt-5.6-sol",
  "last_id": "cursor:gpt-5.5"
}
```

An optional `limit` query truncates `data` and sets `has_more`. The route does not expose a pagination cursor.

Claude Code gateway discovery filters IDs according to its own model rules. See [Models and routing](/using/models-and-routing/).

## `POST /v1/chat/completions`

This route exists when `CCP_CODEX_RESPONSES_API=1` or `codex.responsesApi` is true. Despite the setting name, the request `model` can select Codex, Kimi, Grok, or Cursor Agent. Incoming bearer credentials are accepted for client compatibility and replaced with the selected provider's proxy-owned authentication.

Codex keeps its dedicated Chat Completions translator and supports:

- text messages with `system`, `developer`, `user`, and `assistant` roles
- streaming and buffered responses
- `reasoning_effort` values `none`, `low`, `medium`, `high`, `xhigh`, and `max`
- `response_format` values `text`, `json_object`, and strict `json_schema`
- `stream_options.include_usage`
- `temperature` and `top_p` on models that use the full Responses lane
- `user` as a Responses safety identifier

Omitted Codex reasoning effort defaults to `medium`. `CCP_CODEX_EFFORT` or `codex.effort` takes precedence over the request. Every translated Codex request uses `store: false`, upstream streaming, and `reasoning.context: "all_turns"`.

Kimi, Grok, and Cursor use the shared OpenAI compatibility adapter. It supports:

- `system`, `developer`, `user`, `assistant`, and `tool` messages
- text, supported user images, assistant function calls, and tool results
- function tools and named, automatic, required, or disabled tool choice
- `max_tokens` or `max_completion_tokens`
- `reasoning_effort`
- streaming, buffered responses, and `stream_options.include_usage`

A buffered response uses the standard `chat.completion` object. A streaming response emits `chat.completion.chunk` events and ends with `data: [DONE]`. Tool-call indexes are contiguous even when provider reasoning or text blocks precede them. Grok citations appear as Chat Completions annotations.

The adapter validates each provider's capabilities before making an upstream request. Unsupported non-null fields return an OpenAI `invalid_request_error` with `error.code` set to `unsupported_parameter` and the field in `error.param`. Supported fields differ between the dedicated Codex translator and the shared provider adapter. Cursor's `Read`, `Write`, and `Bash` tool bridge requires `stream: true` and a stable session header.

## `POST /v1/images/generations`

This route exists only when `CCP_CODEX_IMAGES_API=1` or `codex.imagesApi` is true. It reuses the proxy-owned ChatGPT/Codex OAuth session and forwards a bounded JSON request to the Codex image service:

```json
{
  "prompt": "A paper-cut fox in a moonlit forest",
  "model": "gpt-image-2",
  "background": "auto",
  "quality": "auto",
  "size": "auto"
}
```

`prompt` is required. `model` defaults to and is restricted to `gpt-image-2`; `background`, `quality`, and `size` default to `auto`. Optional `n` must be between 1 and 10. Unknown fields and URL response formats are rejected rather than silently forwarded. Successful responses contain `data[].b64_json`; the proxy never writes generated image data to traffic captures.

## `POST /v1/images/edits`

This route uses the same opt-in gate and accepts either:

- Codex JSON with `images: [{"image_url":"data:image/png;base64,..."}]`; or
- OpenAI-style `multipart/form-data` with one to five repeated `image` or `image[]` files and text fields `prompt`, `model`, `background`, `quality`, `size`, and `n`.

Multipart PNG, JPEG, WebP, and GIF signatures are validated and translated to Codex data URLs. The internal Codex edit contract is JSON, so multipart is an ingress compatibility adapter. Masks, remote image URLs, variations, unsupported fields, and other media types return a 4xx OpenAI error. Request bodies, individual files, aggregate inputs, responses, and concurrency are bounded to protect the proxy process.

The Images API is an internal ChatGPT Codex integration, not the public OpenAI Platform Images API. It consumes the signed-in ChatGPT account's entitlement and quota, and the internal contract can change independently of the public API.

## `POST /v1/responses`

This route exists when `CCP_CODEX_RESPONSES_API=1` or `codex.responsesApi` is true. The request `model` selects Codex, Kimi, Grok, or Cursor Agent through the same registry used by `/v1/messages` and `/v1/chat/completions`.

Registered Codex models use native Responses passthrough. The proxy validates the Codex model, replaces incoming auth with proxy-owned ChatGPT Codex auth, refreshes rejected access tokens, and preserves native JSON responses and SSE bodies.

Kimi, Grok, and Cursor use the shared OpenAI compatibility adapter. It accepts:

- string input or message input items
- `instructions`
- function calls and function-call outputs
- function tools and tool choice
- `max_output_tokens`
- `reasoning.effort`
- streaming or buffered output

The adapter maps provider text, reasoning, function calls, usage, finish status, and errors into Responses objects and lifecycle events. Grok hosted search emits `web_search_call` output items, and Grok citations use Responses URL citation annotations. Responses echo the accepted tool list and tool choice. A provider `max_tokens` stop produces an incomplete response with reason `max_output_tokens`.

`store: true` and unsupported non-null fields are rejected rather than ignored. Stored response retrieval or deletion and WebSocket client ingress are not implemented.

## OpenAI routing, sessions, and errors

Both OpenAI creation routes normalize a trailing `[1m]`, resolve configured model aliases, and select the provider from the model ID. Unknown IDs return HTTP 400 with the supported catalog. Aliases follow `aliasProvider`, while explicit provider IDs keep their provider.

The first non-empty `x-claude-code-session-id`, `session_id`, or `x-client-request-id` header supplies session identity. Stable session identity is required for Cursor tool bridging and supports provider affinity where applicable. Shared OpenAI request validation completes before session state changes.

Authentication, permission, rate-limit, invalid-request, and provider API failures use OpenAI error envelopes. Rate-limit responses preserve `Retry-After`. Malformed provider streams and provider response-size violations return gateway errors rather than blaming the client.

The monitor and structured logs record the selected provider and resolved model. Optional traffic capture records the OpenAI request, translated Anthropic request, provider request and response where available, intermediate Anthropic SSE, and bounded downstream OpenAI output. Traffic artifacts preserve prompts and tool content.

## Other routes

Unmatched paths return the proxy's not-found response. The server has no administrative mutation API, credential API, or remote shutdown route.
