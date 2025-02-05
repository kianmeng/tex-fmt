use crate::colors::*;
use crate::format_file;
use crate::fs;
use crate::logging::*;
use crate::Cli;
use similar::{ChangeTag, TextDiff};

fn test_file(source_file: &str, target_file: &str) -> bool {
    let args = Cli::new();
    let mut logs = Vec::<Log>::new();
    let source_text = fs::read_to_string(source_file).unwrap();
    let target_text = fs::read_to_string(target_file).unwrap();
    let fmt_source_text =
        format_file(&source_text, source_file, &args, &mut logs);

    if fmt_source_text != target_text {
        println!(
            "{}fail {}{} {}-> {}{}{}",
            RED, YELLOW, source_file, RESET, YELLOW, target_file, RESET
        );
        let diff = TextDiff::from_lines(&fmt_source_text, &target_text);
        for change in diff.iter_all_changes() {
            match change.tag() {
                ChangeTag::Delete => print!(
                    "{}@ {}: {}- {}{}",
                    PURPLE,
                    change.old_index().unwrap(),
                    RED,
                    change,
                    RESET
                ),
                ChangeTag::Insert => print!(
                    "{}@ {}: {}+ {}{}",
                    PURPLE,
                    change.new_index().unwrap(),
                    GREEN,
                    change,
                    RESET
                ),
                ChangeTag::Equal => {}
            };
        }
    }

    fmt_source_text == target_text
}

fn read_files_from_dir(dir: &str) -> Vec<String> {
    fs::read_dir(dir)
        .unwrap()
        .map(|f| f.unwrap().file_name().into_string().unwrap())
        .collect()
}

#[test]
fn test_source() {
    let source_files = read_files_from_dir("./tests/source/");
    let mut fail = false;
    for file in source_files {
        if !test_file(
            &format!("tests/source/{}", file),
            &format!("tests/target/{}", file),
        ) {
            fail = true
        }
    }
    if fail {
        panic!("Some tests failed")
    }
}

#[test]
fn test_target() {
    let target_files = read_files_from_dir("./tests/target/");
    let mut fail = false;
    for file in target_files {
        if !test_file(
            &format!("tests/target/{}", file),
            &format!("tests/target/{}", file),
        ) {
            fail = true
        }
    }
    if fail {
        panic!("Some tests failed")
    }
}

#[test]
#[ignore]
fn test_short() {
    let files = vec![
        //"brackets.tex",
        //"cam-thesis.cls",
        //"comments.tex",
        //"cv.tex",
        //"document.tex",
        //"environment_lines.tex",
        //"heavy_wrap.tex",
        //"higher_categories_thesis.bib",
        //"higher_categories_thesis.tex",
        //"ignore.tex",
        //"lists.tex",
        //"masters_dissertation.tex",
        //"ociamthesis.cls",
        //"phd_dissertation.tex",
        //"phd_dissertation_refs.bib",
        //"puthesis.cls",
        //"quiver.sty",
        //"readme.tex",
        "short_document.tex",
        //"tikz_network.sty",
        //"unicode.tex",
        //"verbatim.tex",
        //"wgu-cv.cls",
        //"wrap.tex",
    ];
    let mut fail = false;
    for file in files {
        if !test_file(
            &format!("tests/source/{}", file),
            &format!("tests/target/{}", file),
        ) {
            fail = true
        }
    }
    if fail {
        panic!("Some tests failed")
    }
}
