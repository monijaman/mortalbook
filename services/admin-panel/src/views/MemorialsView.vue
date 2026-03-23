<template>
  <div>
    <!-- Page header -->
    <div class="d-flex flex-wrap align-center justify-space-between gap-3 mb-4">
      <div>
        <div class="text-h6 font-weight-bold">Memorials</div>
        <div class="text-caption text-medium-emphasis">
          {{ total }} record{{ total !== 1 ? "s" : "" }} total
        </div>
      </div>
      <div class="d-flex align-center gap-3">
        <SearchBar
          v-model="searchQuery"
          placeholder="Search by name…"
          @search="(q: string) => store.dispatch('memorials/search', q)"
        />
        <v-btn
          color="primary"
          variant="flat"
          prepend-icon="mdi-plus"
          @click="store.dispatch('memorials/openCreate')"
        >
          New memorial
        </v-btn>
      </div>
    </div>

    <!-- Active search chip -->
    <div v-if="searchQuery" class="d-flex align-center gap-2 mb-3">
      <span class="text-body-2 text-medium-emphasis">Results for</span>
      <v-chip
        size="small"
        color="primary"
        variant="tonal"
        closable
        @click:close="clearSearch"
      >
        {{ searchQuery }}
      </v-chip>
    </div>

    <!-- Table -->
    <PersonTable
      :people="items"
      :loading="isLoading"
      @edit="(p) => store.dispatch('memorials/openEdit', p)"
      @delete="confirmDelete"
      @toggle-status="toggleStatus"
    />

    <!-- Pagination -->
    <Pagination
      :current="currentPage"
      :total-pages="totalPages"
      :total="total"
      @page="(p: number) => store.dispatch('memorials/goToPage', p)"
    />

    <!-- Edit / Create modal -->
    <EditModal
      :is-open="isEditModalOpen"
      :person="selectedPerson"
      :saving="isSaving"
      @close="store.dispatch('memorials/closeEdit')"
      @save="handleSave"
    />

    <!-- Delete confirmation -->
    <v-dialog v-model="deleteDialog" max-width="400" rounded="xl">
      <v-card rounded="xl">
        <v-card-text class="pa-6">
          <div class="d-flex align-center mb-4">
            <v-avatar color="error" variant="tonal" size="44" class="mr-4">
              <v-icon color="error">mdi-trash-can-outline</v-icon>
            </v-avatar>
            <div>
              <div class="text-subtitle-1 font-weight-bold">
                Delete memorial?
              </div>
              <div class="text-body-2 text-medium-emphasis">
                <strong>{{ deleteTarget?.name }}</strong> will be permanently
                removed.
              </div>
            </div>
          </div>
        </v-card-text>
        <v-card-actions class="px-6 pb-4">
          <v-spacer />
          <v-btn variant="outlined" @click="deleteDialog = false">Cancel</v-btn>
          <v-btn color="error" variant="flat" @click="executeDelete"
            >Yes, delete</v-btn
          >
        </v-card-actions>
      </v-card>
    </v-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from "vue";
import { useStore } from "vuex";
import { RootState } from "../store";
import { Memorial } from "../store/modules/memorials";
import SearchBar from "../components/memorials/SearchBar.vue";
import PersonTable from "../components/memorials/PersonTable.vue";
import Pagination from "../components/memorials/Pagination.vue";
import EditModal from "../components/memorials/EditModal.vue";

const store = useStore<RootState>();
const deleteTarget = ref<Memorial | null>(null);
const deleteDialog = ref(false);

const items = computed(() => store.state.memorials.items);
const total = computed(() => store.state.memorials.total);
const isLoading = computed(() => store.state.memorials.isLoading);
const isSaving = computed(() => store.state.memorials.isSaving);
const searchQuery = computed(() => store.state.memorials.searchQuery);
const currentPage = computed(() => store.state.memorials.currentPage);
const totalPages = computed(() => store.getters["memorials/totalPages"]);
const selectedPerson = computed(() => store.state.memorials.selectedPerson);
const isEditModalOpen = computed(() => store.state.memorials.isEditModalOpen);

function confirmDelete(person: Memorial) {
  deleteTarget.value = person;
  deleteDialog.value = true;
}

async function executeDelete() {
  if (!deleteTarget.value) return;
  await store.dispatch("memorials/deletePerson", deleteTarget.value.id);
  deleteDialog.value = false;
  deleteTarget.value = null;
}

function toggleStatus(person: Memorial) {
  store.dispatch("memorials/updateStatus", {
    id: person.id,
    status: person.status === "active" ? "inactive" : "active",
  });
}

function handleSave(id: string | null, payload: Partial<Memorial>) {
  if (id) {
    store.dispatch("memorials/saveEdit", { id, payload });
  } else {
    store.dispatch("memorials/createMemorial", payload);
  }
}

function clearSearch() {
  store.dispatch("memorials/search", "");
}

onMounted(() => store.dispatch("memorials/fetchMemorials"));
</script>
