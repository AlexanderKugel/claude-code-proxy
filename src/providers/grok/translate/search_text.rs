//! Text rendering of a hosted search that xAI already ran.
//!
//! The faithful translation of hosted search is `server_tool_use` followed by
//! `web_search_tool_result` or `x_search_tool_result`. Those two block types
//! are unrenderable in some Anthropic clients -- notably the Claude Code VS
//! Code webview, whose content-block renderer knows `text`, `thinking`,
//! `redacted_thinking`, `tool_use`, `tool_result`, `image`, `document`, and
//! `fallback`, and prints "Unsupported content type" for anything else.
//!
//! A `text` block says the same thing and draws everywhere, so it is the
//! default shape. `CCP_GROK_SEARCH_BLOCKS=native` selects the block types
//! above instead.

/// One line naming a search the model already ran. The result payload is not
/// included: xAI returns the findings as URL citations on the answer text, not
/// as a separate result set, so the result block was always empty.
pub fn search_line(name: &str, query: &str) -> String {
    let label = match name {
        "x_search" => "X search",
        "web_search" => "web search",
        other => other,
    };
    let query = query.trim();
    if query.is_empty() {
        format!("[{label}]\n")
    } else {
        format!("[{label}: {query}]\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn names_map_to_readable_labels() {
        assert_eq!(search_line("web_search", "cars"), "[web search: cars]\n");
        assert_eq!(search_line("x_search", "outage"), "[X search: outage]\n");
    }

    #[test]
    fn an_unknown_hosted_tool_keeps_its_own_name() {
        assert_eq!(
            search_line("news_search", "budget"),
            "[news_search: budget]\n"
        );
    }

    #[test]
    fn an_empty_query_drops_the_colon() {
        // Grok emits follow-up search events with no query of their own.
        assert_eq!(search_line("web_search", "   "), "[web search]\n");
    }
}
