<template>
  <v-app-bar elevation="0" color="white" border="b">
    <v-app-bar-title>
      <span class="text-subtitle-1 font-weight-bold text-medium-emphasis">{{
        pageTitle
      }}</span>
    </v-app-bar-title>

    <template #append>
      <v-chip color="success" variant="tonal" size="small" class="mr-3">
        <v-icon start size="10">mdi-circle</v-icon>
        Live
      </v-chip>

      <v-avatar color="primary" size="36" class="mr-3" style="cursor: pointer">
        <span class="text-caption font-weight-bold text-white">AD</span>
      </v-avatar>

      <v-btn
        icon="mdi-logout-variant"
        variant="text"
        color="error"
        size="small"
        class="mr-2"
        title="Logout"
        @click="logout"
      />
    </template>
  </v-app-bar>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useRoute, useRouter } from "vue-router";
import { useStore } from "vuex";
import { RootState } from "../store";

const route = useRoute();
const router = useRouter();
const store = useStore<RootState>();

const pageTitle = computed(() => {
  const map: Record<string, string> = {
    "/": "Dashboard",
    "/memorials": "Memorials",
    "/analytics": "Analytics",
  };
  return map[route.path] ?? "Admin Panel";
});

const logout = () => {
  store.dispatch("auth/logout");
  router.push("/login");
};
</script>
