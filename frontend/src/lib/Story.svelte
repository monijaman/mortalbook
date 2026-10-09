<script>
  // Renders a story (light Markdown) with each text run translated into the visitor's language.
  import T from '$lib/T.svelte';
  import { parseBlocks, parseInline, padSplit } from '$lib/markdown.js';

  let { bio, from = 'auto' } = $props();
  const blocks = $derived(parseBlocks(bio));
</script>

{#snippet core(s)}
  {@const [lead, text, trail] = padSplit(s.text)}
  {lead}{#if text}
    {#if s.bold && s.italic}<strong><em><T {text} {from} /></em></strong
    >{:else if s.bold}<strong><T {text} {from} /></strong
    >{:else if s.italic}<em><T {text} {from} /></em
    >{:else}<T {text} {from} />{/if}
  {/if}{trail}
{/snippet}

{#snippet inline(text)}
  {#each parseInline(text) as s}
    {#if s.href}<a href={s.href} target="_blank" rel="noopener noreferrer nofollow">{@render core(s)}</a
      >{:else}{@render core(s)}{/if}
  {/each}
{/snippet}

<div class="story">
  {#each blocks as b}
    {#if b.type === 'heading'}
      {#if b.level === 2}<h2>{@render inline(b.text)}</h2>{:else}<h3>{@render inline(b.text)}</h3>{/if}
    {:else if b.type === 'quote'}
      <blockquote>{@render inline(b.text)}</blockquote>
    {:else if b.type === 'ul'}
      <ul>{#each b.items as item}<li>{@render inline(item)}</li>{/each}</ul>
    {:else if b.type === 'ol'}
      <ol>{#each b.items as item}<li>{@render inline(item)}</li>{/each}</ol>
    {:else}
      <p>{@render inline(b.text)}</p>
    {/if}
  {/each}
</div>

<style>
  .story { white-space: pre-line; }
  .story :global(p), .story :global(ul), .story :global(ol), .story :global(blockquote) { margin: 0 0 1em; }
  .story :global(h2), .story :global(h3) { margin: 1.4em 0 0.5em; line-height: 1.25; }
  .story :global(ul), .story :global(ol) { padding-left: 1.4em; white-space: normal; }
  .story :global(blockquote) { border-left: 3px solid var(--accent); padding-left: 1rem; color: var(--muted); font-style: italic; }
  .story :global(a) { color: var(--accent); }
</style>
