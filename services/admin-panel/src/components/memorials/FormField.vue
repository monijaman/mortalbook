<template>
  <!-- FormField — label + slot + optional hint/error -->
  <div>
    <label :for="fieldId" class="mb-1 block text-sm font-medium text-gray-700">
      {{ label
      }}<span v-if="required" class="ml-0.5 text-red-500" aria-hidden="true"
        >*</span
      >
    </label>

    <!-- Input slot -->
    <slot />

    <p v-if="hint && !error" class="mt-1 text-xs text-gray-400">{{ hint }}</p>
    <p v-if="error" role="alert" class="mt-1 text-xs font-medium text-red-500">
      {{ error }}
    </p>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  label: string;
  fieldId: string;
  hint?: string;
  error?: string;
  required?: boolean;
}>();
</script>
