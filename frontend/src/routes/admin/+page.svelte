<script>
  import { goto } from '$app/navigation';
  import T from '$lib/T.svelte';

  let busy = $state(false);
  let error = $state('');
  let videoUrls = $state(['']);

  async function submit(e) {
    e.preventDefault();
    error = '';
    busy = true;
    try {
      const fd = new FormData(e.currentTarget);
      videoUrls.filter((u) => u.trim()).forEach((u) => fd.append('video_url', u.trim()));
      const res = await fetch('/api/people', { method: 'POST', body: fd });
      const body = await res.json().catch(() => ({}));
      if (!res.ok) throw new Error(body.error || `Error ${res.status}`);
      await goto(`/people/${body.id}`);
    } catch (err) {
      error = err.message;
    } finally {
      busy = false;
    }
  }
</script>

<svelte:head>
  <title>Add a Person — Mortalbook</title>
  <meta name="robots" content="noindex,nofollow" />
</svelte:head>

<h1><T text="Add someone you remember" /></h1>
<p class="muted"><T text="Write in any language — visitors will read it in their own." /></p>
<p><a href="/admin/review">Review automated recent-death candidates</a></p>

<form onsubmit={submit}>
  <label>
    <span><T text="Full name" /> *</span>
    <input name="name" required maxlength="200" />
  </label>

  <div class="row">
    <label>
      <span><T text="Date of birth" /></span>
      <input type="date" name="birth_date" />
    </label>
    <label>
      <span><T text="Date of passing" /> *</span>
      <input type="date" name="death_date" required />
    </label>
  </div>

  <label>
    <span><T text="Their story" /></span>
    <textarea name="bio" rows="8" maxlength="10000"></textarea>
  </label>

  <label>
    <span><T text="Main photo" /></span>
    <input type="file" name="photo" accept="image/jpeg,image/png,image/webp,image/gif" />
  </label>

  <label>
    <span><T text="More photos and videos" /></span>
    <input type="file" name="media" multiple accept="image/*,video/mp4,video/webm,video/quicktime" />
  </label>

  <fieldset>
    <legend><T text="Video links (YouTube, Vimeo)" /></legend>
    {#each videoUrls as _, i}
      <input type="url" placeholder="https://" bind:value={videoUrls[i]} />
    {/each}
    <button type="button" class="link" onclick={() => videoUrls.push('')}>+ <T text="Add another link" /></button>
  </fieldset>

  {#if error}<p class="error">{error}</p>{/if}

  <button type="submit" disabled={busy}>
    {#if busy}<T text="Saving…" />{:else}<T text="Save memorial" />{/if}
  </button>
</form>

<style>
  form { display: grid; gap: 1.2rem; max-width: 680px; margin-top: 1.5rem; }
  label span, legend { display: block; margin-bottom: 0.3rem; color: var(--muted); font-size: 0.95rem; }
  .row { display: grid; gap: 1rem; grid-template-columns: 1fr 1fr; }
  fieldset { border: 1px solid var(--line); border-radius: 8px; display: grid; gap: 0.6rem; }
  .link { border: 0; padding: 0; justify-self: start; text-decoration: underline; }
  .link:hover { background: none; color: var(--text); }
  .error { color: var(--danger); margin: 0; }
  form > button[type='submit'] { justify-self: start; }
</style>
