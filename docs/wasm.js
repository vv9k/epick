function on_wasm_loaded() {
    // This call installs a bunch of callbacks and then returns.
    wasm_bindgen.start("the_canvas_id");
}

// We'll defer our execution until the wasm is ready to go.
// Here we tell bindgen the path to the wasm file so it can start
// initialization and return to us a promise when it's done.
wasm_bindgen("./epick_bg.wasm")
    .then(on_wasm_loaded)["catch"](console.error);

