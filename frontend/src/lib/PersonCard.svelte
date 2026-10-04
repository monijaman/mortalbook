<script>
  import T from '$lib/T.svelte';
  import { years } from '$lib/format.js';

  let { person, showAgo = false } = $props();
  const excerpt = $derived(
    person.bio.length > 160 ? person.bio.slice(0, 160).trimEnd() + '…' : person.bio
  );
  const ago = $derived(showAgo ? years(person.death_date) : 0);
</script>

<a class="card" href="/people/{person.id}">
  <div class="photo">
    {#if person.photo_url}
      <img src={person.photo_url} alt={person.name} loading="lazy" />
    {:else}
      <div class="placeholder">✦</div>
    {/if}
  </div>
  <div class="body">
    <h3>{person.name}</h3>
    <p class="dates">
      {person.birth_date ? person.birth_date.slice(0, 4) : '—'} – {person.death_date.slice(0, 4)}
      {#if showAgo && ago > 0}· {ago} <T text={ago === 1 ? 'year ago' : 'years ago'} />{/if}
    </p>
    {#if excerpt}<p class="excerpt"><T text={excerpt} from={person.lang || 'auto'} /></p>{/if}
  </div>
</a>

<style>
  .card {
    display: flex; flex-direction: column; background: var(--panel); border: 1px solid var(--line);
    border-radius: 10px; overflow: hidden; color: inherit; text-decoration: none;
    transition: border-color 0.2s, transform 0.2s;
  }
  .card:hover { border-color: var(--accent); transform: translateY(-2px); }
  .photo { aspect-ratio: 4 / 3; background: #0b0c0e; }
  img { width: 100%; height: 100%; object-fit: cover; filter: grayscale(0.85) contrast(0.95); }
  .card:hover img { filter: grayscale(0.2); }
  .placeholder { height: 100%; display: grid; place-items: center; font-size: 2.5rem; color: var(--muted); }
  .body { padding: 0.9rem 1rem 1.1rem; }
  h3 { margin: 0 0 0.2rem; font-weight: 600; font-size: 1.2rem; }
  .dates { margin: 0 0 0.6rem; color: var(--accent); font-size: 0.9rem; }
  .excerpt { margin: 0; color: var(--muted); font-size: 0.95rem; line-height: 1.5; }
</style>
