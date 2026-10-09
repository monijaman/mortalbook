<script>
  import T from '$lib/T.svelte';
  import { years, yearLabel } from '$lib/format.js';

  // One person at a time, `interval` ms each, cross-fading.
  let { people, interval = 7000 } = $props();

  let index = $state(0);
  let paused = $state(false);
  const visible = $derived(Array.from({ length: Math.min(3, people.length) }, (_, offset) => people[(index + offset) % people.length]).filter(Boolean));

  $effect(() => {
    void index; // restart the timer after every change, including manual dot clicks
    if (people.length < 2 || paused) return;
    const id = setInterval(() => {
      index = (index + 1) % people.length;
    }, interval);
    return () => clearInterval(id);
  });

  $effect(() => {
    if (index >= people.length) index = 0;
  });

  const excerpt = (bio) => {
    const text = bio.split('\n\nSource:')[0].trim();
    return text.length > 320 ? text.slice(0, 320).trimEnd() + '…' : text;
  };
</script>

<section
  class="show"
  role="group"
  aria-roledescription="carousel"
  onmouseenter={() => (paused = true)}
  onmouseleave={() => (paused = false)}
  onfocusin={() => (paused = true)}
  onfocusout={() => (paused = false)}
>
  <div class="stage">
    {#each visible as p (p.id)}
      <article class="slide active">
        <div class="photo">
          {#if p.photo_url}
            <img src={p.photo_url} alt={p.name} />
          {:else}
            <div class="placeholder">✦</div>
          {/if}
        </div>
        <div class="text">
          <h2>{p.name}</h2>
          <p class="dates">
            {p.birth_date ? yearLabel(p.birth_date, p.birth_precision) : '—'} – {yearLabel(p.death_date, p.death_precision)}
            {#if years(p.death_date) > 0}
              · {years(p.death_date)} <T text={years(p.death_date) === 1 ? 'year ago' : 'years ago'} />
            {/if}
          </p>
          {#if p.occupation}<p class="occ"><T text={p.occupation} /></p>{/if}
          {#if p.bio}<p class="bio"><T text={excerpt(p.bio)} from={p.lang || 'auto'} /></p>{/if}
          <a class="btn" href="/people/{p.id}"><T text="Read their story" /></a>
        </div>
      </article>
    {/each}
  </div>

  {#if people.length > 1}
    <div class="dots" role="tablist" aria-label="Choose person">
      {#each people as p, i (p.id)}
        <button
          class="dot"
          class:on={i === index}
          role="tab"
          aria-selected={i === index}
          aria-label={p.name}
          onclick={() => (index = i)}
        ></button>
      {/each}
    </div>
  {/if}
</section>

<style>
  .show { margin: 0 0 3rem; }
  .stage { display: grid; grid-template-columns: repeat(3, minmax(0, 1fr)); gap: 1rem; }
  .slide {
    display: grid; grid-template-columns: 1fr; gap: 1rem;
    align-items: center; padding: 1.6rem; border: 1px solid var(--line); border-radius: 14px;
    background: linear-gradient(135deg, var(--surface), var(--panel));
    animation: fade-in 1.2s ease;
  }
  .photo { aspect-ratio: 3 / 4; background: var(--photo-placeholder); border-radius: 10px; overflow: hidden; }
  img { width: 100%; height: 100%; object-fit: cover; filter: grayscale(0.75) contrast(0.95); }
  .placeholder { height: 100%; display: grid; place-items: center; font-size: 4rem; color: var(--muted); }
  h2 { font-size: clamp(1.8rem, 4vw, 2.8rem); margin: 0; }
  .dates { color: var(--accent); margin: 0.3rem 0 0.2rem; font-size: 1.1rem; }
  .occ { margin: 0 0 0.8rem; font-style: italic; }
  .bio { margin: 0 0 1.4rem; color: var(--muted); line-height: 1.65; display: -webkit-box; -webkit-line-clamp: 6; line-clamp: 6; -webkit-box-orient: vertical; overflow: hidden; }
  .dots { display: flex; justify-content: center; flex-wrap: wrap; gap: 0.55rem; margin-top: 1.2rem; }
  .dot { width: 10px; height: 10px; padding: 0; border-radius: 50%; border: 1px solid var(--muted); background: transparent; }
  .dot:hover { background: var(--muted); color: inherit; }
  .dot.on { background: var(--accent); border-color: var(--accent); }
  @keyframes fade-in { from { opacity: 0; } to { opacity: 1; } }
  @media (max-width: 720px) {
    .stage { grid-template-columns: 1fr; }
    .photo { aspect-ratio: 4 / 3; max-height: 280px; }
  }
  @media (prefers-reduced-motion: reduce) {
    .slide { animation: none; }
  }
</style>
