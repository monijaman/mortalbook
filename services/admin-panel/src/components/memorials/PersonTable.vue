<template>
  <v-card rounded="xl" elevation="0" border>
    <!-- Loading -->
    <div v-if="loading" class="d-flex justify-center align-center py-12">
      <v-progress-circular indeterminate color="primary" />
      <span class="ml-4 text-medium-emphasis">Loading memorials…</span>
    </div>

    <!-- Empty -->
    <div
      v-else-if="people.length === 0"
      class="d-flex flex-column align-center justify-center py-12 text-medium-emphasis"
    >
      <v-icon size="56" color="grey-lighten-1">mdi-flower-outline</v-icon>
      <p class="mt-4 text-subtitle-1 font-weight-medium">No records found</p>
      <p class="text-body-2">Try adjusting your search terms.</p>
    </div>

    <!-- Table -->
    <v-table v-else hover>
      <thead>
        <tr>
          <th class="text-caption font-weight-bold text-uppercase">Photo</th>
          <th class="text-caption font-weight-bold text-uppercase">Name</th>
          <th class="text-caption font-weight-bold text-uppercase">
            Born / Died
          </th>
          <th class="text-caption font-weight-bold text-uppercase">Status</th>
          <th class="text-caption font-weight-bold text-uppercase text-right">
            Actions
          </th>
        </tr>
      </thead>
      <tbody>
        <tr v-for="person in people" :key="person.id">
          <td>
            <v-avatar size="36" color="grey-lighten-3">
              <v-img v-if="person.image_url" :src="person.image_url" cover />
              <v-icon v-else color="grey">mdi-account</v-icon>
            </v-avatar>
          </td>
          <td>
            <div class="font-weight-medium">{{ person.name }}</div>
            <div
              v-if="person.occupation"
              class="text-caption text-medium-emphasis"
            >
              {{ person.occupation }}
            </div>
          </td>
          <td class="text-body-2 text-medium-emphasis">
            {{ formatYear(person.date_of_birth) }} &ndash;
            {{ formatYear(person.date_of_death) }}
          </td>
          <td>
            <v-chip
              :color="statusColor(person.status)"
              variant="tonal"
              size="small"
              >{{ person.status }}</v-chip
            >
          </td>
          <td class="text-right">
            <v-btn
              icon
              size="small"
              variant="text"
              :title="person.status === 'active' ? 'Deactivate' : 'Activate'"
              @click="$emit('toggle-status', person)"
            >
              <v-icon size="18">{{
                person.status === "active"
                  ? "mdi-pause-circle-outline"
                  : "mdi-play-circle-outline"
              }}</v-icon>
            </v-btn>
            <v-btn
              icon
              size="small"
              variant="text"
              color="primary"
              title="Edit"
              @click="$emit('edit', person)"
            >
              <v-icon size="18">mdi-pencil-outline</v-icon>
            </v-btn>
            <v-btn
              icon
              size="small"
              variant="text"
              color="error"
              title="Delete"
              @click="$emit('delete', person)"
            >
              <v-icon size="18">mdi-trash-can-outline</v-icon>
            </v-btn>
          </td>
        </tr>
      </tbody>
    </v-table>
  </v-card>
</template>

<script setup lang="ts">
import { Memorial } from "../../store/modules/memorials";

defineProps<{
  people: Memorial[];
  loading?: boolean;
}>();
defineEmits<{
  (e: "edit", person: Memorial): void;
  (e: "delete", person: Memorial): void;
  (e: "toggle-status", person: Memorial): void;
}>();

function formatYear(dateString: string | null | undefined): string {
  if (!dateString) return "?";
  return String(new Date(dateString).getUTCFullYear());
}

const STATUS_COLOR: Record<string, string> = {
  active: "success",
  inactive: "warning",
  pending: "info",
  deleted: "error",
};
function statusColor(status: string): string {
  return STATUS_COLOR[status] ?? "grey";
}
</script>
