/** Whole years between an ISO date (YYYY-MM-DD) and today. */
export function years(iso) {
  const [y, m, d] = iso.split('-').map(Number);
  const now = new Date();
  let n = now.getFullYear() - y;
  if (now.getMonth() + 1 < m || (now.getMonth() + 1 === m && now.getDate() < d)) n--;
  return n;
}

/** "YYYY-MM-DD" -> localized long date, using the visitor's UI language. */
export function longDate(iso, locale) {
  const [y, m, d] = iso.split('-').map(Number);
  try {
    return new Date(Date.UTC(y, m - 1, d)).toLocaleDateString(locale, {
      year: 'numeric', month: 'long', day: 'numeric', timeZone: 'UTC'
    });
  } catch {
    return iso;
  }
}

/** Embed URL for YouTube / Vimeo links, or null for anything else. */
export function embedUrl(url) {
  try {
    const u = new URL(url);
    const host = u.hostname.replace(/^www\./, '');
    if (host === 'youtu.be') return `https://www.youtube.com/embed/${u.pathname.slice(1)}`;
    if (host === 'youtube.com' && u.searchParams.get('v'))
      return `https://www.youtube.com/embed/${u.searchParams.get('v')}`;
    if (host === 'vimeo.com' && /^\/\d+/.test(u.pathname))
      return `https://player.vimeo.com/video/${u.pathname.slice(1)}`;
  } catch {}
  return null;
}

/** Age in whole years at death. */
export function ageAt(birthIso, deathIso) {
  const [by, bm, bd] = birthIso.split('-').map(Number);
  const [dy, dm, dd] = deathIso.split('-').map(Number);
  let n = dy - by;
  if (dm < bm || (dm === bm && dd < bd)) n--;
  return n;
}
