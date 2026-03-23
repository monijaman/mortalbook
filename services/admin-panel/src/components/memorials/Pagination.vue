<template>
  <div
    v-if="totalPages > 1"
    class="d-flex align-center justify-space-between pt-2"
  >
    <span class="text-caption text-medium-emphasis">
      Page <strong>{{ current }}</strong> of
      <strong>{{ totalPages }}</strong> &nbsp;&middot;&nbsp;
      <strong>{{ total }}</strong> records
    </span>
    <v-pagination
      :model-value="current"
      :length="totalPages"
      :total-visible="7"
      density="compact"
      rounded
      @update:model-value="$emit('page', $event)"
    />
  </div>
</template>

<script setup lang="ts">
defineProps<{
  current: number;
  totalPages: number;
  total?: number;
}>();
defineEmits<{ (e: "page", page: number): void }>();
</script>
