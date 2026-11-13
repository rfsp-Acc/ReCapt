fn main() {
    // O Metal do whisper.cpp usa __isPlatformVersionAtLeast, que vem do runtime do clang
    // e nao e ligado sozinho em alguns toolchains de macOS.
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("macos") {
        if let Ok(out) = std::process::Command::new("clang").arg("--print-resource-dir").output() {
            let dir = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if !dir.is_empty() {
                println!("cargo:rustc-link-search={dir}/lib/darwin");
                println!("cargo:rustc-link-lib=static=clang_rt.osx");
            }
        }
    }
    tauri_build::build()
}
