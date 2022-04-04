//! `rivia-file` provides orchestration for the manipulation of file content
//!
//! ### Example
//! ```
//! use rivia_file::prelude::*;
//! ```
mod edit;

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
    let data = vfs::read_all(path)?;
    let rx = &Regex::new(rx.as_ref()).map_err(|_| FileError::FailedToExtractString)?;
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
    let data = vfs::read_all(path)?;
    let mut values = vec![];
    let rx = &Regex::new(rx.as_ref()).map_err(|_| FileError::FailedToExtractString)?;
    for cap in rx.captures_iter(&data) {
        values.append(&mut cap.iter().filter_map(|x| x.map(|y| y.as_str().to_string())).collect::<Vec<String>>());
    }
    Ok(values)
}

/// Insert lines at the location determined by the regular expression and offset
///
/// * Handles path expansion and absolute path resolution
/// * Insert will be before the regex location match. Use offset=1 to insert after match
/// * Offset will be added to the resulting location: negative is allowed
/// * Given lines will have a newline appended to them during insertion
///
/// ### Examples
/// ```
/// use rivia_file::prelude::*;
///
/// assert!(vfs::set_memfs().is_ok());
/// let file = vfs::root().mash("file");
/// assert!(vfs::append_lines(&file, &["foo2"]).is_ok());
/// assert!(file::insert_lines(&file, &["foo1"], r"foo2", 0).is_ok());
/// assert_eq!(vfs::read_lines(&file).unwrap(), vec!["foo1".to_string(), "foo2".to_string()]);
/// ```
pub fn insert_lines<T: AsRef<Path>, U: AsRef<str>>(path: T, lines: &[U], rx: U, offset: isize) -> RvResult<()>
{
    // Match regex on file's lines for insert location
    let mut loc = -1;
    let mut f_lines = vfs::read_lines(&path)?;
    let rx = &Regex::new(rx.as_ref()).map_err(|_| FileError::FailedToExtractString)?;
    for (i, line) in f_lines.iter().enumerate() {
        if rx.is_match(line) {
            loc = i as isize;
            break;
        }
    }

    // Validate and adjust offset
    if loc == -1 || loc + offset < 0 {
        return Err(FileError::InsertLocationNotFound.into());
    }
    let mut i = (loc + offset) as usize;

    // Insert given lines
    for line in lines {
        f_lines.insert(i, line.as_ref().to_string());
        i += 1;
    }

    // Write out the modified file
    vfs::write_lines(&path, &f_lines)?;

    Ok(())
}

/// Replaces the match of the regex with the given value
///
/// Wraps the regex crate's replace and thus supports the same bash like variable exapansion in the
/// value string by capture name or capture number to compose the resulting value from parts of the
/// original match.
///
/// * Handles path expansion and absolute path resolution
/// * If no match is found then the file is not changed
///
/// ### Examples
///
/// Named capture groups in the values are supported with a bash like variable syntax. Curly braces
/// are optional unless needed to separate the variable name from text values.
/// ```
/// use rivia_file::prelude::*;
///
/// assert!(vfs::set_memfs().is_ok());
/// let file = vfs::root().mash("file");
/// assert_write_all!(&file, "Springsteen, Bruce");
/// assert!(file::replace_all(&file, r"(?P<last>[^,\s]+),\s+(?P<first>\S+)", "${first}_${last}").is_ok());
/// assert_read_all!(&file, "Bruce_Springsteen");
/// ```
///
/// Unnamed capture groups in the values are supported with a bash like variable syntax. Curly
/// braces are optional unless needed to separate the variable name from the text values.
/// ```
/// use rivia_file::prelude::*;
///
/// assert!(vfs::set_memfs().is_ok());
/// let file = vfs::root().mash("file");
/// assert_write_all!(&file, "Springsteen, Bruce");
/// assert!(file::replace_all(&file, r"([^,\s]+),\s+(\S+)", "${2}_${1}").is_ok());
/// assert_read_all!(&file, "Bruce_Springsteen");
/// ```
pub fn replace_all<T: AsRef<Path>, U: AsRef<str>>(path: T, rx: U, value: U) -> RvResult<()>
{
    let rx = &Regex::new(rx.as_ref()).map_err(|_| FileError::FailedToExtractString)?;
    let read_data = vfs::read_all(&path)?;
    let write_data = rx.replace_all(&read_data, value.as_ref()).to_string();
    if read_data != write_data {
        vfs::write_all(&path, write_data).unwrap();
    }
    Ok(())
}

/// Variation of replace with out bash like variable expansion
///
/// * Handles path expansion and absolute path resolution
/// * Can freely use $ in values without worry of expansion
///
/// ### Examples
/// ```
/// use rivia_file::prelude::*;
///
/// let tmpdir = assert_memfs_setup!();
/// let file = tmpdir.mash("file");
/// assert!(vfs::append_lines(&file, &["foo1", "foo2"]).is_ok());
/// assert!(file::replace_all_ne(&file, r"foo2", "$1").is_ok());
/// assert_eq!(vfs::read_lines(&file).unwrap(), vec!["foo1".to_string(), "$1".to_string(),
/// ]);
/// ```
pub fn replace_all_ne<T: AsRef<Path>, U: AsRef<str>>(path: T, rx: U, value: U) -> RvResult<()>
{
    let rx = &Regex::new(rx.as_ref()).map_err(|_| FileError::FailedToExtractString)?;
    let read_data = vfs::read_all(&path)?;
    let write_data = rx.replace_all(&read_data, regex::NoExpand(value.as_ref())).to_string();
    if read_data != write_data {
        vfs::write_all(&path, write_data).unwrap();
    }
    Ok(())
}

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

        assert!(vfs::append_lines(&file1, &[
            "Not my favorite movie: 'Citizen Kane' (1941)",
            "Another not great movie: 'Zoolander' (2001)"
        ])
        .is_ok());
        assert_eq!(file::extract_all(&file1, r"'[^']+'\s+\(\d{4}\)").unwrap(), vec![
            "'Citizen Kane' (1941)",
            "'Zoolander' (2001)"
        ]);
        assert_remove_all!(&tmpdir);
    }

    #[test]
    fn test_insert_lines_error_cases()
    {
        let tmpdir = assert_memfs_setup!();
        let dir = tmpdir.mash("dir");
        let file = dir.mash("file");

        // fail abs
        assert_eq!(
            file::insert_lines("", &["foo"], r"foo", 0).unwrap_err().to_string(),
            PathError::Empty.to_string()
        );

        // parent doesn't exist
        assert_eq!(
            file::insert_lines(&file, &["foo"], r"foo", 0).unwrap_err().to_string(),
            PathError::does_not_exist(&file).to_string()
        );

        // exists but is not a file
        assert_mkdir_p!(&dir);
        assert_eq!(
            file::insert_lines(&dir, &["foo"], r"foo", 0).unwrap_err().to_string(),
            PathError::is_not_file(&dir).to_string()
        );

        // file exists and regix is invalid
        assert_write_all!(&file, "foo");
        assert_eq!(
            file::insert_lines(&file, &["foo"], r"[", 0).unwrap_err().to_string(),
            FileError::FailedToExtractString.to_string()
        );

        // Offset out of range
        assert_eq!(
            file::insert_lines(&file, &["foo"], r"", -2).unwrap_err().to_string(),
            FileError::InsertLocationNotFound.to_string()
        );

        assert_remove_all!(&tmpdir);
    }

    #[test]
    fn test_insert_lines_multi_before()
    {
        let tmpdir = assert_memfs_setup!();
        let file = tmpdir.mash("file");
        assert!(vfs::append_lines(&file, &["foo3"]).is_ok());

        assert!(file::insert_lines(&file, &["foo1", "foo2"], r"foo3", 0).is_ok());
        assert_eq!(vfs::read_lines(&file).unwrap(), vec![
            "foo1".to_string(),
            "foo2".to_string(),
            "foo3".to_string(),
        ]);

        assert_remove_all!(&tmpdir);
    }

    #[test]
    fn test_insert_lines_multi_before_neg()
    {
        let tmpdir = assert_memfs_setup!();
        let file = tmpdir.mash("file");
        assert!(vfs::append_lines(&file, &["foo3", "foo4"]).is_ok());

        assert!(file::insert_lines(&file, &["foo1", "foo2"], r"foo4", -1).is_ok());
        assert_eq!(vfs::read_lines(&file).unwrap(), vec![
            "foo1".to_string(),
            "foo2".to_string(),
            "foo3".to_string(),
            "foo4".to_string(),
        ]);

        assert_remove_all!(&tmpdir);
    }

    #[test]
    fn test_insert_lines_multi_after()
    {
        let tmpdir = assert_memfs_setup!();
        let file = tmpdir.mash("file");
        assert!(vfs::append_lines(&file, &["foo3"]).is_ok());

        assert!(file::insert_lines(&file, &["foo1", "foo2"], r"foo3", 1).is_ok());
        assert_eq!(vfs::read_lines(&file).unwrap(), vec![
            "foo3".to_string(),
            "foo1".to_string(),
            "foo2".to_string(),
        ]);

        assert_remove_all!(&tmpdir);
    }

    #[test]
    fn test_insert_lines_single_offset()
    {
        let tmpdir = assert_memfs_setup!();
        let file = tmpdir.mash("file");

        // Seed the file
        assert!(vfs::append_lines(&file, &["foo3"]).is_ok());

        // Insert before with offset = 0
        assert!(file::insert_lines(&file, &["foo2"], r"foo3", 0).is_ok());
        assert_eq!(vfs::read_lines(&file).unwrap(), vec!["foo2".to_string(), "foo3".to_string(),]);

        // Insert after with offset = 1
        assert!(file::insert_lines(&file, &["foo4"], r"foo3", 1).is_ok());
        assert_eq!(vfs::read_lines(&file).unwrap(), vec![
            "foo2".to_string(),
            "foo3".to_string(),
            "foo4".to_string(),
        ]);

        // Insert before negative with offset = -1
        assert!(file::insert_lines(&file, &["foo1"], r"foo3", -1).is_ok());
        assert_eq!(vfs::read_lines(&file).unwrap(), vec![
            "foo1".to_string(),
            "foo2".to_string(),
            "foo3".to_string(),
            "foo4".to_string(),
        ]);

        assert_remove_all!(&tmpdir);
    }

    #[test]
    fn test_replace()
    {
        let tmpdir = assert_memfs_setup!();
        let file = tmpdir.mash("file");
        assert!(vfs::append_lines(&file, &["foo1", "foo2", "foo3"]).is_ok());

        assert!(file::replace_all(&file, r"foo2", "blah").is_ok());
        assert_eq!(vfs::read_lines(&file).unwrap(), vec![
            "foo1".to_string(),
            "blah".to_string(),
            "foo3".to_string(),
        ]);

        assert_remove_all!(&tmpdir);
    }
}
