---
title: What is claude-code-proxy?
description: Run Claude Code with Codex, Kimi, Grok, or Cursor Agent through one local Anthropic-compatible proxy.
---

<div class="hero-copy">
claude-code-proxy keeps the Claude Code harness and routes its requests to Codex, Kimi, Grok, or Cursor Agent. One local process handles authentication, provider selection, protocol translation, streaming, and diagnostics.
</div>

<div class="route-rail" aria-label="Request route from Claude Code through the proxy to upstream providers">
  <div class="route-node"><strong>Claude Code</strong><span>Anthropic messages and tools</span></div>
  <div class="route-arrow" aria-hidden="true">→</div>
  <div class="route-node"><strong>claude-code-proxy</strong><span>routing, translation, auth</span></div>
  <div class="route-arrow" aria-hidden="true">→</div>
  <div class="route-node provider-stack"><code>Codex</code><code>Kimi</code><code>Grok</code><code>Cursor Agent</code></div>
</div>

## Why use it?

- **Keep the Claude Code experience.** Skills, tools, hooks, subagents, IDE integrations, and the terminal interface stay on the client side.
- **Use subscription-backed providers.** Authenticate with supported consumer accounts instead of putting provider API keys into Claude Code.
- **Switch providers by model.** A single proxy process routes every request from its model ID.
- **Preserve streaming and tools.** The proxy translates provider-specific streams, reasoning, tool calls, results, images, and usage into Anthropic-shaped responses.
- **See what is happening.** The monitor TUI shows sessions, requests, errors, models, token use, and throughput. Structured logs and optional traffic captures support deeper diagnosis.

![Claude Code running through claude-code-proxy](/claude-code-screenshot.webp)

## The operating boundary

claude-code-proxy is a local compatibility layer. It does not replace Claude Code, manage Claude Code profiles, or authenticate incoming clients. Claude Code connects through `ANTHROPIC_BASE_URL`, while `CCP_*` settings configure the proxy itself.

<div class="security-callout">
<strong>Protect the listener and your account.</strong> The proxy binds to loopback by default and accepts requests without client authentication. Keep it on loopback unless a firewall or authenticating reverse proxy protects it. Provider policies and account enforcement can change, and unofficial clients may carry account risk.
</div>

## Next steps

Start with the [short Codex setup](/getting-started/), compare the [supported providers](/providers/choosing-a-provider/), or read [how requests flow](/how-it-works/).
