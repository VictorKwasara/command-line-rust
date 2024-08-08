use assert_cmd::Command;

#[test]
fn works() {
    assert!(true);
}

// #[test]
// fn runs() {
//     let mut cmd = Command::new("solana");
//     let res = cmd.output();
//     println!("What am i {:?}", res);
//     assert!(res.is_ok());
// }

// #[test]
// fn runs_hello() {
//     let mut cmd = Command::new("hello");
//     let res = cmd.output();
//     assert!(res.is_ok());
// }

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("exps_one").unwrap();
    cmd.assert().success().stdout("Hello, world!\n");
}

#[test]
fn true_ok() {
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok() {
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}

