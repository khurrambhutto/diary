import { EditorView, keymap } from "@codemirror/view";
import { EditorState } from "@codemirror/state";
import { defaultKeymap, history, historyKeymap } from "@codemirror/commands";
import { searchKeymap, highlightSelectionMatches } from "@codemirror/search";
import { bracketMatching, indentOnInput } from "@codemirror/language";
import { journalTheme } from "./theme";
import type { Extension } from "@codemirror/state";

/**
 * Phase 1 extension preset — deliberately minimal.
 * No markdown, no vim, no language package. Add those in Phase 2.
 */
export function journalExtensions(): Extension[] {
  return [
    history(),
    EditorView.lineWrapping,
    highlightSelectionMatches(),
    indentOnInput(),
    bracketMatching(),
    EditorState.allowMultipleSelections.of(true),
    journalTheme,
    keymap.of([...defaultKeymap, ...historyKeymap, ...searchKeymap]),
  ];
}