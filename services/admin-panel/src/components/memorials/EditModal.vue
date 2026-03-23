<template>
  <v-dialog
    :model-value="isOpen"
    max-width="700"
    @update:model-value="!$event && requestClose()"
  >
    <v-card rounded="xl">
      <v-card-title class="d-flex align-center justify-space-between pa-5">
        <span class="text-subtitle-1 font-weight-bold">{{
          isCreate ? "New Memorial" : "Edit Memorial"
        }}</span>
        <v-btn icon variant="text" size="small" @click="requestClose">
          <v-icon>mdi-close</v-icon>
        </v-btn>
      </v-card-title>

      <v-divider />

      <v-card-text class="pa-5" v-if="draft">
        <v-tabs v-model="activeTab" color="primary" class="mb-4">
          <v-tab v-for="tab in TABS" :key="tab.key" :value="tab.key">{{
            tab.label
          }}</v-tab>
        </v-tabs>

        <v-window v-model="activeTab">
          <!-- Details tab -->
          <v-window-item value="details">
            <v-row dense>
              <v-col cols="12">
                <v-text-field
                  v-model.trim="draft.name"
                  label="Full name *"
                  variant="outlined"
                  density="compact"
                  :error-messages="errors.name"
                  @input="errors.name = ''"
                />
              </v-col>
              <v-col cols="6">
                <v-text-field
                  v-model="draft.date_of_birth"
                  label="Date of birth"
                  type="date"
                  variant="outlined"
                  density="compact"
                  :error-messages="errors.date_of_birth"
                  @input="errors.date_of_birth = ''"
                />
              </v-col>
              <v-col cols="6">
                <v-text-field
                  v-model="draft.date_of_death"
                  label="Date of death"
                  type="date"
                  variant="outlined"
                  density="compact"
                  :error-messages="errors.date_of_death"
                  @input="errors.date_of_death = ''"
                />
              </v-col>
              <v-col cols="12">
                <v-textarea
                  v-model="draft.biography"
                  label="Biography"
                  variant="outlined"
                  density="compact"
                  rows="4"
                  auto-grow
                />
              </v-col>
              <v-col cols="6">
                <v-select
                  v-model="draft.status"
                  label="Status"
                  variant="outlined"
                  density="compact"
                  :items="['active', 'inactive', 'pending', 'deleted']"
                />
              </v-col>
            </v-row>
          </v-window-item>

          <!-- Media tab -->
          <v-window-item value="media">
            <v-row dense>
              <v-col cols="12">
                <v-text-field
                  v-model="draft.image_url"
                  label="Image URL"
                  prepend-inner-icon="mdi-image-outline"
                  variant="outlined"
                  density="compact"
                  hint="Direct URL to a portrait image"
                  persistent-hint
                />
              </v-col>
              <v-col cols="12">
                <v-text-field
                  v-model="youtubeRaw"
                  label="YouTube URL or video ID"
                  prepend-inner-icon="mdi-youtube"
                  variant="outlined"
                  density="compact"
                  :error-messages="errors.youtube"
                  :append-inner-icon="youtubeRaw ? 'mdi-close' : ''"
                  @input="onYoutubeInput"
                  @click:append-inner="clearYoutube"
                />
              </v-col>
            </v-row>
          </v-window-item>
        </v-window>
      </v-card-text>

      <v-divider />

      <v-card-actions class="pa-4">
        <v-spacer />
        <v-btn variant="outlined" :disabled="saving" @click="requestClose"
          >Cancel</v-btn
        >
        <v-btn
          color="primary"
          variant="flat"
          :loading="saving"
          @click="submit"
          >{{ isCreate ? "Create memorial" : "Save changes" }}</v-btn
        >
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { ref, reactive, watch, computed } from "vue";
import type { Memorial } from "../../store/modules/memorials";

type DraftMemorial = Partial<Memorial>;

const props = defineProps<{
  isOpen: boolean;
  person: Memorial | null;
  saving?: boolean;
}>();

const emit = defineEmits<{
  (e: "close"): void;
  (e: "save", id: string | null, payload: DraftMemorial): void;
}>();

const isCreate = computed(() => !props.person?.id);

const TABS = [
  { key: "details", label: "Details" },
  { key: "media", label: "Media" },
];

const activeTab = ref<string>("details");
const draft = ref<DraftMemorial | null>(null);
const youtubeRaw = ref<string>("");
const errors = reactive<Record<string, string>>({});

watch(
  () => [props.person, props.isOpen],
  () => {
    if (!props.isOpen) return;
    const p = props.person;
    draft.value = p
      ? { ...p }
      : {
          name: "",
          date_of_birth: null,
          date_of_death: null,
          biography: "",
          status: "active",
          youtube_video_id: null,
        };
    youtubeRaw.value = draft.value.youtube_video_id ?? "";
    Object.keys(errors).forEach((k) => delete errors[k]);
    activeTab.value = "details";
  },
  { immediate: true },
);

function extractYoutubeId(url: string): string | null {
  if (!url) return null;
  const patterns = [
    /(?:youtube\.com\/watch\?v=|youtu\.be\/|youtube\.com\/embed\/)([A-Za-z0-9_-]{11})/,
    /^([A-Za-z0-9_-]{11})$/,
  ];
  for (const re of patterns) {
    const m = url.match(re);
    if (m) return m[1];
  }
  return null;
}

function onYoutubeInput(): void {
  errors["youtube"] = "";
  const id = extractYoutubeId(youtubeRaw.value);
  if (youtubeRaw.value && !id)
    errors["youtube"] = "Could not extract a YouTube video ID from this URL.";
  if (draft.value) draft.value.youtube_video_id = id ?? null;
}

function clearYoutube(): void {
  youtubeRaw.value = "";
  if (draft.value) draft.value.youtube_video_id = null;
  errors["youtube"] = "";
}

function validate(): boolean {
  Object.keys(errors).forEach((k) => delete errors[k]);
  let valid = true;
  if (!draft.value?.name) {
    errors["name"] = "Name is required.";
    valid = false;
  }
  if (draft.value?.date_of_birth && draft.value?.date_of_death) {
    if (
      new Date(draft.value.date_of_birth) >= new Date(draft.value.date_of_death)
    ) {
      errors["date_of_death"] = "Date of death must be after date of birth.";
      valid = false;
    }
  }
  if (errors["youtube"]) valid = false;
  if (!valid && errors["name"]) activeTab.value = "details";
  else if (!valid && errors["youtube"]) activeTab.value = "media";
  return valid;
}

function submit(): void {
  if (!validate() || !draft.value) return;
  emit("save", draft.value.id ?? null, { ...draft.value });
}

function requestClose(): void {
  if (props.saving) return;
  emit("close");
}
</script>
