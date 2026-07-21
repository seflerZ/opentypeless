fn main() {
    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=framework=Speech");

    #[cfg(all(target_os = "windows", target_env = "msvc"))]
    println!(
        "cargo:rustc-link-arg=/MANIFESTDEPENDENCY:type='win32' name='Microsoft.Windows.Common-Controls' version='6.0.0.0' processorArchitecture='*' publicKeyToken='6595b64144ccf1df' language='*'"
    );

    #[cfg(all(target_os = "windows", target_env = "gnu"))]
    {
        // Generate a manifest resource for Common Controls v6 (visual styles)
        // using windres, since GNU ld does not support /MANIFESTDEPENDENCY.
        let manifest = r#"<?xml version="1.0" encoding="UTF-8" standalone="yes"?>
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <dependency>
    <dependentAssembly>
      <assemblyIdentity
        type="win32"
        name="Microsoft.Windows.Common-Controls"
        version="6.0.0.0"
        processorArchitecture="*"
        publicKeyToken="6595b64144ccf1df"
        language="*"
      />
    </dependentAssembly>
  </dependency>
</assembly>"#;
        let out_dir = std::path::PathBuf::from(
            std::env::var("OUT_DIR").expect("OUT_DIR not set"),
        );
        let manifest_path = out_dir.join("app.manifest");
        let rc_path = out_dir.join("app.rc");
        let res_path = out_dir.join("app.res");
        std::fs::write(&manifest_path, manifest).expect("failed to write manifest");
        // Use forward slashes in the RC file to avoid backslash escape issues
        let manifest_posix = manifest_path.display().to_string().replace('\\', "/");
        std::fs::write(
            &rc_path,
            format!("1 24 \"{}\"", manifest_posix),
        )
        .expect("failed to write rc file");
        let status = std::process::Command::new("windres")
            .arg(&rc_path)
            .arg("-o")
            .arg(&res_path)
            .status()
            .expect("failed to run windres");
        if status.success() {
            println!(
                "cargo:rustc-link-arg-bin=opentypeless={}",
                res_path.display()
            );
        } else {
            panic!("windres failed (status: {})", status);
        }
    }

    tauri_build::build()
}
