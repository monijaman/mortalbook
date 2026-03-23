<template>
  <div class="container mx-auto">
    <h1 class="text-3xl font-bold mb-8">Users Management</h1>

    <div class="bg-white rounded-lg shadow overflow-hidden">
      <table class="min-w-full divide-y divide-gray-200">
        <thead class="bg-gray-50">
          <tr>
            <th
              class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase"
            >
              Email
            </th>
            <th
              class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase"
            >
              Username
            </th>
            <th
              class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase"
            >
              Admin
            </th>
            <th
              class="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase"
            >
              Actions
            </th>
          </tr>
        </thead>
        <tbody class="bg-white divide-y divide-gray-200">
          <tr v-for="user in users" :key="user.id" class="hover:bg-gray-50">
            <td class="px-6 py-4 text-sm text-gray-900">{{ user.email }}</td>
            <td class="px-6 py-4 text-sm text-gray-900">{{ user.username }}</td>
            <td class="px-6 py-4 text-sm">
              <span
                :class="[
                  'px-2 py-1 rounded',
                  user.is_admin
                    ? 'bg-blue-200 text-blue-800'
                    : 'bg-gray-200 text-gray-800',
                ]"
              >
                {{ user.is_admin ? "Yes" : "No" }}
              </span>
            </td>
            <td class="px-6 py-4 text-sm space-x-2">
              <button
                @click="deleteUser(user.id)"
                class="px-3 py-1 bg-red-500 text-white rounded hover:bg-red-600"
              >
                Delete
              </button>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from "vue";
import { useStore } from "vuex";
import { RootState } from "../store";
import { userService, User } from "../services/api";

const store = useStore<RootState>();
const users = ref<User[]>([]);

const loadUsers = async () => {
  try {
    const data = await userService.getUsers();
    users.value = data.data || [];
  } catch {
    store.dispatch("ui/addNotification", {
      message: "Failed to load users",
      type: "error",
    });
  }
};

const deleteUser = async (id: string) => {
  if (!confirm("Are you sure?")) return;
  try {
    await userService.deleteUser(id);
    users.value = users.value.filter((u) => u.id !== id);
    store.dispatch("ui/addNotification", {
      message: "User deleted successfully",
      type: "success",
    });
  } catch {
    store.dispatch("ui/addNotification", {
      message: "Failed to delete user",
      type: "error",
    });
  }
};

onMounted(loadUsers);
</script>
