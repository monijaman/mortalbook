<template>
  <v-text-field
    v-model="query"
    :placeholder="placeholder"
    prepend-inner-icon="mdi-magnify"
    :append-inner-icon="query ? 'mdi-close' : ''"
    variant="outlined"
    density="compact"
    hide-details
    clearable
    style="max-width: 360px"
    @click:append-inner="clear"
    @keydown.escape="clear"
  />
</template>

<script setup lang="ts">
import { ref, watch } from "vue";

const props = defineProps<{
  modelValue?: string;
  placeholder?: string;
  debounce?: number;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", val: string): void;
  (e: "search", val: string): void;
}>();

const query = ref(props.modelValue ?? "");
let debounceTimer: ReturnType<typeof setTimeout> | null = null;

watch(query, (val) => {
  emit("update:modelValue", val ?? "");
  if (debounceTimer) clearTimeout(debounceTimer);
  debounceTimer = setTimeout(
    () => emit("search", (val ?? "").trim()),
    props.debounce ?? 350,
  );
});

watch(
  () => props.modelValue,
  (val) => {
    if (val !== query.value) query.value = val ?? "";
  },
);

function clear() {
  query.value = "";
}
</script>
