//! `rivia-file` provides orchestration for the manipulation of file content
//!
//! ### Example
//! ```
//! use rivia_file::prelude::*;
//! ```
mod edit;
use std::io::{BufRead, BufReader};

use regex::Regex;
use rivia_vfs::prelude::*;

/// All essential symbols in a simple consumable form
///
/// ### Examples
/// ```
/// use rivia_file::prelude::*;
/// ```
pub mod prelude
{
    // Re-exports
    pub use regex::Regex;
    pub use rivia_vfs::prelude::*;

    // Export internal types
    pub mod file
    {
        pub use crate::*;
    }
}

/// Returns the first captured string from the given regular expression
///
/// * Handles path expansion and absolute path resolution
///
/// ### Examples
/// ```
/// use rivia_file::prelude::*;
///
/// assert!(vfs::set_memfs().is_ok());
/// let file1 = vfs::root().mash("file1");
/// assert!(vfs::write_all(&file1, "Not my favorite movie: 'Citizen Kane' (1941).").is_ok());
/// assert_eq!(file::extract(&file1, r"'([^']+)'\s+\((\d{4})\)").unwrap(), "Citizen Kane");
/// ```
pub fn extract<T: AsRef<Path>, U: AsRef<str>>(path: T, rx: U) -> RvResult<String>
{
    extract_rx(path, &Regex::new(rx.as_ref()).map_err(|_| FileError::FailedToExtractString)?)
}

/// Returns the first captured string from the given regular expression
///
/// * Handles path expansion and absolute path resolution
///
/// ### Examples
/// ```
/// use rivia_file::prelude::*;
///
/// assert!(vfs::set_memfs().is_ok());
/// let file1 = vfs::root().mash("file1");
/// let rx = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
/// assert!(vfs::write_all(&file1, "Not my favorite movie: 'Citizen Kane' (1941).").is_ok());
/// assert_eq!(file::extract_rx(&file1, &rx).unwrap(), "Citizen Kane");
/// ```
pub fn extract_rx<T: AsRef<Path>>(path: T, rx: &Regex) -> RvResult<String>
{
    let data = vfs::read_all(path)?;
    let caps = rx.captures(&data).ok_or(FileError::FailedToExtractString)?;
    let value = caps.get(1).ok_or(FileError::FailedToExtractString)?;
    Ok(value.as_str().to_string())
}

/// Returns all the captured strings from the given regular expression
///
/// * Handles path expansion and absolute path resolution
///
/// ### Examples
/// ```
/// use rivia_file::prelude::*;
///
/// assert!(vfs::set_memfs().is_ok());
/// let file1 = vfs::root().mash("file1");
/// assert!(vfs::append_all(&file1, "Not my favorite movie: 'Citizen Kane' (1941)\n").is_ok());
/// assert!(vfs::append_all(&file1, "Another not great movie: 'Zoolander' (2001)").is_ok());
/// assert_eq!(file::extract_all(&file1, r"'[^']+'\s+\(\d{4}\)").unwrap(), vec!["'Citizen Kane' (1941)", "'Zoolander' (2001)"]);
/// ```
pub fn extract_all<T: AsRef<Path>, U: AsRef<str>>(path: T, rx: U) -> RvResult<Vec<String>>
{
    extract_all_rx(path, &Regex::new(rx.as_ref()).map_err(|_| FileError::FailedToExtractString)?)
}

/// Returns all the captured strings from the given regular expression
///
/// * Handles path expansion and absolute path resolution
///
/// ### Examples
/// ```
/// use rivia_file::prelude::*;
///
/// assert!(vfs::set_memfs().is_ok());
/// let file1 = vfs::root().mash("file1");
/// let rx = Regex::new(r"'[^']+'\s+\(\d{4}\)").unwrap();
/// assert!(vfs::append_all(&file1, "Not my favorite movie: 'Citizen Kane' (1941)\n").is_ok());
/// assert!(vfs::append_all(&file1, "Another not great movie: 'Zoolander' (2001)").is_ok());
/// assert_eq!(file::extract_all_rx(&file1, &rx).unwrap(), vec!["'Citizen Kane' (1941)", "'Zoolander' (2001)"]);
/// ```
pub fn extract_all_rx<T: AsRef<Path>>(path: T, rx: &Regex) -> RvResult<Vec<String>>
{
    let data = vfs::read_all(path)?;
    let mut values = vec![];
    for cap in rx.captures_iter(&data) {
        values.append(
            &mut cap.iter().filter_map(|x| x.and_then(|y| Some(y.as_str().to_string()))).collect::<Vec<String>>(),
        );
    }
    Ok(values)
}

/// Insert lines at the location determined by the regular expression and offset
///
/// * Handles path expansion and absolute path resolution
/// * Offset
///
/// ### Examples
/// ```
/// use rivia_file::prelude::*;
///
/// assert!(vfs::set_memfs().is_ok());
/// let file1 = vfs::root().mash("file1");
/// let rx = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
/// assert!(vfs::write_all(&file1, "Not my favorite movie: 'Citizen Kane' (1941).").is_ok());
/// assert_eq!(file::extract_rx(&file1, &rx).unwrap(), "Citizen Kane");
/// ```
pub fn insert_rx<T: AsRef<Path>, U: AsRef<str>>(path: T, lines: &[U], rx: &Regex) -> RvResult<()>
{
    // Match regex on file's lines for insert location
    let mut loc = -1;
    let data = vfs::read_all(path)?;
    for line in BufReader::new(data.as_bytes()).lines() {
        loc = loc + 1;
        let line = line?;
        if rx.is_match(&line) {
            break;
        }
    }
    if loc == -1 {
        return Err(FileError::InsertLocationNotFound.into());
    }

    // Write out the modified file

    Ok(())
}

// {edit: /etc/sudoers, insert: append,  "builder ALL=(ALL) NOPASSWD: ALL"}
// {edit: /root/.bashrc, regex: '|^(export PATH.*)|\1:/opt/<%=distro%>/bin|'}
// {edit: /etc/skel/.bashrc, regex: '|^(export PATH.*)|\1:/opt/<%=distro%>/bin|'}
// {edit: /etc/hosts, insert: append,  '127.0.0.1 localhost'}
//   - edit: /etc/locale.conf insert: append values:
//       - 'LANG=<%=language%>.<%=character_set%>'
//       - 'LANGUAGE=<%=language%>.<%=character_set%>'
//   - {edit: /etc/locale.gen, regex: '|^#(<%=language%>\..*)|\1|'}
//  - {edit: /etc/profile.d/locale.sh, insert: append,  'export LC_COLLATE=C'}
//   - {edit: /etc/profile.d/locale.sh, insert: append,  'export
//     LC_ALL=<%=language%>.<%=character_set%>'}
//   - edit: /etc/lsb-release insert: append values:
//       - 'LSB_VERSION=1.4'
//       - 'DISTRIB_ID=<%=distro%>'
//       - 'DISTRIB_RELEASE=rolling'
// - 'DISTRIB_DESCRIPTION=<%=distro%>'

//       # Minimal amount of swapping without disabling it entirely
//       - {edit: '/etc/sysctl.d/10-<%=distro%>.conf', insert: append,  "vm.swappiness = 1"}
//       # Enable kernel ipv4 forwarding for containers
//       - {edit: '/etc/sysctl.d/10-<%=distro%>.conf', insert: append,  "net.ipv4.ip_forward = 1"}
//       # Disable ipv6 forwarding
//       - {edit: '/etc/sysctl.d/10-<%=distro%>.conf', insert: append, "net.ipv6.conf.all.forwarding
//         = 0"}
//       # Increase the number of user file watches to max
//       - {edit: '/etc/sysctl.d/10-<%=distro%>.conf', insert: append,  "fs.inotify.max_user_watches
//         = 524288"}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests
{
    use crate::prelude::*;

    #[test]
    fn test_extract()
    {
        let tmpdir = assert_memfs_setup!();
        let file1 = tmpdir.mash("file1");
        assert!(vfs::write_all(&file1, "Not my favorite movie: 'Citizen Kane' (1941).").is_ok());
        assert_eq!(file::extract(&file1, r"'([^']+)'\s+\((\d{4})\)").unwrap(), "Citizen Kane");
        assert_remove_all!(&tmpdir);
    }

    #[test]
    fn test_extract_all()
    {
        let tmpdir = assert_memfs_setup!();
        let file1 = tmpdir.mash("file1");

        assert!(vfs::append_all(&file1, "Not my favorite movie: 'Citizen Kane' (1941)").is_ok());
        assert!(vfs::append_all(&file1, "\n").is_ok());
        assert!(vfs::append_all(&file1, "Another not great movie: 'Zoolander' (2001)").is_ok());
        assert_eq!(file::extract_all(&file1, r"'[^']+'\s+\(\d{4}\)").unwrap(), vec![
            "'Citizen Kane' (1941)",
            "'Zoolander' (2001)"
        ]);
        assert_remove_all!(&tmpdir);
    }
}
