import { browser } from '$app/environment';
import { replaceState } from '$app/navigation';
import { writable } from 'svelte/store';

const STORAGE_KEY = 'mortalbook-country';

function detectCountryFromLocale() {
  if (!browser) return '';

  const locales = navigator.languages?.length ? navigator.languages : [navigator.language];
  for (const locale of locales) {
    const region = locale
      ?.split(/[-_]/)
      .slice(1)
      .find((part) => /^[A-Z]{2}$/i.test(part))
      ?.toUpperCase();
    if (!region) continue;

    const name = new Intl.DisplayNames(['en'], { type: 'region' }).of(region);
    if (name && name !== region) return name;
  }
  return '';
}

function loadCountryPreference() {
  if (!browser) return '';

  try {
    const urlCountry = new URL(window.location.href).searchParams.get('country');
    if (urlCountry !== null) return urlCountry;

    const saved = localStorage.getItem(STORAGE_KEY);
    if (saved !== null) return saved;

    const detected = detectCountryFromLocale();
    if (detected) localStorage.setItem(STORAGE_KEY, detected);
    return detected;
  } catch (error) {
    console.error('Unable to load the saved country preference.', error);
    return detectCountryFromLocale();
  }
}

export const selectedCountry = writable(loadCountryPreference());

export function syncCountryUrl(country) {
  if (!browser) return;

  const url = new URL(window.location.href);
  if (country) url.searchParams.set('country', country);
  else url.searchParams.delete('country');

  if (url.href !== window.location.href) replaceState(url, {});
}

if (browser) {
  selectedCountry.subscribe((country) => {
    try {
      if (country) localStorage.setItem(STORAGE_KEY, country);
      else localStorage.removeItem(STORAGE_KEY);
    } catch (error) {
      console.error('Unable to save the country preference.', error);
    }
  });
}
