#[cfg(not(target_family = "wasm"))]
#[path = "tokio.rs"]
mod platform;

#[cfg(target_family = "wasm")]
#[path = "gloo.rs"]
mod platform;

#[doc(inline)]
pub use self::platform::*;

use std::time::Duration;

/// Forced cool down duration performed at every request. E621 allows at most 2 requests per second,
/// so the lowest safe value we can have here is 500 ms.
const REQ_COOLDOWN_DURATION: Duration = Duration::from_millis(600);
