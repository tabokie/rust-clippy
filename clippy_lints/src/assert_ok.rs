use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::macros::{find_assert_args, root_macro_call_first_node, PanicExpn};
use clippy_utils::source::snippet_opt;
use if_chain::if_chain;
use rustc_errors::Applicability;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint_pass, declare_tool_lint};
use rustc_span::sym;

declare_clippy_lint! {
    /// ### What it does
    /// Checks for `assert!(r.is_ok())` calls.
    ///
    /// ### Why is this bad?
    /// An assertion failure cannot output a useful message of the error.
    ///
    /// ### Known problems
    /// The error type needs to implement `Debug`.
    ///
    /// ### Example
    /// ```rust,ignore
    /// # let r = Ok::<_, ()>(());
    /// assert!(r.is_ok());
    /// ```
    #[clippy::version = "1.64.0"]
    pub ASSERT_OK,
    style,
    "`assert!(r.is_ok())` gives worse error message than directly calling `r.unwrap()`"
}

declare_lint_pass!(AssertOk => [ASSERT_OK]);

impl<'tcx> LateLintPass<'tcx> for AssertOk {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, e: &'tcx Expr<'_>) {
        if_chain! {
            if let Some(macro_call) = root_macro_call_first_node(cx, e);
            if matches!(cx.tcx.get_diagnostic_name(macro_call.def_id), Some(sym::assert_macro));
            if let Some((condition, panic_expn)) = find_assert_args(cx, e, macro_call.expn);
            if matches!(panic_expn, PanicExpn::Empty);
            if let ExprKind::MethodCall(method_segment, args, _) = condition.kind;
            if method_segment.ident.name == sym!(is_ok);
            let method_receiver = &args[0];
            if let Some(method_receiver_snippet) = snippet_opt(cx, method_receiver.span);
            then {
                span_lint_and_sugg(
                    cx,
                    ASSERT_OK,
                    macro_call.span,
                    &format!(
                        "`assert!({}.is_ok())` gives bad error message",
                        method_receiver_snippet
                    ),
                    "replace with",
                    format!(
                        "{}.unwrap()",
                        method_receiver_snippet
                    ),
                    Applicability::Unspecified,
                );
            }
        }
    }
}
