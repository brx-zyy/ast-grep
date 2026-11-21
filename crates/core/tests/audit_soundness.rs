use ast_grep_core::language::Language;
use ast_grep_core::matcher::{Pattern, PatternBuilder, PatternError};
use ast_grep_core::pinned::PinnedNodeData;
use ast_grep_core::source::Edit;
use ast_grep_core::tree_sitter::{LanguageExt, StrDoc, TSLanguage};
use ast_grep_core::{AstGrep, Doc};

#[derive(Clone)]
struct TestLang;

impl Language for TestLang {
    fn kind_to_id(&self, kind: &str) -> u16 {
        let lang: TSLanguage = tree_sitter_typescript::LANGUAGE_TSX.into();
        lang.id_for_node_kind(kind, true)
    }

    fn field_to_id(&self, field: &str) -> Option<u16> {
        let lang: TSLanguage = tree_sitter_typescript::LANGUAGE_TSX.into();
        lang.field_id_for_name(field).map(|id| id.get())
    }

    fn build_pattern(&self, builder: &PatternBuilder) -> Result<Pattern, PatternError> {
        builder.build(|src| StrDoc::try_new(src, self.clone()))
    }
}

impl LanguageExt for TestLang {
    fn get_ts_language(&self) -> TSLanguage {
        tree_sitter_typescript::LANGUAGE_TSX.into()
    }
}

fn root(source: &str) -> AstGrep<StrDoc<TestLang>> {
    AstGrep::<StrDoc<TestLang>>::doc(StrDoc::try_new(source, TestLang).unwrap())
}

#[test]
fn safe_edit_can_break_string_utf8_invariant() {
    let mut root = root("ok");
    let result = root.edit(Edit::<String> {
        position: 0,
        deleted_length: 2,
        inserted_text: vec![0xff],
    });
    assert!(result.is_ok(), "safe edit unexpectedly rejected invalid UTF-8");
    // Do not inspect the resulting String: it is intentionally invalid.
    std::mem::forget(root);
}

#[test]
fn into_raw_node_still_uses_pre_move_root_reference() {
    let root = root("let a = 123");
    let pinned = PinnedNodeData::new(root, |r| r.root());
    let (_moved_root, node) = pinned.into_raw();
    // This accesses the root reference retained by the node after Root was moved.
    assert_eq!(node.text(), "let a = 123");
}
