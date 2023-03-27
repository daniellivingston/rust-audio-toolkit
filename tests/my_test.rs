use rta::notes;

#[test]
fn test_notes() {
    let n = notes();
    assert!(n.contains_key("C"));
}
