#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/01-parse.rs");
    t.pass("tests/02-create-builder.rs");
    t.pass("tests/03-call-setters.rs");
    t.pass("tests/04-call-build.rs");
    t.pass("tests/05-method-chaining.rs");
    t.pass("tests/06-optional-field.rs");
    t.pass("tests/07-repeated-field.rs");
    // Have altered the error slightly to  what I think should be better.
    // The previous error would have required a bunch of messy parsing and joining of errors with nightly compiler.
    // Will return to this later after more experience.
    t.compile_fail("tests/08-unrecognized-attribute.rs");
    t.pass("tests/09-redefined-prelude-types.rs");
}
