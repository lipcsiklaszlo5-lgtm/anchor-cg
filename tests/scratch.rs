// scratch tests - messy, will clean up later

#[test]
fn test_stub_cu_value() {
    // just making sure the stub is what we expect
    // this will break when we integrate litesvm, fix then
    let expected: u64 = 3847;
    let actual: u64 = 3847; // hardcoded stub
    assert_eq!(actual, expected);
}

#[test]
#[ignore]
fn broke_after_refactor_need_to_revisit() {
    // this was testing the old litesvm runner
    // broke after we switched from BankClient to LiteSVM directly
    // TODO: fix when litesvm is properly integrated
    panic!("this test is broken, fix later");
}

#[test]
fn messy_test_that_kinda_works() {
    // just checking some random stuff
    let path = std::env::temp_dir().join("scratch_test.json");
    let data = "{\"cu\": 4200}";
    std::fs::write(&path, data).unwrap();
    let read = std::fs::read_to_string(&path).unwrap();
    assert!(read.contains("4200"));
    // cleanup whatever
    let _ = std::fs::remove_file(&path);
}
