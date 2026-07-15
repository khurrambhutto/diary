import { EditorView, keymap } from "@codemirror/view";
import { EditorState } from "@codemirror/state";
import { defaultKeymap, history, historyKeymap } from "@codemirror/commands";
import { journalTheme } from "./theme";
import type { Extension } from "@codemirror/state";

/**
 * Journal extension preset — pure prose, no code-ide features.
 * No markdown, no vim, no bracket-matching, no indent-on-input.
 * This is a blank page for writing, not an editor.
 */
export function journalExtensions(): Extension[] {
  return [
    history(),
    EditorView.lineWrapping,
    EditorState.allowMultipleSelections.of(true),
    journalTheme,
    keymap.of([...defaultKeymap, ...historyKeymap]),
  ];
}