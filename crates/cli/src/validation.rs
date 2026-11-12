use std::path::Path;

pub fn is_image(path: &Path) -> std::io::Result<bool> {
    log::info!("Checking file type for {}", path.display());

    // Reads only the header bytes of the file
    if let Some(kind) = infer::get_from_path(path)? {
        log::info!(
            "Detected file type: {} ({})",
            kind.mime_type(),
            kind.extension()
        );
        Ok(kind.matcher_type() == infer::MatcherType::Image)
    } else {
        log::warn!("No known file signature detected for {}", path.display());
        Ok(false)
    }
}
