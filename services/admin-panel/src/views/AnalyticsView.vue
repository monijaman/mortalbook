<template>
  <div class="container mx-auto">
    <h1 class="text-3xl font-bold mb-8">Analytics</h1>

    <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
      <div class="bg-white p-6 rounded-lg shadow">
        <h2 class="text-xl font-bold mb-4">Total Events</h2>
        <div class="text-4xl font-bold text-blue-500">
          {{ analytics.total_events }}
        </div>
      </div>

      <div class="bg-white p-6 rounded-lg shadow">
        <h2 class="text-xl font-bold mb-4">Period</h2>
        <div class="text-lg text-gray-600">
          Last {{ analytics.period_days }} days
        </div>
      </div>
    </div>

    <div class="mt-8 bg-white p-6 rounded-lg shadow">
      <h2 class="text-xl font-bold mb-4">Recent Events</h2>
      <div class="space-y-2">
        <div
          v-for="event in events"
          :key="event.id"
          class="flex justify-between py-2 border-b"
        >
          <span>{{ event.event_type }}</span>
          <span class="text-gray-600">{{
            new Date(event.created_at).toLocaleString()
          }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useStore } from "vuex";
import { RootState } from "../store";
import {
  analyticsService,
  AnalyticsDashboard,
  AnalyticsEvent,
} from "../services/api";

const store = useStore<RootState>();
const analytics = ref<AnalyticsDashboard>({ total_events: 0, period_days: 30 });
const events = ref<AnalyticsEvent[]>([]);

const loadAnalytics = async () => {
  try {
    analytics.value = await analyticsService.getDashboard();
    const eventsData = await analyticsService.getEvents();
    events.value = eventsData.data || [];
  } catch {
    store.dispatch("ui/addNotification", {
      message: "Failed to load analytics",
      type: "error",
    });
  }
};

onMounted(loadAnalytics);
</script>
