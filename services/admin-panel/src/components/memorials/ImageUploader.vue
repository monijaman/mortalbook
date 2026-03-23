<template>
  <!--
    ImageUploader
    – Accepts a file via drag-and-drop or click.
    – Requests a pre-signed S3 PUT URL from the backend.
    – Uploads directly to S3 (no file proxy through our API).
    – Emits `uploaded` with the resulting public_url when done.
  -->
  <div>
    <!-- Drop zone -->
    <div
      role="button"
      tabindex="0"
      :class="[
        'relative flex flex-col items-center justify-center gap-2 rounded-xl border-2 border-dashed px-4 py-8 text-center transition',
        isDragging
          ? 'border-indigo-400 bg-indigo-50 text-indigo-600'
          : 'border-gray-300 bg-gray-50 text-gray-500 hover:border-indigo-300 hover:bg-indigo-50/30',
        disabled && 'pointer-events-none opacity-50',
      ]"
      aria-label="Upload portrait image"
      @click="openFilePicker"
      @keydown.enter="openFilePicker"
      @keydown.space.prevent="openFilePicker"
      @dragover.prevent="isDragging = true"
      @dragleave.prevent="isDragging = false"
      @drop.prevent="onDrop"
    >
      <!-- Preview -->
      <img
        v-if="preview"
        :src="preview"
        alt="Selected image preview"
        class="mb-2 h-28 w-28 rounded-full object-cover ring-2 ring-indigo-300"
      />
      <div
        v-else
        class="flex h-16 w-16 items-center justify-center rounded-full bg-gray-200 text-3xl"
        aria-hidden="true"
      >
        🖼️
      </div>

      <span class="text-sm font-medium">
        {{ isDragging ? "Drop to upload" : "Click or drag an image here" }}
      </span>
      <span class="text-xs text-gray-400">PNG, JPG, WEBP — max 5 MB</span>

      <!-- Hidden native input -->
      <input
        ref="fileInput"
        type="file"
        class="sr-only"
        accept="image/png,image/jpeg,image/webp"
        aria-hidden="true"
        @change="onFileSelected"
      />
    </div>

    <!-- Progress bar -->
    <div v-if="uploading" class="mt-3">
      <div class="mb-1 flex justify-between text-xs text-gray-500">
        <span>Uploading…</span>
        <span>{{ progress }}%</span>
      </div>
      <div class="h-2 w-full overflow-hidden rounded-full bg-gray-200">
        <div
          class="h-full rounded-full bg-indigo-500 transition-all duration-300"
          :style="{ width: `${progress}%` }"
        />
      </div>
    </div>

    <!-- Error -->
    <p v-if="error" role="alert" class="mt-2 text-xs font-medium text-red-500">
      {{ error }}
    </p>

    <!-- Success -->
    <p
      v-if="uploadedUrl && !uploading"
      class="mt-2 flex items-center gap-1 text-xs text-green-600"
    >
      <svg
        class="h-3.5 w-3.5"
        viewBox="0 0 20 20"
        fill="currentColor"
        aria-hidden="true"
      >
        <path
          fill-rule="evenodd"
          d="M10 18a8 8 0 1 0 0-16 8 8 0 0 0 0 16Zm3.857-9.809a.75.75 0 0 0-1.214-.882l-3.483 4.79-1.88-1.88a.75.75 0 1 0-1.06 1.061l2.5 2.5a.75.75 0 0 0 1.137-.089l4-5.5Z"
          clip-rule="evenodd"
        />
      </svg>
      Image uploaded successfully.
    </p>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";
import { memorialService } from "../../services/api";

// ── Props & Emits ─────────────────────────────────────────────────────────────
const props = defineProps<{
  /** The memorial id — required for the presigned-url endpoint */
  memorialId: string;
  /** Disable while the parent form is submitting */
  disabled?: boolean;
}>();

const emit = defineEmits<{
  /** Emitted with the public S3 URL once the upload completes */
  (e: "uploaded", url: string): void;
}>();

// ── State ─────────────────────────────────────────────────────────────────────
const fileInput = ref<HTMLInputElement | null>(null);
const isDragging = ref<boolean>(false);
const preview = ref<string | null>(null);
const uploading = ref<boolean>(false);
const progress = ref<number>(0);
const error = ref<string>("");
const uploadedUrl = ref<string>("");

const MAX_SIZE_BYTES = 5 * 1024 * 1024; // 5 MB
const ALLOWED_TYPES = ["image/png", "image/jpeg", "image/webp"];

// ── Handlers ──────────────────────────────────────────────────────────────────
function openFilePicker(): void {
  if (!props.disabled) fileInput.value?.click();
}

function onDrop(event: DragEvent): void {
  isDragging.value = false;
  const file = event.dataTransfer?.files?.[0];
  if (file) handleFile(file);
}

function onFileSelected(event: Event): void {
  const input = event.target as HTMLInputElement;
  const file = input.files?.[0];
  if (file) handleFile(file);
  // Reset so re-selecting the same file triggers onChange again
  input.value = "";
}

async function handleFile(file: File): Promise<void> {
  error.value = "";
  uploadedUrl.value = "";

  // ── Client-side validation ────────────────────────────────────────────────
  if (!ALLOWED_TYPES.includes(file.type)) {
    error.value =
      "Unsupported file type. Please upload a PNG, JPG, or WEBP image.";
    return;
  }
  if (file.size > MAX_SIZE_BYTES) {
    error.value = "File is too large. Maximum size is 5 MB.";
    return;
  }

  // Local preview
  preview.value = URL.createObjectURL(file);
  uploading.value = true;
  progress.value = 0;

  try {
    // 1 — Get pre-signed URL from the backend
    const { upload_url, public_url } =
      await memorialService.getPresignedUploadUrl(
        props.memorialId,
        file.name,
        file.type,
      );

    // 2 — PUT file directly to S3
    await memorialService.uploadFileToS3(upload_url, file, (pct: number) => {
      progress.value = pct;
    });

    uploadedUrl.value = public_url;
    emit("uploaded", public_url);
  } catch (err) {
    console.error("[ImageUploader] upload failed", err);
    error.value = "Upload failed. Please try again.";
    preview.value = null;
  } finally {
    uploading.value = false;
  }
}
</script>
