use assert_cmd::Command;
use predicates::prelude::*;
use std::io::Write;
use tempfile::{Builder, NamedTempFile};

fn create_temp_file(content: &str) -> NamedTempFile {
    let mut temp_file = Builder::new().tempfile_in("./").unwrap();
    // Write known content into the file.

    write!(temp_file, "{}", content).unwrap();

    return temp_file;
}

#[test]
fn test_file_non_existent() {
    let mut cmd = Command::cargo_bin("cwc").unwrap();
    let filename = "non_existent_file.txt";

    cmd.arg(filename)
        .assert()
        .code(1)
        .stderr(predicate::str::contains(format!(
            "cwc: {}: open: No such file",
            filename
        )));
}

#[test]
fn test_count_bytes_one_file() {
    let file_content = "Hello, world!";
    let mut cmd = Command::cargo_bin("cwc").unwrap();
    let file = create_temp_file(file_content);
    let file_path = file.path().to_str().unwrap();

    cmd.arg("-c")
        .arg(file_path)
        .assert()
        .code(0)
        .stdout(predicate::str::contains(&format!(
            "{:>7} {}",
            13, file_path
        )));
}

#[test]
fn test_count_bytes_two_files() {
    let file_content1 = "Hello, world!";
    let file1 = create_temp_file(file_content1);
    let file_path1 = file1.path().to_str().unwrap();

    let file_content2 = "I have a dream!
    I have a dream that one day on the red hills of Georgia,
    the sons of former slaves and the sons of former slave owners
    will be able to sit down together at the table of brotherhood.";
    let file2 = create_temp_file(file_content2);
    let file_path2 = file2.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("cwc").unwrap();

    let stdout_predicate = predicate::str::contains(&format!("{:>7} {}", 13, file_path1)).and(
        predicate::str::contains(&format!("{:>7} {}", 209, file_path2))
            .and(predicate::str::contains(&format!("{:>7} {}", 222, "total"))),
    );
    cmd.arg("-c")
        .args(&[file_path1, file_path2])
        .assert()
        .code(0)
        .stdout(stdout_predicate);
}

#[test]
fn test_count_lines_one_file() {
    let file_content = "Hello,
    world!";
    let mut cmd = Command::cargo_bin("cwc").unwrap();
    let file = create_temp_file(file_content);
    let file_path = file.path().to_str().unwrap();

    cmd.arg("-l")
        .arg(file_path)
        .assert()
        .code(0)
        .stdout(predicate::str::contains(&format!("{:>7} {}", 1, file_path)));
}

#[test]
fn test_count_lines_two_files() {
    let file_content1 = "Hello, world!";
    let file1 = create_temp_file(file_content1);
    let file_path1 = file1.path().to_str().unwrap();

    let file_content2 = "I have a dream!
    I have a dream that one day on the red hills of Georgia,
    the sons of former slaves and the sons of former slave owners
    will be able to sit down together at the table of brotherhood.";
    let file2 = create_temp_file(file_content2);
    let file_path2 = file2.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("cwc").unwrap();

    let stdout_predicate = predicate::str::contains(&format!("{:>7} {}", 0, file_path1)).and(
        predicate::str::contains(&format!("{:>7} {}", 3, file_path2))
            .and(predicate::str::contains(&format!("{:>7} {}", 3, "total"))),
    );
    cmd.arg("-l")
        .args(&[file_path1, file_path2])
        .assert()
        .code(0)
        .stdout(stdout_predicate);
}

#[test]
fn test_count_words_one_file() {
    let file_content = "Hello,
    world!";
    let mut cmd = Command::cargo_bin("cwc").unwrap();
    let file = create_temp_file(file_content);
    let file_path = file.path().to_str().unwrap();

    cmd.arg("-w")
        .arg(file_path)
        .assert()
        .code(0)
        .stdout(predicate::str::contains(&format!("{:>7} {}", 2, file_path)));
}

#[test]
fn test_count_words_two_files() {
    let file_content1 = "Hello, world!";
    let file1 = create_temp_file(file_content1);
    let file_path1 = file1.path().to_str().unwrap();

    let file_content2 = "I have a dream!
    I have a dream that one day on the red hills of Georgia,
    the sons of former slaves and the sons of former slave owners
    will be able to sit down together at the table of brotherhood.";
    let file2 = create_temp_file(file_content2);
    let file_path2 = file2.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("cwc").unwrap();

    let stdout_predicate = predicate::str::contains(&format!("{:>7} {}", 2, file_path1)).and(
        predicate::str::contains(&format!("{:>7} {}", 41, file_path2))
            .and(predicate::str::contains(&format!("{:>7} {}", 43, "total"))),
    );
    cmd.arg("-w")
        .args(&[file_path1, file_path2])
        .assert()
        .code(0)
        .stdout(stdout_predicate);
}

#[test]
fn test_count_chars_one_file() {
    let file_content = "Comment ça va éléonore ?";
    let mut cmd = Command::cargo_bin("cwc").unwrap();
    let file = create_temp_file(file_content);
    let file_path = file.path().to_str().unwrap();

    cmd.arg("-m")
        .arg(file_path)
        .assert()
        .code(0)
        .stdout(predicate::str::contains(&format!(
            "{:>7} {}",
            24, file_path
        )));
}

#[test]
fn test_count_chars_two_files() {
    let file_content1 = "Comment ça va éléonore ?";
    let file1 = create_temp_file(file_content1);
    let file_path1 = file1.path().to_str().unwrap();

    let file_content2 = "I have a dream!
    I have a dream that one day on the red hills of Georgia,
    the sons of former slaves and the sons of former slave owners
    will be able to sit down together at the table of brotherhood.";
    let file2 = create_temp_file(file_content2);
    let file_path2 = file2.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("cwc").unwrap();

    let stdout_predicate = predicate::str::contains(&format!("{:>7} {}", 24, file_path1)).and(
        predicate::str::contains(&format!("{:>7} {}", 209, file_path2))
            .and(predicate::str::contains(&format!("{:>7} {}", 233, "total"))),
    );
    cmd.arg("-m")
        .args(&[file_path1, file_path2])
        .assert()
        .code(0)
        .stdout(stdout_predicate);
}

#[test]
fn test_no_flag_one_file() {
    let file_content = "Hello, world!";
    let mut cmd = Command::cargo_bin("cwc").unwrap();
    let file = create_temp_file(file_content);
    let file_path = file.path().to_str().unwrap();

    cmd.arg(file_path)
        .assert()
        .code(0)
        .stdout(predicate::str::contains(&format!(
            "{:>7} {:>7} {:>7} {}",
            0, 2, 13, file_path
        )));
}

#[test]
fn test_no_flag_two_files() {
    let file_content1 = "Hello, world!";
    let file1 = create_temp_file(file_content1);
    let file_path1 = file1.path().to_str().unwrap();

    let file_content2 = "I have a dream!
    I have a dream that one day on the red hills of Georgia,
    the sons of former slaves and the sons of former slave owners
    will be able to sit down together at the table of brotherhood.";
    let file2 = create_temp_file(file_content2);
    let file_path2 = file2.path().to_str().unwrap();

    let mut cmd = Command::cargo_bin("cwc").unwrap();

    let stdout_predicate =
        predicate::str::contains(&format!("{:>7} {:>7} {:>7} {}", 0, 2, 13, file_path1)).and(
            predicate::str::contains(&format!("{:>7} {:>7} {:>7} {}", 3, 41, 209, file_path2)).and(
                predicate::str::contains(&format!("{:>7} {:>7} {:>7} {}", 3, 43, 222, "total")),
            ),
        );
    cmd.args(&[file_path1, file_path2])
        .assert()
        .code(0)
        .stdout(stdout_predicate);
}

#[test]
fn test_read_from_stdin() {
    let input = "Hello, world!";
    let mut cmd = Command::cargo_bin("cwc").unwrap();

    cmd.write_stdin(input)
        .assert()
        .code(0)
        .stdout(predicate::str::contains(&format!(
            "{:>7} {:>7} {:>7}",
            0, 2, 13
        )));
}
