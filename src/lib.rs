//! `rivia-file` provides orchestration for the manipulation of file content
//!
//! ### Example
//! ```
//! use rivia_file::prelude::*;
//! ```
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
    extract_p(path, &Regex::new(rx.as_ref()).map_err(|_| VfsError::FailedToExtractString)?)
}

/// Returns the first captured string from the given regular expression
///
/// ### Examples
/// ```
/// use rivia_file::prelude::*;
///
/// assert!(vfs::set_memfs().is_ok());
/// let file1 = vfs::root().mash("file1");
/// let rx = Regex::new(r"'([^']+)'\s+\((\d{4})\)").unwrap();
/// assert!(vfs::write_all(&file1, "Not my favorite movie: 'Citizen Kane' (1941).").is_ok());
/// assert_eq!(file::extract_p(&file1, &rx).unwrap(), "Citizen Kane");
/// ```
pub fn extract_p<T: AsRef<Path>>(path: T, rx: &Regex) -> RvResult<String>
{
    let data = vfs::read_all(path)?;
    let caps = rx.captures(&data).ok_or(VfsError::FailedToExtractString)?;
    let value = caps.get(1).ok_or(VfsError::FailedToExtractString)?;
    Ok(value.as_str().to_string())
}

/// Returns all the captured strings from the given regular expression
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
    extract_all_p(path, &Regex::new(rx.as_ref()).map_err(|_| VfsError::FailedToExtractString)?)
}

/// Returns all the captured strings from the given regular expression
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
/// assert_eq!(file::extract_all_p(&file1, &rx).unwrap(), vec!["'Citizen Kane' (1941)", "'Zoolander' (2001)"]);
/// ```
pub fn extract_all_p<T: AsRef<Path>>(path: T, rx: &Regex) -> RvResult<Vec<String>>
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
