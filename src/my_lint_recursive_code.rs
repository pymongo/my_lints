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
    pub MY_LINT_RECURSIVE_CODE,
    Warn,
    "my_lint_recursive_code"
}

rustc_session::declare_lint_pass!(MyLintRecursiveCode => [MY_LINT_RECURSIVE_CODE]);

/**
## TODO
检测循环调用导致无限递归的问题
Reference: https://github.com/rust-lang/rust/pull/75067/files
```text
fn cycle1() { //~ WARN function cannot return without recursing
    cycle2();
}

fn cycle2() { //~ WARN function cannot return without recursing
    cycle1();
}
```

## Input:
```text
#[allow(dead_code)]
fn fib(n: i32) -> i32 {
    if n == 0 || n == 1 {
        return 1;
    }
    fib(n-1) + fib(n-2)
}
```

## func_expr:
```text
// ...
[src/my_lint_recursive_code.rs:27] func_expr = Expr {
    hir_id: HirId {
        owner: DefId(0:3 ~ test_recursive_lint[89fc]::fib),
        local_id: 27,
    },
    kind: Path(
        Resolved(
            None,
            Path {
                span: examples/test_recursive_lint.rs:6:16: 6:19 (#0),
                res: Def(
                    Fn,
                    DefId(0:3 ~ test_recursive_lint[89fc]::fib),
                ),
                segments: [
                    PathSegment {
                        ident: fib#0,
                        hir_id: Some(
                            HirId {
                                owner: DefId(0:3 ~ test_recursive_lint[89fc]::fib),
                                local_id: 26,
                            },
                        ),
                        res: Some(
                            Err,
                        ),
                        args: None,
                        infer_args: true,
                    },
                ],
            },
        ),
    ),
    span: examples/test_recursive_lint.rs:6:16: 6:19 (#0),
}
```
*/
impl rustc_lint::LateLintPass<'_> for MyLintRecursiveCode {
    fn check_expr_post(&mut self, cx: &rustc_lint::LateContext<'_>, expr: &rustc_hir::Expr<'_>) {
        if let rustc_hir::ExprKind::Call(func_expr, _) = &expr.kind {
            // 函数调用表达式 func_expr 的前一个函数调用**栈帧**函数的 defid
            let func_expr_owner_defid = func_expr.hir_id.owner.to_def_id();
            if let rustc_hir::ExprKind::Path(rustc_hir::QPath::Resolved(_, path)) = &func_expr.kind
            {
                // path.res: The resolution of a path or export
                if let Some(func_expr_call_defid) = path.res.opt_def_id() {
                    if func_expr_owner_defid == func_expr_call_defid {
                        clippy_utils::diagnostics::span_lint(
                            cx,
                            MY_LINT_RECURSIVE_CODE,
                            expr.span,
                            "our company forbid recursive code, Reference: company_code_style.pdf, page 17",
                        );
                    }
                }
            }
        }
    }
}
