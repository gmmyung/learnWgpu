use learn_wgpu::run;

fn main() {
    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_futures::spawn_local(run());

    // Use tokio to run the future on the main thread.
    #[cfg(not(target_arch = "wasm32"))]
    tokio::runtime::Runtime::new().unwrap().block_on(run());
}
