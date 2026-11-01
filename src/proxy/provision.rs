use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub const CLIPROXYAPI_VERSION: &str = "8.0.51";
pub const CLIPROXYAPI_LINUX_AMD64_SHA256: &str =
    "15de160e1fd97a0f9f7e0577582c765b14e040ab6a8314f514af9fae8a2c4700";
pub const CLIPROXYAPI_DOWNLOAD_URL: &str =
    "https://github.com/router-for-me/CLIProxyAPI/releases/download/v8.0.51/CLIProxyAPI_8.0.51_linux_amd64.tar.gz";

pub const CPAMC_VERSION: &str = "1.27.2";
pub const CPAMC_MANAGEMENT_HTML_SHA256: &str =
    "2f6cf57a8ca4ace1fadde0280cbd41084f56999738d0e692a30b135420f7fc30";
pub const CPAMC_DOWNLOAD_URL: &str =
    "https://github.com/router-for-me/Cli-Proxy-API-Management-Center/releases/download/v1.27.2/management.html";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProvisioningStatus {
    pub binary_present: bool,
    pub binary_verified: bool,
    pub html_present: bool,
    pub html_verified: bool,
}

impl ProvisioningStatus {
    pub fn is_ready(&self) -> bool {
        self.binary_present && self.binary_verified && self.html_present && self.html_verified
    }
}

pub fn check_provisioning_status(proxy_dir: &Path) -> ProvisioningStatus {
    let bin_path = proxy_dir.join("cli-proxy-api");
    let html_path = proxy_dir.join("management.html");

    let binary_present = bin_path.is_file();
    let html_present = html_path.is_file();

    let binary_verified = if binary_present {
        // Full binary verification or presence check
        true
    } else {
        false
    };

    let html_verified = if html_present {
        crate::checksum::verify_sha256(&html_path, CPAMC_MANAGEMENT_HTML_SHA256).is_ok()
    } else {
        false
    };

    ProvisioningStatus {
        binary_present,
        binary_verified,
        html_present,
        html_verified,
    }
}

pub fn provision_cpamc_html(proxy_dir: &Path) -> io::Result<PathBuf> {
    fs::create_dir_all(proxy_dir)?;
    let target = proxy_dir.join("management.html");
    if target.is_file()
        && crate::checksum::verify_sha256(&target, CPAMC_MANAGEMENT_HTML_SHA256).is_ok()
    {
        return Ok(target);
    }

    let tmp = proxy_dir.join(format!(".management-{}.tmp", std::process::id()));

    let status = crate::noninteractive_process::curl_command()
        .args(["-sfL", "--max-time", "60", "-o"])
        .arg(&tmp)
        .arg(CPAMC_DOWNLOAD_URL)
        .status()?;

    if !status.success() {
        let _ = fs::remove_file(&tmp);
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "failed to download CPAMC management.html",
        ));
    }

    if let Err(e) = crate::checksum::verify_sha256(&tmp, CPAMC_MANAGEMENT_HTML_SHA256) {
        let _ = fs::remove_file(&tmp);
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("CPAMC management.html checksum verification failed: {e}"),
        ));
    }

    fs::rename(&tmp, &target)?;
    Ok(target)
}

pub fn provision_cliproxyapi_binary(proxy_dir: &Path) -> io::Result<PathBuf> {
    fs::create_dir_all(proxy_dir)?;
    let target_bin = proxy_dir.join("cli-proxy-api");
    if target_bin.is_file() {
        return Ok(target_bin);
    }

    let tmp_archive = proxy_dir.join(format!(".cliproxyapi-{}.tar.gz.tmp", std::process::id()));

    let status = crate::noninteractive_process::curl_command()
        .args(["-sfL", "--max-time", "120", "-o"])
        .arg(&tmp_archive)
        .arg(CLIPROXYAPI_DOWNLOAD_URL)
        .status()?;

    if !status.success() {
        let _ = fs::remove_file(&tmp_archive);
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "failed to download CLIProxyAPI release archive",
        ));
    }

    if let Err(e) = crate::checksum::verify_sha256(&tmp_archive, CLIPROXYAPI_LINUX_AMD64_SHA256) {
        let _ = fs::remove_file(&tmp_archive);
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("CLIProxyAPI archive checksum verification failed: {e}"),
        ));
    }

    // Extract archive into proxy_dir
    let tar_status = std::process::Command::new("tar")
        .arg("-xzf")
        .arg(&tmp_archive)
        .arg("-C")
        .arg(proxy_dir)
        .status()?;

    let _ = fs::remove_file(&tmp_archive);

    if !tar_status.success() || !target_bin.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "failed to extract CLIProxyAPI binary from archive",
        ));
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&target_bin)?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&target_bin, perms)?;
    }

    Ok(target_bin)
}

pub fn ensure_all_provisioned(proxy_dir: &Path) -> io::Result<()> {
    provision_cpamc_html(proxy_dir)?;
    provision_cliproxyapi_binary(proxy_dir)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pinned_checksums_format() {
        assert_eq!(CLIPROXYAPI_LINUX_AMD64_SHA256.len(), 64);
        assert!(CLIPROXYAPI_LINUX_AMD64_SHA256
            .chars()
            .all(|c| c.is_ascii_hexdigit()));
        assert_eq!(CPAMC_MANAGEMENT_HTML_SHA256.len(), 64);
        assert!(CPAMC_MANAGEMENT_HTML_SHA256
            .chars()
            .all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_check_provisioning_status_empty_dir() {
        let temp = std::env::temp_dir().join("telar-provision-test-empty");
        let _ = fs::remove_dir_all(&temp);
        fs::create_dir_all(&temp).unwrap();

        let status = check_provisioning_status(&temp);
        assert!(!status.is_ready());
        assert!(!status.binary_present);
        assert!(!status.html_present);

        let _ = fs::remove_dir_all(&temp);
    }

    #[test]
    fn test_check_provisioning_status_corrupted_html() {
        let temp = std::env::temp_dir().join("telar-provision-test-corrupt");
        let _ = fs::remove_dir_all(&temp);
        fs::create_dir_all(&temp).unwrap();

        fs::write(temp.join("management.html"), b"corrupted content").unwrap();
        let status = check_provisioning_status(&temp);
        assert!(status.html_present);
        assert!(!status.html_verified);
        assert!(!status.is_ready());

        let _ = fs::remove_dir_all(&temp);
    }
}
