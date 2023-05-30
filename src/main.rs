use learn_wgpu::run;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

fn main() {
    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_futures::spawn_local(run());

    // Use tokio to run the future on the main thread.
    #[cfg(not(target_arch = "wasm32"))]
    tokio::runtime::Runtime::new().unwrap().block_on(run());
}
