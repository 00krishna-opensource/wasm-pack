use command::utils::set_crate_path;
use error::Error;
use npm;
use slog::Logger;
use std::path::Path;
use std::result;
use PBAR;

pub fn publish(path: Option<String>, log: &Logger) -> result::Result<(), Error> {
    let crate_path = set_crate_path(path);

    info!(&log, "Publishing the npm package...");
    info!(&log, "npm info located in the npm debug log");
    let pkg_directory: Box<&Path> = find_pkg_directory(&crate_path).ok_or(Error::PkgNotFound {
        message: format!(
            "Unable to find the pkg directory at path '{}', or in a child directory of '{}'",
            &crate_path, &crate_path
        ),
    })?;

    npm::npm_publish(&pkg_directory.to_string_lossy())?;
    info!(&log, "Published your package!");

    PBAR.message("💥  published your package!");
    Ok(())
}

fn find_pkg_directory(guess_path: &str) -> Option<Box<&Path>> {
    let path = Path::new(guess_path);
    if is_pkg_directory(path) {
        return Some(Box::new(path));
    }

    path.parent().and_then(|v| {
        if is_pkg_directory(v) {
            Some(Box::new(v))
        } else {
            None
        }
    })
}

fn is_pkg_directory(path: &Path) -> bool {
    path.exists() && path.is_dir() && path.ends_with("pkg")
}
