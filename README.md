# aidos-rs

A server that allows use of bangs like duckduckgo with any search engine.

Search terms are specified in `config.toml`. `default` must be present, and
is the normal search engine used. All other engines are placed in the form
of `bang = "search_url"`. Some examples are shown in `config.toml`.
