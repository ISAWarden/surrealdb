#[cfg(not(target_family = "wasm"))]
pub(crate) use tokio::time::*;
#[cfg(target_family = "wasm")]
pub(crate) use std::time::Duration;
#[cfg(target_family = "wasm")]
pub(crate) use wasmtimer::{std::Instant, tokio::*};
