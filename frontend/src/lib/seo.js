import { plainText } from '$lib/markdown.js';
export const SITE_URL = 'https://mortalbook.com';
export const DEFAULT_DESCRIPTION =
  'Mortalbook is a place to remember people who have passed away. Discover their lives, stories, and anniversaries.';
export const DEFAULT_IMAGE = `${SITE_URL}/og-image.svg`;

export function canonicalUrl(path) {
  return new URL(path, SITE_URL).href;
}

export function personDescription(person) {
  const bio = plainText(person.bio || '').replace(/\s+/g, ' ').trim();
  if (bio) return bio.length > 300 ? `${bio.slice(0, 297).trimEnd()}...` : bio;

  const dates = [person.birth_date, person.death_date].filter(Boolean);
  const occupation = person.occupation ? ` ${person.occupation}.` : '';
  return `Remembering ${person.name}.${occupation}${dates.length ? ` Life dates: ${dates.join(' – ')}.` : ''}`;
}

export function jsonLd(value) {
  return JSON.stringify(value).replace(/</g, '\\u003c');
}
