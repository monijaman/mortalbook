<template>
  <div class="notifications-wrapper">
    <v-snackbar
      v-for="notification in notifications"
      :key="notification.id"
      :model-value="true"
      :color="snackColor(notification.type)"
      location="top right"
      rounded="lg"
      timeout="3500"
    >
      <v-icon start>{{ snackIcon(notification.type) }}</v-icon>
      {{ notification.message }}
    </v-snackbar>
  </div>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useStore } from "vuex";
import { RootState } from "../store";
import { NotificationType } from "../store/modules/ui";

const store = useStore<RootState>();
const notifications = computed(() => store.state.ui.notifications);

const snackColor = (type: NotificationType): string =>
  ({ info: "info", success: "success", error: "error", warning: "warning" })[
    type
  ] ?? "info";

const snackIcon = (type: NotificationType): string =>
  ({
    info: "mdi-information",
    success: "mdi-check-circle",
    error: "mdi-alert-circle",
    warning: "mdi-alert",
  })[type] ?? "mdi-information";
</script>
