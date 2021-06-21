rustc_session::declare_lint! {
    /// **What it does:**
    ///
    /// **Why is this bad?**
    ///
    /// **Known problems:** None.
    ///
    /// **Example:**
    ///
    /// ```rust
    /// // example code where clippy issues a warning
    /// ```
    /// Use instead:
    /// ```rust
    /// // example code which does not raise clippy warning
    /// ```
    pub MY_LINT_FUNCTION_NAME_IS_FEATURE,
    Warn,
    "my_lint_function_name_is_feature"
}

rustc_session::declare_lint_pass!(MyLintFunctionNameIsFeature => [MY_LINT_FUNCTION_NAME_IS_FEATURE]);

impl rustc_lint::EarlyLintPass for MyLintFunctionNameIsFeature {
    fn check_fn(
        &mut self,
        cx: &rustc_lint::EarlyContext<'_>,
        fn_kind: rustc_ast::visit::FnKind<'_>,
        span: rustc_span::Span,
        _: rustc_ast::NodeId,
    ) {
        // Ignore FnKind::Closure
        if let rustc_ast::visit::FnKind::Fn(_, ident, ..) = fn_kind {
            if ident.as_str() == "feature" {
                clippy_utils::diagnostics::span_lint(
                    cx,
                    MY_LINT_FUNCTION_NAME_IS_FEATURE,
                    span,
                    "fn feature() generate by vscode-ra is not allow to commit",
                );
            }
        }
    }
}
