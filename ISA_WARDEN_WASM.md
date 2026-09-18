# ISA Warden browser timer patch

This branch starts at upstream revision `93ab219d69f09d8f999851b0359c80ebe6726102`, the source revision of the published `surrealdb-core` 3.2.4 crate. It carries ISA Warden's existing browser timer compatibility patch; it does not upgrade the database engine.

The datastore bootstrap, transaction retries, query timeouts, and sleep paths use a crate-local timer adapter: Tokio on native targets and `wasmtimer` on WASM. Query cancellation uses an abortable browser-local task on WASM. This addresses the embedded-engine failure described in [upstream issue #6711](https://github.com/surrealdb/surrealdb/issues/6711).

The core manifest resolves sibling packages from crates.io, matching the published crate. This allows consumers to patch only `surrealdb-core` without introducing incompatible Git-sourced copies of types also used by the registry SDK.

ISA Warden consumes this branch through a public HTTPS Git URL pinned to a full commit SHA. Validation lives in its `chat-ui-3/scripts/browser-chat` harness and covers Chromium and Firefox, main-thread and worker persistence, transactions, lifecycle, and failures. Remove the override only after an upstream replacement passes the same suite.
