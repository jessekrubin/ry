"""Tests based on the examples in the regex documentation

REF: https://docs.rs/regex/latest/regex/#examples
"""
from ry import dev as ry
import pytest

def test_regex_example_find_a_middle_initial() -> None:
    r"""
    use regex::Regex;

    // We use 'unwrap()' here because it would be a bug in our program if the
    // pattern failed to compile to a regex. Panicking in the presence of a bug
    // is okay.
    let re = Regex::new(r"Homer (.)\. Simpson").unwrap();
    let hay = "Homer J. Simpson";
    let Some(caps) = re.captures(hay) else { return };
    assert_eq!("J", &caps[1]);
    """

    re = ry.Regex(r"Homer (.)\. Simpson")
    hay = "Homer J. Simpson"
    caps = re.captures(hay)
    assert caps[1] == "J"
    print(caps, caps[1])
    assert False



def test_regex_example_validate_date_format() -> None:
    r"""
    use regex::Regex;

    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    assert!(re.is_match("2010-03-14"));
    """

    re = ry.Regex(r"^\d{4}-\d{2}-\d{2}$")
    assert re.is_match("2010-03-14")


def test_regex_example_validate_date_format_unicode() -> None:
    r"""
    use regex::Regex;

    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    assert!(re.is_match("𝟚𝟘𝟙𝟘-𝟘𝟛-𝟙𝟜"));
    """

    re = ry.Regex(r"^\d{4}-\d{2}-\d{2}$")
    assert re.is_match("𝟚𝟘𝟙𝟘-𝟘𝟛-𝟙𝟜")



def test_regex_example_find_dates_in_haystack() -> None:
    r"""
    use regex::Regex;

    let re = Regex::new(r"[0-9]{4}-[0-9]{2}-[0-9]{2}").unwrap();
    let hay = "What do 1865-04-14, 1881-07-02, 1901-09-06 and 1963-11-22 have in common?";
    // 'm' is a 'Match', and 'as_str()' returns the matching part of the haystack.
    let dates: Vec<&str> = re.find_iter(hay).map(|m| m.as_str()).collect();
    assert_eq!(dates, vec![
        "1865-04-14",
        "1881-07-02",
        "1901-09-06",
        "1963-11-22",
    ]);
    """
    re = ry.Regex(r"[0-9]{4}-[0-9]{2}-[0-9]{2}")
    hay = "What do 1865-04-14, 1881-07-02, 1901-09-06 and 1963-11-22 have in common?"
    dates_iterable = re.find_iter(hay)

    dates = [m.as_str() for m in dates_iterable]
    assert dates == ["1865-04-14", "1881-07-02", "1901-09-06", "1963-11-22"]


def test_regex_example_find_dates_in_haystack_with_captures() -> None:
    r"""
    use regex::Regex;

    let re = Regex::new(r"(?<y>[0-9]{4})-(?<m>[0-9]{2})-(?<d>[0-9]{2})").unwrap();
    let hay = "What do 1865-04-14, 1881-07-02, 1901-09-06 and 1963-11-22 have in common?";
    // 'm' is a 'Match', and 'as_str()' returns the matching part of the haystack.
    let dates: Vec<(&str, &str, &str)> = re.captures_iter(hay).map(|caps| {
        // The unwraps are okay because every capture group must match if the whole
        // regex matches, and in this context, we know we have a match.
        //
        // Note that we use `caps.name("y").unwrap().as_str()` instead of
        // `&caps["y"]` because the lifetime of the former is the same as the
        // lifetime of `hay` above, but the lifetime of the latter is tied to the
        // lifetime of `caps` due to how the `Index` trait is defined.
        let year = caps.name("y").unwrap().as_str();
        let month = caps.name("m").unwrap().as_str();
        let day = caps.name("d").unwrap().as_str();
        (year, month, day)
    }).collect();
    assert_eq!(dates, vec![
        ("1865", "04", "14"),
        ("1881", "07", "02"),
        ("1901", "09", "06"),
        ("1963", "11", "22"),
    ]);
    """

    re = ry.Regex(r"(?<y>[0-9]{4})-(?<m>[0-9]{2})-(?<d>[0-9]{2})")
    hay = "What do 1865-04-14, 1881-07-02, 1901-09-06 and 1963-11-22 have in common?"
    date_captures_iterable = re.captures(hay)
    print(date_captures_iterable)


    dates = [(caps["y"], caps["m"], caps["d"]) for caps in date_captures_iterable]
    assert dates == [("1865", "04", "14"), ("1881", "07", "02"), ("1901", "09", "06"), ("1963", "11", "22")]





def test_regex_example_find_dates_in_haystack_with_captures_extract() -> None:
    r"""
    use regex::Regex;

    let re = Regex::new(r"([0-9]{4})-([0-9]{2})-([0-9]{2})").unwrap();
    let hay = "What do 1865-04-14, 1881-07-02, 1901-09-06 and 1963-11-22 have in common?";
    let dates: Vec<(&str, &str, &str)> = re.captures_iter(hay).map(|caps| {
        let (_, [year, month, day]) = caps.extract();
        (year, month, day)
    }).collect();
    assert_eq!(dates, vec![
        ("1865", "04", "14"),
        ("1881", "07", "02"),
        ("1901", "09", "06"),
        ("1963", "11", "22"),
    ]);
    """
    pytest.skip("Not implemented")


def test_regex_example_replace_with_named_capture_groups() -> None:
    r"""
    use regex::Regex;

    let re = Regex::new(r"(?<y>\d{4})-(?<m>\d{2})-(?<d>\d{2})").unwrap();
    let before = "1973-01-05, 1975-08-25 and 1980-10-18";
    let after = re.replace_all(before, "$m/$d/$y");
    assert_eq!(after, "01/05/1973, 08/25/1975 and 10/18/1980");
    """
    pytest.skip("Not implemented")

def test_regex_example_verbose_mode() -> None:
    r"""
    use regex::Regex;

    let re = Regex::new(r"(?x)
      (?P<y>\d{4}) # the year, including all Unicode digits
      -
      (?P<m>\d{2}) # the month, including all Unicode digits
      -
      (?P<d>\d{2}) # the day, including all Unicode digits
    ").unwrap();

    let before = "1973-01-05, 1975-08-25 and 1980-10-18";
    let after = re.replace_all(before, "$m/$d/$y");
    assert_eq!(after, "01/05/1973, 08/25/1975 and 10/18/1980");
    """
    pytest.skip("Not implemented")


def test_regex_example_match_multiple_regexes() -> None:
    r"""
    use regex::RegexSet;

    let set = RegexSet::new(&[
        r"\w+",
        r"\d+",
        r"\pL+",
        r"foo",
        r"bar",
        r"barfoo",
        r"foobar",
    ]).unwrap();

    // Iterate over and collect all of the matches. Each match corresponds to the
    // ID of the matching pattern.
    let matches: Vec<_> = set.matches("foobar").into_iter().collect();
    assert_eq!(matches, vec![0, 2, 3, 4, 6]);

    // You can also test whether a particular regex matched:
    let matches = set.matches("foobar");
    assert!(!matches.matched(5));
    assert!(matches.matched(6));
    """
    pytest.skip("Not implemented")


def test_regex_example_match_multiple_regular_expressions_simultaneously() -> None:
    r"""
    use regex::RegexSet;

    let set = RegexSet::new(&[
        r"\w+",
        r"\d+",
        r"\pL+",
        r"foo",
        r"bar",
        r"barfoo",
        r"foobar",
    ]).unwrap();

    // Iterate over and collect all of the matches. Each match corresponds to the
    // ID of the matching pattern.
    let matches: Vec<_> = set.matches("foobar").into_iter().collect();
    assert_eq!(matches, vec![0, 2, 3, 4, 6]);

    // You can also test whether a particular regex matched:
    let matches = set.matches("foobar");
    assert!(!matches.matched(5));
    assert!(matches.matched(6));
    """
    pytest.skip("Not implemented")


if __name__ == "__main__":

    test_regex_example_find_dates_in_haystack()
