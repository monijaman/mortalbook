<template>
  <v-app>
    <template v-if="!isLoginPage">
      <Sidebar />
      <Header />
      <v-main style="background: #f5f5f7">
        <v-container fluid class="pa-6">
          <router-view />
        </v-container>
      </v-main>
    </template>
    <template v-else>
      <router-view />
    </template>
    <Notifications />
  </v-app>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { useRoute } from "vue-router";
import Sidebar from "./components/Sidebar.vue";
import Header from "./components/Header.vue";
import Notifications from "./components/Notifications.vue";

const route = useRoute();
const isLoginPage = computed(() => route.path === "/login");
</script>
