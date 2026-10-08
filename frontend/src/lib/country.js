import { browser } from '$app/environment';
import { writable } from 'svelte/store';

const STORAGE_KEY = 'mortalbook-country';
const savedCountry = browser ? localStorage.getItem(STORAGE_KEY) || '' : '';

export const selectedCountry = writable(savedCountry);

if (browser) {
  selectedCountry.subscribe((country) => {
    if (country) localStorage.setItem(STORAGE_KEY, country);
    else localStorage.removeItem(STORAGE_KEY);
  });
}
