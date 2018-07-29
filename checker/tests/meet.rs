//! Tests for meet.csv files.

extern crate checker;
extern crate csv;

use checker::check_meet::{self, do_check};
use checker::Report;

use std::path::PathBuf;

/// Executes checks against a string representation of a CSV,
/// returning the number of errors.
fn check(csv: &str) -> usize {
    let report = Report::new(PathBuf::from("[inline]"));
    let mut rdr = csv::ReaderBuilder::new()
        .quoting(false)
        .from_reader(csv.as_bytes());
    let (errors, _warnings) = do_check(&mut rdr, report).unwrap().count_messages();
    errors
}

/// Helper for calling check_meet::check_meetpath(). Returns number of errors.
fn check_meetpath(s: &str) -> usize {
    // Although the tests use the final MeetPath, the library code expects
    // the full path to the meet.csv, and derives the MeetPath from that.
    let mut path = PathBuf::from(s);
    path.push("meet.csv");

    let mut report = Report::new(path);
    check_meet::check_meetpath(&mut report);
    let (errors, _warnings) = report.count_messages();
    errors
}

#[test]
fn test_empty_file() {
    assert!(check("") > 0);
}

#[test]
fn test_bob3() {
    let data = "Federation,Date,MeetCountry,MeetState,MeetTown,MeetName\n
                WRPF,2016-08-19,USA,CA,Mountain View,Boss of Bosses 3";
    assert_eq!(check(data), 0);
}

#[test]
fn test_missing_headers() {
    // Missing Federation.
    let data = "Date,MeetCountry,MeetState,MeetTown,MeetName\n
                2016-08-19,USA,CA,Mountain View,Boss of Bosses 3";
    assert!(check(data) > 0);

    // Missing Date.
    let data = "Federation,MeetCountry,MeetState,MeetTown,MeetName\n
                WRPF,USA,CA,Mountain View,Boss of Bosses 3";
    assert!(check(data) > 0);

    // Missing MeetCountry.
    let data = "Federation,Date,MeetState,MeetTown,MeetName\n
                WRPF,2016-08-19,CA,Mountain View,Boss of Bosses 3";
    assert!(check(data) > 0);

    // Missing MeetState.
    let data = "Federation,Date,MeetCountry,MeetTown,MeetName\n
                WRPF,2016-08-19,USA,Mountain View,Boss of Bosses 3";
    assert!(check(data) > 0);

    // Missing MeetTown.
    let data = "Federation,Date,MeetCountry,MeetState,MeetName\n
                WRPF,2016-08-19,USA,CA,Boss of Bosses 3";
    assert!(check(data) > 0);

    // Missing MeetName.
    let data = "Federation,Date,MeetCountry,MeetState,MeetTown\n
                WRPF,2016-08-19,USA,CA,Mountain View";
    assert!(check(data) > 0);
}

#[test]
fn test_header_typos() {
    // Typo Federation.
    let data = "Fedaration,Date,MeetCountry,MeetState,MeetTown,MeetName\n
                WRPF,2016-08-19,USA,CA,Mountain View,Boss of Bosses 3";
    assert!(check(data) > 0);

    // Typo Date.
    let data = "Federation,Dote,MeetCountry,MeetState,MeetTown,MeetName\n
                WRPF,2016-08-19,USA,CA,Mountain View,Boss of Bosses 3";
    assert!(check(data) > 0);

    // Typo MeetCountry.
    let data = "Federation,Date,MeatCountry,MeetState,MeetTown,MeetName\n
                WRPF,2016-08-19,USA,CA,Mountain View,Boss of Bosses 3";
    assert!(check(data) > 0);

    // Typo MeetState.
    let data = "Federation,Date,MeetCountry,MeatState,MeetTown,MeetName\n
                WRPF,2016-08-19,USA,CA,Mountain View,Boss of Bosses 3";
    assert!(check(data) > 0);

    // Typo MeetTown.
    let data = "Federation,Date,MeetCountry,MeetState,MeatTown,MeetName\n
                WRPF,2016-08-19,USA,CA,Mountain View,Boss of Bosses 3";
    assert!(check(data) > 0);

    // Typo MeetName.
    let data = "Federation,Date,MeetCountry,MeetState,MeatTown,MeatName\n
                WRPF,2016-08-19,USA,CA,Mountain View,Boss of Bosses 3";
    assert!(check(data) > 0);
}

/// Test that headers have not been reordered.
///
/// Although entries.csv allows reordering, meet.csv does not.
#[test]
fn test_reordered_headers() {
    let data = "Federation,Date,MeetState,MeetCountry,MeetTown,MeetName\n
                WRPF,2016-08-19,CA,USA,Mountain View,Boss of Bosses 3";
    assert!(check(data) > 0);
}

#[test]
fn test_meetpath_successes() {
    assert_eq!(check_meetpath("/wrpf/bob3"), 0);
    assert_eq!(check_meetpath("/uspa/0302"), 0);
    assert_eq!(check_meetpath("/cpu/2013-11-02-81b29779"), 0);
}

#[test]
fn test_meetpath_failures() {
    // Underscore is disallowed: use '-' instead.
    assert_eq!(check_meetpath("/dsf/welt_kampf"), 1);

    // Non-alphanemuric ASCII is disallowed.
    assert_eq!(check_meetpath("/dsf/welt:kampf"), 1);
    assert_eq!(check_meetpath("/dsf/welt\"kampf"), 1);

    // Spacing is disallowed.
    assert_eq!(check_meetpath("/dsf/welt kampf"), 1);
    assert_eq!(check_meetpath("/dsf/weltkampf "), 1);

    // Non-ASCII UTF-8 is disallowed.
    assert_eq!(check_meetpath("/wrpf/белкинасила"), 1);
}
