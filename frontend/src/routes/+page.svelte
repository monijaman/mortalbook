<script>
  import { onMount } from 'svelte';
  import T from '$lib/T.svelte';
  import PersonCard from '$lib/PersonCard.svelte';
  import Slideshow from '$lib/Slideshow.svelte';
  import SearchBox from '$lib/SearchBox.svelte';
  import { lang } from '$lib/i18n.js';
  import { selectedCountry } from '$lib/country.js';

  let people = $state([]);
  let upcoming = $state([]);
  let recent = $state([]);
  let loading = $state(true);
  let failed = $state(false);
  let today = $state(new Date());

  function locationHints() {
    const timezone = Intl.DateTimeFormat().resolvedOptions().timeZone || '';
    const locale = navigator.language || '';
    const city = timezone.split('/').pop()?.replaceAll('_', ' ') || '';
    const region = locale.match(/[-_]([A-Z]{2})$/i)?.[1]?.toUpperCase() || '';
    const countryNames = { BD: 'Bangladesh', IN: 'India', PK: 'Pakistan', NP: 'Nepal', LK: 'Sri Lanka', BT: 'Bhutan', MM: 'Myanmar' };
    const country = countryNames[region] || '';
    const subcontinent = ['Bangladesh', 'India', 'Pakistan', 'Nepal', 'Sri Lanka', 'Bhutan', 'Myanmar'].includes(country) ? 'South Asia' : '';
    return [city, country, subcontinent, timezone.split('/')[0]].filter(Boolean).map((term) => term.toLowerCase());
  }

  function sortByLocation(items, country) {
    const hints = country ? [country.toLowerCase()] : locationHints();
    return [...items].sort((a, b) => {
      const score = (person) => {
        const place = `${person.death_place || ''} ${person.birth_place || ''}`.toLowerCase();
        const rank = hints.findIndex((hint) => place.includes(hint));
        return rank < 0 ? hints.length : rank;
      };
      return score(a) - score(b) || new Date(b.death_date) - new Date(a.death_date) || a.name.localeCompare(b.name);
    });
  }

  onMount(() => {
    // use the visitor's local date, not the server's
    today = new Date();
    let controller;
    const unsubscribe = selectedCountry.subscribe((country) => {
      controller?.abort();
      controller = new AbortController();
      const currentController = controller;
      const params = new URLSearchParams({
        year: String(today.getFullYear()),
        month: String(today.getMonth() + 1),
        day: String(today.getDate())
      });
      if (country) params.set('country', country);
      loading = true;
      failed = false;
      upcoming = [];
      recent = [];

      Promise.all([
        fetch(`/api/people/today?${params}`, { signal: currentController.signal }),
        fetch(`/api/people/week?${params}`, { signal: currentController.signal })
      ])
        .then(async ([todayResponse, weekResponse]) => {
          if (!todayResponse.ok) throw new Error(`Today's people request failed: ${todayResponse.status}`);
          people = sortByLocation(await todayResponse.json(), country);
          if (weekResponse.ok) ({ upcoming, recent } = await weekResponse.json());
          else console.error(`Weekly people request failed: ${weekResponse.status}`);
        })
        .catch((error) => {
          if (error.name !== 'AbortError') {
            console.error(error);
            failed = true;
          }
        })
        .finally(() => {
          if (!currentController.signal.aborted) loading = false;
        });
    });

    return () => {
      unsubscribe();
      controller?.abort();
    };
  });

  const dateLabel = $derived(today.toLocaleDateString($lang, { month: 'long', day: 'numeric' }));
</script>

<svelte:head><title>Mortalbook — remembered today</title></svelte:head>

<section class="hero">
  <p class="eyebrow"><T text="Remembered on this day" /></p>
  <h1>{dateLabel}</h1>
  <div class="find"><SearchBox large /></div>
</section>

{#if loading}
  <p class="muted center"><T text="Loading…" /></p>
{:else if failed}
  <p class="muted center"><T text="Something went wrong. Please try again later." /></p>
{:else if people.length === 0}
  <p class="muted center empty">
    <T text="No one in our book passed away on this day." />
    <a href="/admin"><T text="Add someone you remember." /></a>
  </p>
{:else}
  <Slideshow {people} />

  <section class="block">
    <h2><T text="Also remembered today" /></h2>
    <div class="grid">
      {#each people as person (person.id)}
        <PersonCard {person} showAgo />
      {/each}
    </div>
  </section>
{/if}

{#if upcoming.length}
  <section class="block">
    <h2><T text="Coming up this week" /></h2>
    <div class="grid">
      {#each upcoming as person (person.id)}
        <PersonCard {person} showAgo days={person.days} />
      {/each}
    </div>
  </section>
{/if}

{#if recent.length}
  <section class="block">
    <h2><T text="Last week" /></h2>
    <div class="grid">
      {#each recent as person (person.id)}
        <PersonCard {person} showAgo days={person.days} />
      {/each}
    </div>
  </section>
{/if}

<p class="center more"><a class="btn" href="/people"><T text="Browse everyone remembered" /> →</a></p>

<style>
  .hero { text-align: center; padding: 1.5rem 0 2.5rem; }
  .eyebrow { color: var(--accent); text-transform: uppercase; letter-spacing: 0.2em; font-size: 0.85rem; margin: 0; }
  h1 { font-size: clamp(2.2rem, 6vw, 3.6rem); margin: 0.3rem 0 0; }
  .find { margin-top: 2rem; }
  .center { text-align: center; }
  .block { margin: 0 0 3.5rem; }
  .block h2 { margin: 0 0 1.2rem; padding-bottom: 0.5rem; border-bottom: 1px solid var(--line); }
  .more { margin: 1rem 0 0; }
</style>
