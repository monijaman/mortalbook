<script>
  // Rich text editor for stories. Edits visually (bold, italic, headings, lists, quotes, links)
  // and stores light Markdown in `value`, so old plain-text stories open and save unchanged.
  import { toHtml, toMarkdown } from '$lib/markdown.js';
  import { adminApi } from '$lib/admin.js';

  // `context`: what is known about the person (name, dates, occupation, places) for AI generation.
  let { value = $bindable(''), max = 10000, label = 'Story', context = null } = $props();

  let editor = $state();
  let source = $state(false);
  let lastEmitted = '';
  let active = $state({});

  const words = $derived(value.trim() ? value.replace(/[*#>\[\]()-]/g, ' ').trim().split(/\s+/).length : 0);
  const over = $derived(value.length > max);

  // Load external changes (first load, switching back from source view) into the editor.
  $effect(() => {
    if (!editor || source) return;
    if (value !== lastEmitted) {
      editor.innerHTML = toHtml(value);
      lastEmitted = value;
    }
  });

  function sync() {
    lastEmitted = toMarkdown(editor);
    value = lastEmitted;
    refresh();
  }

  function refresh() {
    const q = (c) => {
      try { return document.queryCommandState(c); } catch { return false; }
    };
    const block = (document.queryCommandValue('formatBlock') || '').toLowerCase();
    active = {
      bold: q('bold'), italic: q('italic'), ul: q('insertUnorderedList'), ol: q('insertOrderedList'),
      h2: block === 'h2', h3: block === 'h3', quote: block === 'blockquote'
    };
  }

  function run(command, arg) {
    editor.focus();
    document.execCommand(command, false, arg);
    sync();
  }

  function toggleBlock(tag) {
    const current = (document.queryCommandValue('formatBlock') || '').toLowerCase();
    run('formatBlock', current === tag ? 'p' : tag);
  }

  function addLink() {
    const sel = window.getSelection();
    if (!sel || sel.isCollapsed) return alert('Select the text you want to turn into a link first.');
    const url = prompt('Link address (https://…)');
    if (!url) return;
    if (!/^https?:\/\//i.test(url.trim())) return alert('The link must start with http:// or https://');
    run('createLink', url.trim());
  }

  function onPaste(event) {
    // Never paste foreign HTML/styles; paste text (Markdown is understood) instead.
    event.preventDefault();
    const pasted = event.clipboardData.getData('text/plain');
    document.execCommand('insertHTML', false, toHtml(pasted));
    sync();
  }

  function onKeydown(event) {
    if (!(event.ctrlKey || event.metaKey)) return;
    const k = event.key.toLowerCase();
    if (k === 'k') {
      event.preventDefault();
      addLink();
    }
  }

  function onFocus() {
    document.execCommand('defaultParagraphSeparator', false, 'p');
  }

  // ---- AI story generation (OpenAI with web search, via the admin API) ----
  let dialog = $state();
  let prompt = $state('');
  let mode = $state('replace');
  let withSources = $state(true);
  let generating = $state(false);
  let genError = $state('');
  let result = $state(null);

  function openGenerate() {
    genError = '';
    result = null;
    mode = 'replace';
    dialog.showModal();
  }

  /** Drop the inline "([site](url))" citations the web-search model sometimes adds. */
  function stripCitations(text) {
    return text
      .replace(/\s*\(\s*\[[^\]]*\]\(https?:[^)]*\)\s*\)/g, '')
      .replace(/\s*\[[^\]]*\]\(https?:[^)]*utm_source=openai[^)]*\)/g, '')
      .trim();
  }

  async function generate() {
    generating = true;
    genError = '';
    result = null;
    try {
      const data = await adminApi('/api/admin/generate-story', {
        method: 'POST',
        body: JSON.stringify({
          name: context?.name || '',
          birth_date: context?.birth_date || null,
          death_date: context?.death_date || null,
          occupation: context?.occupation || null,
          birth_place: context?.birth_place || null,
          death_place: context?.death_place || null,
          prompt: prompt.trim() || null,
          current_story: value.trim() || null
        })
      });
      result = { story: stripCitations(data.story), sources: data.sources || [] };
    } catch (err) {
      genError = err.message;
    } finally {
      generating = false;
    }
  }

  function useResult() {
    let text = result.story;
    if (withSources && result.sources.length) {
      const links = result.sources
        .slice(0, 8)
        .map((s) => `- [${s.title.replace(/[\[\]]/g, '')}](${s.url})`)
        .join('\n');
      text += `\n\n**Sources**\n\n${links}`;
    }
    value = mode === 'append' && value.trim() ? `${value.trim()}\n\n${text}` : text;
    dialog.close();
  }

  function toggleSource() {
    if (!source && editor) sync();
    source = !source;
  }
</script>

<div class="story">
  <div class="top">
    <span class="label">{label}</span>
  </div>

  <div class="toolbar" role="toolbar" aria-label="Formatting">
    {#if !source}
      <button type="button" class:on={active.bold} onmousedown={(e) => e.preventDefault()} onclick={() => run('bold')} title="Bold (Ctrl+B)"><b>B</b></button>
      <button type="button" class:on={active.italic} onmousedown={(e) => e.preventDefault()} onclick={() => run('italic')} title="Italic (Ctrl+I)"><i>I</i></button>
      <span class="sep"></span>
      <button type="button" class:on={active.h2} onmousedown={(e) => e.preventDefault()} onclick={() => toggleBlock('h2')} title="Heading">H2</button>
      <button type="button" class:on={active.h3} onmousedown={(e) => e.preventDefault()} onclick={() => toggleBlock('h3')} title="Subheading">H3</button>
      <button type="button" class:on={active.quote} onmousedown={(e) => e.preventDefault()} onclick={() => toggleBlock('blockquote')} title="Quote">“ ”</button>
      <span class="sep"></span>
      <button type="button" class:on={active.ul} onmousedown={(e) => e.preventDefault()} onclick={() => run('insertUnorderedList')} title="Bulleted list">• List</button>
      <button type="button" class:on={active.ol} onmousedown={(e) => e.preventDefault()} onclick={() => run('insertOrderedList')} title="Numbered list">1. List</button>
      <span class="sep"></span>
      <button type="button" onmousedown={(e) => e.preventDefault()} onclick={addLink} title="Add link (Ctrl+K)">🔗 Link</button>
      <button type="button" onmousedown={(e) => e.preventDefault()} onclick={() => run('unlink')} title="Remove link">Unlink</button>
      <span class="sep"></span>
      <button type="button" onmousedown={(e) => e.preventDefault()} onclick={() => run('undo')} title="Undo (Ctrl+Z)">↶</button>
      <button type="button" onmousedown={(e) => e.preventDefault()} onclick={() => run('redo')} title="Redo (Ctrl+Y)">↷</button>
      <button type="button" onmousedown={(e) => e.preventDefault()} onclick={() => run('removeFormat')} title="Clear formatting">Clear</button>
    {/if}
    {#if context}
      <button type="button" class="right ai" onclick={openGenerate} title="Write the story with AI, using a web search">✨ Generate story</button>
    {/if}
    <button type="button" class:right={!context} class:on={source} onclick={toggleSource} title="Edit the raw text">
      {source ? 'Back to editor' : '</> Source'}
    </button>
  </div>

  {#if source}
    <textarea bind:value rows="16" aria-label="{label} (source)" spellcheck="true"></textarea>
  {:else}
    <div
      class="editor"
      contenteditable="true"
      role="textbox"
      aria-multiline="true"
      aria-label={label}
      tabindex="0"
      bind:this={editor}
      oninput={sync}
      onkeyup={refresh}
      onmouseup={refresh}
      onfocus={onFocus}
      onkeydown={onKeydown}
      onpaste={onPaste}
    ></div>
  {/if}

  <div class="stats" class:over>
    {words} words · {value.length.toLocaleString()} / {max.toLocaleString()} characters
    {#if over}— too long, shorten it before saving{/if}
  </div>
</div>

<dialog bind:this={dialog} class="gen" aria-labelledby="gen-title">
  <h3 id="gen-title">✨ Generate story with AI</h3>
  <p class="muted">
    OpenAI searches the web for <strong>{context?.name || 'this person'}</strong> and writes a draft.
    You review it before anything is saved. Always check the facts.
  </p>

  {#if !result}
    <label>
      <span>Optional instructions</span>
      <textarea
        bind:value={prompt}
        rows="4"
        maxlength="2000"
        placeholder="e.g. Focus on his literary work. Keep it under 300 words. Write in Bengali."
        disabled={generating}
      ></textarea>
    </label>
    {#if genError}<p class="error" role="alert">{genError}</p>{/if}
    {#if generating}<p class="muted" role="status">Searching the web and writing… this can take up to a minute.</p>{/if}
    <div class="gen-actions">
      <button type="button" onclick={generate} disabled={generating}>{generating ? 'Generating…' : 'Generate'}</button>
      <button type="button" class="plain" onclick={() => dialog.close()} disabled={generating}>Cancel</button>
    </div>
  {:else}
    <div class="gen-preview">{@html toHtml(result.story)}</div>
    {#if result.sources.length}
      <details>
        <summary>{result.sources.length} sources found</summary>
        <ul class="src">
          {#each result.sources as src}<li><a href={src.url} target="_blank" rel="noopener noreferrer">{src.title}</a></li>{/each}
        </ul>
      </details>
      <label class="check"><input type="checkbox" bind:checked={withSources} /> Add the sources list under the story</label>
    {/if}
    {#if value.trim()}
      <fieldset class="mode">
        <label><input type="radio" bind:group={mode} value="replace" /> Replace the current story</label>
        <label><input type="radio" bind:group={mode} value="append" /> Add after the current story</label>
      </fieldset>
    {/if}
    <div class="gen-actions">
      <button type="button" onclick={useResult}>Use this story</button>
      <button type="button" class="plain" onclick={generate} disabled={generating}>{generating ? 'Generating…' : 'Try again'}</button>
      <button type="button" class="plain" onclick={() => dialog.close()}>Discard</button>
    </div>
    {#if genError}<p class="error" role="alert">{genError}</p>{/if}
  {/if}
</dialog>

<style>
  .ai { background: var(--accent) !important; color: var(--accent-contrast) !important; margin-left: auto; }
  .gen {
    width: min(680px, 94vw); max-height: 90vh; overflow: auto; border: 1px solid var(--line);
    border-radius: 12px; background: var(--panel); color: var(--text); padding: 1.25rem 1.5rem;
  }
  .gen::backdrop { background: rgba(0, 0, 0, 0.55); }
  .gen h3 { margin: 0 0 0.5rem; }
  .gen label span { display: block; margin-bottom: 0.3rem; color: var(--muted); }
  .gen textarea { width: 100%; box-sizing: border-box; min-height: 0; font-size: 1rem; }
  .gen-actions { display: flex; gap: 0.6rem; flex-wrap: wrap; margin-top: 1rem; }
  .plain { background: transparent; color: var(--text); border: 1px solid var(--line); }
  .gen-preview {
    max-height: 45vh; overflow: auto; border: 1px solid var(--line); border-radius: 8px;
    padding: 0.5rem 1rem; background: var(--surface); line-height: 1.6; margin: 0.75rem 0;
  }
  .gen-preview :global(p) { margin: 0 0 1em; }
  .src { margin: 0.4rem 0; padding-left: 1.2rem; font-size: 0.9rem; }
  .check, .mode label { display: block; margin: 0.5rem 0; }
  .mode { border: 0; padding: 0; margin: 0.5rem 0; }
  .error { color: var(--danger); }
  .story { display: grid; gap: 0.5rem; max-width: 760px; }
  .label { color: var(--muted); font-size: 0.95rem; }
  .toolbar {
    display: flex; flex-wrap: wrap; gap: 0.3rem; align-items: center; padding: 0.35rem;
    border: 1px solid var(--line); border-radius: 8px; background: var(--panel); position: sticky; top: 0; z-index: 2;
  }
  .toolbar button {
    padding: 0.25rem 0.6rem; font-size: 0.9rem; min-width: 2rem; background: transparent;
    color: var(--text); border: 1px solid transparent;
  }
  .toolbar button:hover { border-color: var(--line); background: var(--surface); color: var(--text); }
  .toolbar button.on { background: var(--accent); color: var(--accent-contrast); }
  .sep { width: 1px; height: 1.4rem; background: var(--line); margin: 0 0.2rem; }
  .right { margin-left: auto; }
  .editor, textarea {
    box-sizing: border-box; width: 100%; min-height: 22rem; padding: 0.9rem 1.1rem;
    border: 1px solid var(--line); border-radius: 8px; background: var(--surface); color: var(--text);
    font: inherit; font-size: 1.1rem; line-height: 1.65;
  }
  .editor { overflow-wrap: anywhere; }
  .editor:focus { outline: 2px solid var(--accent); outline-offset: 1px; }
  .editor :global(p), .editor :global(ul), .editor :global(ol), .editor :global(blockquote) { margin: 0 0 1em; white-space: pre-wrap; }
  .editor :global(h2), .editor :global(h3) { margin: 1.2em 0 0.5em; line-height: 1.25; }
  .editor :global(ul), .editor :global(ol) { padding-left: 1.4em; }
  .editor :global(blockquote) { border-left: 3px solid var(--accent); padding-left: 1rem; color: var(--muted); font-style: italic; }
  .editor :global(a) { color: var(--accent); text-decoration: underline; }
  textarea { font-family: ui-monospace, monospace; font-size: 0.95rem; }
  .stats { color: var(--muted); font-size: 0.85rem; }
  .stats.over { color: var(--danger); }
</style>
