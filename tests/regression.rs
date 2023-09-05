#[test]
fn atmega328p() {
    let mut atdf = std::fs::File::open("tests/atmega328p.atdf").unwrap();
    let svd = atdf2svd::run_test(&mut atdf, vec![]);
    insta::assert_display_snapshot!(svd);
}

#[test]
fn atmega128rfa1() {
    let mut atdf = std::fs::File::open("tests/atmega128rfa1.atdf").unwrap();
    let svd = atdf2svd::run_test(&mut atdf, vec![]);
    insta::assert_display_snapshot!(svd);
}

#[test]
fn attiny817() {
    let mut atdf = std::fs::File::open("tests/attiny817.atdf").unwrap();
    let svd = atdf2svd::run_test(&mut atdf, vec![]);
    insta::assert_display_snapshot!(svd);
}
