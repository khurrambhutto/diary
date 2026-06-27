import { EditorView } from "@codemirror/view";

/**
 * CodeMirror theme for the journal surface.
 * Per AGENTS: no line numbers, no gutters, Newsreader 17px / 1.7, paper bg.
 * The :root CSS tokens drive colors so light/dark follow the OS automatically.
 */
export const journalTheme = EditorView.theme({
  "&": {
    height: "100%",
    backgroundColor: "transparent",
    color: "var(--ink)",
  },
  ".cm-scroller": {
    fontFamily: "inherit",
    overflowY: "auto",
  },
  ".cm-content": {
    fontFamily: '"Newsreader", Georgia, serif',
    fontSize: "17px",
    lineHeight: "1.7",
    padding: "2rem 2.5rem",
    caretColor: "var(--ink)",
    maxWidth: "100%",
  },
  ".cm-line": { padding: 0 },
  // No gutters / line numbers — this is a journal, not an IDE.
  ".cm-gutters": { display: "none" },
  // Keep the active-line highlight very subtle (or none).
  "&.cm-focused .cm-activeLine": { backgroundColor: "transparent" },
  ".cm-activeLine": { backgroundColor: "transparent" },
  // Selection uses the accent-soft token.
  "&.cm-focused .cm-selectionBackground, ::selection": {
    backgroundColor: "var(--accent-soft)",
  },
  // Cursor line caret color.
  ".cm-cursor, .cm-dropCursor": { borderLeftColor: "var(--ink)" },
});