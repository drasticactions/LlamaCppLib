fn main() {
    bindgen::Builder::default()
        .header("../../external/llama.cpp/llama.h")
        .generate().unwrap()
        .write_to_file("llama.rs").unwrap();

    csbindgen::Builder::default()
        .input_bindgen_file("llama.rs")            // read from bindgen generated code
        .rust_file_header("use super::llama::*;")     // import bindgen generated modules(struct/method)
        .csharp_dll_name("llama")
        .csharp_namespace("LLamaCppLib")
        .generate_to_file("llama_ffi.rs", "../../src/LLamaCppLib/NativeMethods.g.cs")
        .unwrap();
}
