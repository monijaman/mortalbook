<script>
  import T from '$lib/T.svelte';
  import { lang } from '$lib/i18n.js';
  import { dateLabel, ageAt, embedUrl } from '$lib/format.js';
  import { canonicalUrl, DEFAULT_IMAGE, jsonLd, personDescription } from '$lib/seo.js';

  let { data } = $props();
  const p = $derived(data.person);
  const images = $derived(p.media.filter((m) => m.kind === 'image'));
  const videos = $derived(p.media.filter((m) => m.kind === 'video'));
  // only when both dates are exact enough for a meaningful number
  const exact = (pr) => pr === 'day' || pr === 'month' || pr === 'year';
  const age = $derived(
    p.birth_date && exact(p.birth_precision) && exact(p.death_precision) ? ageAt(p.birth_date, p.death_date) : null
  );
  const paragraphs = $derived(p.bio.split(/\n{2,}/).filter((s) => s.trim()));
  const description = $derived(personDescription(p));
  const url = $derived(canonicalUrl(`/people/${p.id}`));
  const structuredData = $derived({
    '@context': 'https://schema.org',
    '@type': 'Person',
    name: p.name,
    description,
    url,
    ...(p.birth_date && { birthDate: p.birth_date }),
    deathDate: p.death_date,
    ...(p.birth_place && { birthPlace: { '@type': 'Place', name: p.birth_place } }),
    ...(p.death_place && { deathPlace: { '@type': 'Place', name: p.death_place } }),
    ...(p.occupation && { jobTitle: p.occupation }),
    ...(p.photo_url && { image: new URL(p.photo_url, 'https://mortalbook.com').href })
  });
</script>

<svelte:head>
  <title>{p.name} — Mortalbook Memorial</title>
  <link rel="canonical" href={url} />
  <meta name="description" content={description} />
  <meta property="og:type" content="profile" />
  <meta property="og:title" content="{p.name} — Mortalbook Memorial" />
  <meta property="og:description" content={description} />
  <meta property="og:url" content={url} />
  <meta property="og:image" content={p.photo_url ? new URL(p.photo_url, 'https://mortalbook.com').href : DEFAULT_IMAGE} />
  <meta name="twitter:title" content="{p.name} — Mortalbook Memorial" />
  <meta name="twitter:description" content={description} />
  <meta name="twitter:image" content={p.photo_url ? new URL(p.photo_url, 'https://mortalbook.com').href : DEFAULT_IMAGE} />
  <script type="application/ld+json">{@html jsonLd(structuredData)}</script>
</svelte:head>

<a class="back" href="/people">← <T text="Remembered" /></a>

<article>
  <div class="head">
    {#if p.photo_url}
      <img class="portrait" src={p.photo_url} alt={p.name} />
    {/if}
    <div>
      <h1>{p.name}</h1>
      <p class="dates">
        {#if p.birth_date}{dateLabel(p.birth_date, p.birth_precision, $lang)} – {/if}{dateLabel(p.death_date, p.death_precision, $lang)}
      </p>
      {#if p.occupation}<p class="occ"><T text={p.occupation} /></p>{/if}
      {#if age !== null && age >= 0}
        <p class="muted"><T text="Lived" /> {age} <T text="years" /></p>
      {/if}
    </div>
  </div>

  {#if p.birth_place || p.death_place}
    <dl class="facts">
      {#if p.birth_place}<div><dt><T text="Born in" /></dt><dd>{p.birth_place}</dd></div>{/if}
      {#if p.death_place}<div><dt><T text="Passed away in" /></dt><dd>{p.death_place}</dd></div>{/if}
    </dl>
  {/if}

  {#if paragraphs.length}
    <section class="bio">
      {#each paragraphs as para}
        <p><T text={para} from={p.lang || 'auto'} /></p>
      {/each}
    </section>
  {/if}

  {#if images.length > 1}
    <h2><T text="Photos" /></h2>
    <div class="gallery">
      {#each images as m (m.id)}
        <a href={m.url} target="_blank" rel="noopener"><img src={m.url} alt={p.name} loading="lazy" /></a>
      {/each}
    </div>
  {/if}

  {#if videos.length}
    <h2><T text="Videos" /></h2>
    <div class="videos">
      {#each videos as m (m.id)}
        {#if m.url.startsWith('/uploads/')}
          <!-- svelte-ignore a11y_media_has_caption -->
          <video src={m.url} controls preload="metadata"></video>
        {:else if embedUrl(m.url)}
          <iframe src={embedUrl(m.url)} title={p.name} allowfullscreen loading="lazy"></iframe>
        {:else}
          <a href={m.url} target="_blank" rel="noopener noreferrer">{m.url}</a>
        {/if}
      {/each}
    </div>
  {/if}
</article>

<style>
  .back { color: var(--muted); text-decoration: none; }
  .head { display: flex; flex-wrap: wrap; gap: 2rem; align-items: center; margin: 1.5rem 0 2rem; }
  .portrait {
    width: 240px; max-width: 100%; aspect-ratio: 3 / 4; object-fit: cover; border-radius: 10px;
    border: 1px solid var(--line); filter: grayscale(0.7);
  }
  h1 { font-size: clamp(2rem, 5vw, 3.2rem); margin: 0; }
  .dates { color: var(--accent); font-size: 1.2rem; margin: 0.4rem 0; }
  .occ { margin: 0 0 0.4rem; font-style: italic; }
  .facts { display: flex; flex-wrap: wrap; gap: 0.5rem 2.5rem; margin: 0 0 2rem; }
  .facts div { display: flex; flex-direction: column; }
  dt { color: var(--muted); font-size: 0.85rem; text-transform: uppercase; letter-spacing: 0.1em; }
  dd { margin: 0; font-size: 1.15rem; }
  .bio { max-width: 720px; font-size: 1.2rem; }
  .gallery { display: grid; gap: 0.8rem; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); }
  .gallery img { width: 100%; aspect-ratio: 1; object-fit: cover; border-radius: 8px; filter: grayscale(0.6); }
  .gallery img:hover { filter: none; }
  .videos { display: grid; gap: 1rem; grid-template-columns: repeat(auto-fill, minmax(320px, 1fr)); }
  video, iframe { width: 100%; aspect-ratio: 16 / 9; border: 0; border-radius: 8px; background: #000; }
</style>
