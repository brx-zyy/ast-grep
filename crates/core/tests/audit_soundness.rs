use ast_grep_core::language::Tsx;
use ast_grep_core::pinned::PinnedNodeData;
use ast_grep_core::source::Edit;
use ast_grep_core::tree_sitter::LanguageExt;

#[test]
fn safe_edit_can_break_string_utf8_invariant() {
    let mut root = Tsx.ast_grep("ok");
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
    let root = Tsx.ast_grep("let a = 123");
    let pinned = PinnedNodeData::new(root, |r| r.root());
    let (_moved_root, node) = pinned.into_raw();
    // This accesses the root reference retained by the node after Root was moved.
    assert_eq!(node.text(), "let a = 123");
}
