use std::path::Path;

pub fn is_image(path: &Path) -> std::io::Result<bool> {
    // Reads only the header bytes of the file
    if let Some(kind) = infer::get_from_path(path)? {
        Ok(kind.matcher_type() == infer::MatcherType::Image)
    } else {
        Ok(false)
    }
}
