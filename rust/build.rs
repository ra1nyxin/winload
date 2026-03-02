fn main() {
    println!("cargo:rustc-env=TARGET={}", std::env::var("TARGET").unwrap());

    #[cfg(target_os = "windows")]
    {
        use std::path::Path;
        use winres::WindowsResource;

        let manifest_path = Path::new("winload.exe.manifest");
        if manifest_path.exists() {
            println!("cargo:rerun-if-changed={}", manifest_path.display());

            let mut res = WindowsResource::new();
            res.set_manifest(
                r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
      <requestedPrivileges>
        <requestedExecutionLevel level="requireAdministrator" uiAccess="false" />
      </requestedPrivileges>
    </security>
  </trustInfo>
</assembly>
"#,
            );

            if let Err(e) = res.compile() {
                eprintln!("Warning: Failed to compile resources: {}", e);
            }
        }
    }
}
