use winresource::{VersionInfo, WindowsResource};

const MANIFEST: &str = r#"
<assembly xmlns="urn:schemas-microsoft-com:asm.v1" manifestVersion="1.0">
  <trustInfo xmlns="urn:schemas-microsoft-com:asm.v3">
    <security>
      <requestedPrivileges>
        <requestedExecutionLevel level="asInvoker" uiAccess="false"/>
      </requestedPrivileges>
    </security>
  </trustInfo>
  <compatibility xmlns="urn:schemas-microsoft-com:compatibility.v1">
    <application>
      <supportedOS Id="{8e0f7a12-bfb3-4fe8-b9a5-48fd50a15a9a}"/>
    </application>
  </compatibility>
  <application xmlns="urn:schemas-microsoft-com:asm.v3">
    <windowsSettings>
      <dpiAware xmlns="http://schemas.microsoft.com/SMI/2005/WindowsSettings">true/pm</dpiAware>
      <dpiAwareness xmlns="http://schemas.microsoft.com/SMI/2016/WindowsSettings">PerMonitorV2, PerMonitor</dpiAwareness>
    </windowsSettings>
  </application>
</assembly>
"#;

fn main() {
    let config = slint_build::CompilerConfiguration::new()
        .with_bundled_translations("lang")
        .with_default_translation_context(slint_build::DefaultTranslationContext::None);
    slint_build::compile_with_config("ui/app-window.slint", config).expect("Slint compilation failed");

    let date = build_date();
    println!("cargo:rustc-env=APP_BUILD_DATE={date}");

    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows") {
        println!("cargo:rerun-if-changed=Icon.ico");
        let (version, version_text) = windows_version(&date);
        WindowsResource::new()
            .set_icon("Icon.ico")
            .set("FileDescription", "Ports Launcher")
            .set("ProductName", "Ports Launcher")
            .set("OriginalFilename", "ports_launcher.exe")
            .set("InternalName", "ports_launcher")
            .set("CompanyName", "Nyaldee")
            .set("LegalCopyright", "Copyright © 2026 Nyaldee")
            .set("FileVersion", &version_text)
            .set("ProductVersion", &date.replace('-', "."))
            .set_version_info(VersionInfo::FILEVERSION, version)
            .set_version_info(VersionInfo::PRODUCTVERSION, version)
            .set_manifest(MANIFEST)
            .compile()
            .expect("failed to embed Windows resources");
    }
}

fn build_date() -> String {
    println!("cargo:rerun-if-env-changed=BUILD_DATE");
    std::env::var("BUILD_DATE").unwrap_or_else(|_| chrono::Utc::now().format("%Y-%m-%d").to_string())
}

fn windows_version(date: &str) -> (u64, String) {
    let parts: Vec<u64> = date.split('-').filter_map(|p| p.parse().ok()).collect();
    let &[y, m, d] = parts.as_slice() else { panic!("invalid BUILD_DATE: {date} (expected YYYY-MM-DD)") };
    (y << 48 | m << 32 | d << 16, format!("{y}.{m}.{d}.0"))
}
