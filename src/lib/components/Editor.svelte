<script lang="ts">
  import { onMount } from "svelte";
  import { EditorView } from "@codemirror/view";
  import { EditorState } from "@codemirror/state";
  import { journalExtensions } from "$lib/editor/extensions";
  import { getEntry, setEntry } from "$lib/stores/journal.svelte";

  /**
   * Editor.svelte — CodeMirror 6 wrapper for the journal writing surface.
   *
   * Rules (from AGENTS.md):
   *  - one EditorView per mounted instance, created in onMount, destroyed on cleanup
   *  - never recreate it on prop change — push new content via view.dispatch
   *  - editor updateListener → debounce 400ms → write to store (never per keystroke)
   *  - store updates from elsewhere → dispatch only if doc differs
   */

  let { dateKey = "" } = $props();

  let host: HTMLDivElement;
  let view: EditorView | null = null;
  let saveTimer: ReturnType<typeof setTimeout> | null = null;

  onMount(() => {
    view = new EditorView({
      state: EditorState.create({
        doc: getEntry(dateKey),
        extensions: [
          ...journalExtensions(),
          EditorView.updateListener.of((update) => {
            if (!update.docChanged) return;
            scheduleSave();
          }),
        ],
      }),
      parent: host,
    });

    return () => {
      if (saveTimer) {
        clearTimeout(saveTimer);
        saveTimer = null;
      }
      view?.destroy();
      view = null;
    };
  });

  function scheduleSave(): void {
    if (saveTimer) clearTimeout(saveTimer);
    saveTimer = setTimeout(() => {
      saveTimer = null;
      if (!view) return;
      const body = view.state.doc.toString();
      void setEntry(dateKey, body);
    }, 400);
  }

  // When the date key changes (navigation to another day), swap the doc in
  // place — only when it actually differs from what's on screen.
  $effect(() => {
    const key = dateKey;
    if (!view) return;
    const stored = getEntry(key);
    const current = view.state.doc.toString();
    if (stored !== current) {
      view.dispatch({
        changes: {
          from: 0,
          to: view.state.doc.length,
          insert: stored,
        },
      });
    }
  });
</script>

<div class="editor-host" bind:this={host}></div>

<style>
  .editor-host {
    height: 100%;
    width: 100%;
  }
  .editor-host :global(.cm-editor) {
    height: 100%;
  }
  .editor-host :global(.cm-scroller) {
    height: 100%;
  }
</style>