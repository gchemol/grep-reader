// [[file:../grep-reader.note::3da52855][3da52855]]
use anyhow::*;
use grep_reader::GrepReader;

#[test]
fn test_grep() -> Result<()> {
    let path = "./tests/files/multi.xyz";
    let mut reader = GrepReader::try_from_path(path.as_ref())?;
    let n = reader.mark(r"^\s*\d+\s*$", 2)?;
    assert_eq!(n, 2);
    let n = reader.mark(r"^\s*\d+\s*$", None)?;
    assert_eq!(n, 6);

    let _ = reader.goto_next_marker()?;
    let _ = reader.goto_next_marker()?;
    let mut s = String::new();
    let _ = reader.read_lines(1, &mut s)?;
    assert_eq!(s.trim(), "10");

    // goto the marker directly
    let _ = reader.goto_marker(4)?;
    s.clear();
    reader.read_lines(1, &mut s)?;
    assert_eq!(s.trim(), "16");
    let _ = reader.goto_next_marker()?;
    s.clear();
    reader.read_lines(1, &mut s)?;
    assert_eq!(s.trim(), "13");

    Ok(())
}

#[test]
fn test_grep_read_until() -> Result<()> {
    let path = "./tests/files/multi.xyz";
    // read until next marker
    let mut reader = GrepReader::try_from_path(path.as_ref())?;
    let n = reader.mark(r"^ Configuration number :", None)?;
    assert_eq!(n, 6);
    let mut s = String::new();
    reader.read_until_next_marker(&mut s)?;
    assert!(s.ends_with("          16\r\n"));
    s.clear();
    reader.goto_next_marker()?;
    reader.read_until_next_marker(&mut s)?;
    assert!(s.starts_with(" Configuration number :       14"));
    assert!(s.ends_with("          16\r\n"));
    assert_eq!(reader.current_marker(), 3);
    reader.goto_marker(5)?;
    s.clear();
    reader.read_until_next_marker(&mut s)?;
    assert!(s.starts_with(" Configuration number :       42"));
    assert!(s.ends_with("0.97637  -1.60620\r\n"));

    Ok(())
}
// 3da52855 ends here
