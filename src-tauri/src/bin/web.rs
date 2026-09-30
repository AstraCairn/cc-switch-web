fn main() {
    #[cfg(not(feature = "desktop"))]
    cc_switch_lib::web::serve();

    #[cfg(feature = "desktop")]
    {
        eprintln!(
            "cc-switch-web 需要去掉桌面功能再编译:\n  \
             cargo build --manifest-path src-tauri/Cargo.toml --no-default-features --features web --bin cc-switch-web"
        );
        std::process::exit(1);
    }
}
