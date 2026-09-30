#[cfg_attr(feature = "desktop", tauri::command)]
pub fn enter_lightweight_mode(app: crate::host::AppHandle) -> Result<(), String> {
    #[cfg(feature = "desktop")]
    {
        return crate::lightweight::enter_lightweight_mode(&app);
    }
    #[cfg(not(feature = "desktop"))]
    {
        let _ = app;
        Ok(())
    }
}

#[cfg_attr(feature = "desktop", tauri::command)]
pub fn exit_lightweight_mode(app: crate::host::AppHandle) -> Result<(), String> {
    #[cfg(feature = "desktop")]
    {
        return crate::lightweight::exit_lightweight_mode(&app);
    }
    #[cfg(not(feature = "desktop"))]
    {
        let _ = app;
        Ok(())
    }
}

#[cfg_attr(feature = "desktop", tauri::command)]
pub fn is_lightweight_mode() -> bool {
    #[cfg(feature = "desktop")]
    {
        crate::lightweight::is_lightweight_mode()
    }
    #[cfg(not(feature = "desktop"))]
    {
        false
    }
}
