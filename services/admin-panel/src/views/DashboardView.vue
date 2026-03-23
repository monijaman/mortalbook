<template>
  <div>
    <!-- Stat cards -->
    <v-row class="mb-4">
      <v-col v-for="stat in stats" :key="stat.label" cols="12" sm="6" lg="3">
        <v-card rounded="xl" elevation="0" border>
          <v-card-text class="d-flex align-center pa-5">
            <v-avatar
              :color="stat.color"
              variant="tonal"
              size="52"
              class="mr-4"
            >
              <v-icon :color="stat.color" size="26">{{ stat.icon }}</v-icon>
            </v-avatar>
            <div>
              <div class="text-h5 font-weight-bold">{{ stat.value }}</div>
              <div class="text-caption text-medium-emphasis">
                {{ stat.label }}
              </div>
            </div>
          </v-card-text>
        </v-card>
      </v-col>
    </v-row>

    <v-row>
      <!-- Recent activity -->
      <v-col cols="12" lg="7">
        <v-card rounded="xl" elevation="0" border>
          <v-card-title class="pa-5 pb-2 text-subtitle-1 font-weight-bold">
            <v-icon class="mr-2" color="primary">mdi-history</v-icon>Recent
            Activity
          </v-card-title>
          <v-list lines="two" class="px-2 pb-2">
            <v-list-item
              v-for="activity in activities"
              :key="activity.title"
              :prepend-icon="activity.icon"
              :title="activity.title"
              :subtitle="activity.time"
              rounded="lg"
              class="mb-1"
            >
              <template #prepend>
                <v-avatar :color="activity.color" variant="tonal" size="40">
                  <v-icon :color="activity.color" size="18">{{
                    activity.icon
                  }}</v-icon>
                </v-avatar>
              </template>
            </v-list-item>
          </v-list>
        </v-card>
      </v-col>

      <!-- System status -->
      <v-col cols="12" lg="5">
        <v-card rounded="xl" elevation="0" border>
          <v-card-title class="pa-5 pb-2 text-subtitle-1 font-weight-bold">
            <v-icon class="mr-2" color="success">mdi-server-network</v-icon
            >System Status
          </v-card-title>
          <v-list class="px-2 pb-2">
            <v-list-item
              v-for="svc in services"
              :key="svc.name"
              :title="svc.name"
              rounded="lg"
              class="mb-1"
            >
              <template #append>
                <v-chip
                  :color="svc.ok ? 'success' : 'error'"
                  variant="tonal"
                  size="small"
                >
                  <v-icon start size="10">mdi-circle</v-icon>
                  {{ svc.ok ? "Healthy" : "Down" }}
                </v-chip>
              </template>
            </v-list-item>
          </v-list>
        </v-card>
      </v-col>
    </v-row>
  </div>
</template>

<script setup lang="ts">
interface Stat {
  label: string;
  value: string;
  color: string;
  icon: string;
}
interface Activity {
  title: string;
  time: string;
  icon: string;
  color: string;
}
interface Service {
  name: string;
  ok: boolean;
}

const stats: Stat[] = [
  {
    label: "Total Memorials",
    value: "150",
    color: "primary",
    icon: "mdi-flower-outline",
  },
  {
    label: "Total Users",
    value: "1,245",
    color: "success",
    icon: "mdi-account-group-outline",
  },
  {
    label: "Comments",
    value: "328",
    color: "secondary",
    icon: "mdi-comment-outline",
  },
  {
    label: "Pending Reviews",
    value: "42",
    color: "warning",
    icon: "mdi-clock-outline",
  },
];

const activities: Activity[] = [
  {
    title: "New Memorial Created",
    time: "2 hours ago",
    icon: "mdi-flower-plus-outline",
    color: "primary",
  },
  {
    title: "User Registered",
    time: "5 hours ago",
    icon: "mdi-account-plus-outline",
    color: "success",
  },
  {
    title: "Review Needed",
    time: "1 day ago",
    icon: "mdi-alert-outline",
    color: "warning",
  },
];

const services: Service[] = [
  { name: "Admin API", ok: true },
  { name: "Database", ok: true },
  { name: "Cache", ok: true },
  { name: "Kafka", ok: false },
];
</script>
