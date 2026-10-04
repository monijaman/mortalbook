<script>
  import { onMount } from 'svelte';
  import T from '$lib/T.svelte';
  import PersonCard from '$lib/PersonCard.svelte';
  import { lang } from '$lib/i18n.js';
  import { longDate } from '$lib/format.js';

  let people = $state([]);
  let loading = $state(true);
  let failed = $state(false);
  let today = $state(new Date());

  onMount(async () => {
    // use the visitor's local date, not the server's
    today = new Date();
    try {
      const res = await fetch(`/api/people/today?month=${today.getMonth() + 1}&day=${today.getDate()}`);
      if (!res.ok) throw new Error();
      people = await res.json();
    } catch {
      failed = true;
    }
    loading = false;
  });

  const dateLabel = $derived(
    today.toLocaleDateString($lang, { month: 'long', day: 'numeric' })
  );
</script>

<svelte:head><title>Mortalbook — remembered today</title></svelte:head>

<section class="hero">
  <p class="eyebrow"><T text="Remembered on this day" /></p>
  <h1>{dateLabel}</h1>
</section>

{#if loading}
  <p class="muted"><T text="Loading…" /></p>
{:else if failed}
  <p class="muted"><T text="Something went wrong. Please try again later." /></p>
{:else if people.length === 0}
  <p class="muted empty">
    <T text="No one in our book passed away on this day." />
    <a href="/admin"><T text="Add someone you remember." /></a>
  </p>
{:else}
  <div class="grid">
    {#each people as person (person.id)}
      <PersonCard {person} showAgo />
    {/each}
  </div>
{/if}

<style>
  .hero { text-align: center; padding: 1.5rem 0 2.5rem; }
  .eyebrow { color: var(--accent); text-transform: uppercase; letter-spacing: 0.2em; font-size: 0.85rem; margin: 0; }
  h1 { font-size: clamp(2.2rem, 6vw, 3.6rem); margin: 0.3rem 0 0; }
  .empty { text-align: center; }
</style>
