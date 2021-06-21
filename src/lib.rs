#![feature(rustc_private)]
extern crate rustc_ast;
extern crate rustc_hir;
extern crate rustc_lint;
extern crate rustc_session;
extern crate rustc_span;

dylint_linting::dylint_library!();

mod my_lint_function_name_is_feature;
mod my_lint_recursive_code;

#[no_mangle]
pub fn register_lints(_sess: &rustc_session::Session, lint_store: &mut rustc_lint::LintStore) {
    lint_store.register_lints(&[
        my_lint_function_name_is_feature::MY_LINT_FUNCTION_NAME_IS_FEATURE,
        my_lint_recursive_code::MY_LINT_RECURSIVE_CODE,
    ]);
    lint_store.register_early_pass(|| {
        Box::new(my_lint_function_name_is_feature::MyLintFunctionNameIsFeature)
    });
    lint_store.register_late_pass(|| Box::new(my_lint_recursive_code::MyLintRecursiveCode));
}

#[test]
fn ui_test() {
    dylint_testing::ui_test(
        env!("CARGO_PKG_NAME"),
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("ui_tests"),
    );
}
