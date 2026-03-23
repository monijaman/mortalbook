<template>
  <v-navigation-drawer v-model="drawer" :rail="rail" permanent color="#1a1a2e">
    <!-- Brand -->
    <v-list-item
      prepend-icon="mdi-grave-stone"
      title="Mortalbook"
      nav
      class="py-4"
      style="color: white"
    >
      <template #append>
        <v-btn
          :icon="rail ? 'mdi-chevron-right' : 'mdi-chevron-left'"
          variant="text"
          color="white"
          size="small"
          @click="rail = !rail"
        />
      </template>
    </v-list-item>

    <v-divider style="border-color: rgba(255, 255, 255, 0.1)" class="mb-2" />

    <v-list density="compact" nav>
      <v-list-item
        v-for="item in navItems"
        :key="item.to"
        :prepend-icon="item.icon"
        :title="item.title"
        :to="item.to"
        rounded="lg"
        color="primary"
        style="color: rgba(255, 255, 255, 0.8)"
        class="mb-1"
      />
    </v-list>

    <template #append>
      <v-divider style="border-color: rgba(255, 255, 255, 0.1)" class="mb-2" />
      <v-list density="compact" nav>
        <v-list-item
          prepend-icon="mdi-logout"
          title="Logout"
          rounded="lg"
          style="color: rgba(255, 255, 255, 0.6)"
          @click="logout"
        />
      </v-list>
      <div
        v-if="!rail"
        class="text-center text-caption pa-3"
        style="color: rgba(255, 255, 255, 0.3)"
      >
        v1.0.0
      </div>
    </template>
  </v-navigation-drawer>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { useRouter } from "vue-router";
import { useStore } from "vuex";
import { RootState } from "../store";

const router = useRouter();
const store = useStore<RootState>();
const drawer = ref(true);
const rail = ref(false);

interface NavItem {
  title: string;
  icon: string;
  to: string;
}
const navItems: NavItem[] = [
  { title: "Dashboard", icon: "mdi-view-dashboard-outline", to: "/" },
  { title: "Memorials", icon: "mdi-flower-outline", to: "/memorials" },
  { title: "Analytics", icon: "mdi-chart-line", to: "/analytics" },
];

const logout = () => {
  store.dispatch("auth/logout");
  router.push("/login");
};
</script>
