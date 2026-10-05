/**
 * Split an ISO date into numbers. Handles the signed years the API sends for BC dates
 * ("-0043-03-15" is 44 BC: astronomical numbering, where year 0 is 1 BC).
 */
export function parseIso(iso) {
  const m = /^([+-]?\d+)-(\d{2})-(\d{2})/.exec(iso);
  return m ? [Number(m[1]), Number(m[2]), Number(m[3])] : [NaN, NaN, NaN];
}

/** Astronomical year -> "1879" or "44 BC". */
function yearText(y) {
  return y <= 0 ? `${1 - y} BC` : String(y);
}

/** Year shown on cards: "1879", "44 BC", "c. 1230s", "5th century BC". */
export function yearLabel(iso, precision = 'day') {
  const [y] = parseIso(iso);
  if (precision === 'decade') return `${yearText(y)}s`.replace(/^(\d+) BCs$/, '$1s BC');
  if (precision === 'century') return centuryText(y);
  return yearText(y);
}

function centuryText(y) {
  const bc = y <= 0;
  const c = bc ? Math.floor(-y / 100) + 1 : Math.floor((y - 1) / 100) + 1;
  const suf = c % 100 >= 11 && c % 100 <= 13 ? 'th' : ({ 1: 'st', 2: 'nd', 3: 'rd' })[c % 10] || 'th';
  return `${c}${suf} century${bc ? ' BC' : ''}`;
}

/** Full date at the precision we actually know it. */
export function dateLabel(iso, precision, locale) {
  if (precision === 'day') return longDate(iso, locale);
  if (precision === 'month') {
    const [y, m] = parseIso(iso);
    const d = utcDate(y, m, 1);
    try {
      return d.toLocaleDateString(locale, { year: 'numeric', month: 'long', timeZone: 'UTC', ...(y <= 0 && { era: 'short' }) });
    } catch {
      return iso;
    }
  }
  return yearLabel(iso, precision);
}

function utcDate(y, m, d) {
  const date = new Date(Date.UTC(2000, m - 1, d));
  date.setUTCFullYear(y); // Date.UTC maps years 0-99 to 1900-1999
  return date;
}

/** Whole years between an ISO date (YYYY-MM-DD) and today. */
export function years(iso) {
  const [y, m, d] = parseIso(iso);
  const now = new Date();
  let n = now.getFullYear() - y;
  if (now.getMonth() + 1 < m || (now.getMonth() + 1 === m && now.getDate() < d)) n--;
  return n;
}

/** "YYYY-MM-DD" -> localized long date, using the visitor's UI language. */
export function longDate(iso, locale) {
  const [y, m, d] = parseIso(iso);
  try {
    return utcDate(y, m, d).toLocaleDateString(locale, {
      year: 'numeric', month: 'long', day: 'numeric', timeZone: 'UTC', ...(y <= 0 && { era: 'short' })
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
  const [by, bm, bd] = parseIso(birthIso);
  const [dy, dm, dd] = parseIso(deathIso);
  let n = dy - by;
  if (dm < bm || (dm === bm && dd < bd)) n--;
  return n;
}
