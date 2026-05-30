use assert_cmd::Command;
use predicates::{boolean::PredicateBooleanExt, str::contains};
use tempfile::tempdir;

#[test]
fn add_a_task() {
    let temp_dir = tempdir().unwrap();
    let data_path = temp_dir.path().join("todos.bin");

    Command::cargo_bin("prsq")
        .unwrap()
        .args(["add", "Buy Groceries"])
        .env("PRSQ_DATA", &data_path)
        .assert()
        .success()
        .stdout(contains("Buy Groceries"));
}

#[test]
fn list_tasks() {
    let temp_dir = tempdir().unwrap();
    let data_path = temp_dir.path().join("todos.bin");

    Command::cargo_bin("prsq")
        .unwrap()
        .args(["add", "Buy Groceries"])
        .env("PRSQ_DATA", &data_path)
        .assert()
        .success();

    Command::cargo_bin("prsq")
        .unwrap()
        .args(["add", "Buy Beans"])
        .env("PRSQ_DATA", &data_path)
        .assert()
        .success();

    Command::cargo_bin("prsq")
        .unwrap()
        .arg("list")
        .env("PRSQ_DATA", &data_path)
        .assert()
        .success()
        .stdout(contains("Buy Groceries").and(contains("Buy Beans")));
}

#[test]
fn remove_tasks() {
    let temp_dir = tempdir().unwrap();
    let data_path = temp_dir.path().join("todos.bin");

    Command::cargo_bin("prsq")
        .unwrap()
        .args(["add", "Buy Groceries"])
        .env("PRSQ_DATA", &data_path)
        .assert()
        .success();

    Command::cargo_bin("prsq")
        .unwrap()
        .arg("done")
        .env("PRSQ_DATA", &data_path)
        .assert()
        .success()
        .stdout(contains("task: Buy Groceries has been completed"));
}
