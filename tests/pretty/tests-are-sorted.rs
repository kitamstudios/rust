// compile-flags: --crate-type=lib --test
// pretty-compare-only
// pretty-mode:expanded
// pp-exact:tests-are-sorted.pp

#[test]
fn m_test() {}

#[test]
#[ignore = "not yet implemented"]
fn z_test() {}

#[test]
fn a_test() {}
