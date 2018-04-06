const rust = import("./wasm_greet");
rust.then(m => m.greet("World!"));
