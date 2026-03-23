<template>
  <v-app
    style="
      background: linear-gradient(
        135deg,
        #1a1a2e 0%,
        #16213e 50%,
        #0f3460 100%
      );
    "
  >
    <v-main>
      <v-container fill-height fluid>
        <v-row align="center" justify="center">
          <v-col cols="12" sm="8" md="5" lg="4">
            <!-- Logo / Brand -->
            <div class="text-center mb-8">
              <v-icon size="56" color="white" class="mb-3"
                >mdi-grave-stone</v-icon
              >
              <div class="text-h4 font-weight-bold text-white">Mortalbook</div>
              <div
                class="text-body-2 mt-1"
                style="color: rgba(255, 255, 255, 0.6)"
              >
                Admin Portal
              </div>
            </div>

            <v-card rounded="xl" elevation="12" class="pa-2">
              <v-card-text class="pa-8">
                <div class="text-h6 font-weight-bold mb-6 text-center">
                  Sign in to continue
                </div>

                <v-form @submit.prevent="handleLogin">
                  <v-text-field
                    v-model="email"
                    label="Email address"
                    prepend-inner-icon="mdi-email-outline"
                    type="email"
                    variant="outlined"
                    required
                    class="mb-2"
                  />

                  <v-text-field
                    v-model="password"
                    label="Password"
                    prepend-inner-icon="mdi-lock-outline"
                    :append-inner-icon="showPass ? 'mdi-eye-off' : 'mdi-eye'"
                    :type="showPass ? 'text' : 'password'"
                    variant="outlined"
                    required
                    class="mb-4"
                    @click:append-inner="showPass = !showPass"
                  />

                  <v-btn
                    type="submit"
                    color="primary"
                    block
                    size="large"
                    rounded="lg"
                    :loading="isLoading"
                    class="text-none font-weight-bold"
                  >
                    Sign in
                  </v-btn>
                </v-form>
              </v-card-text>
            </v-card>

            <div
              class="text-center mt-6 text-body-2"
              style="color: rgba(255, 255, 255, 0.5)"
            >
              Mortalbook Admin &mdash; Authorised personnel only
            </div>
          </v-col>
        </v-row>
      </v-container>
    </v-main>
  </v-app>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import { useRouter } from "vue-router";
import { useStore } from "vuex";
import { RootState } from "../store";

const router = useRouter();
const store = useStore<RootState>();

const email = ref("");
const password = ref("");
const showPass = ref(false);
const isLoading = computed(() => store.state.auth.isLoading);

const handleLogin = async () => {
  const success = await store.dispatch("auth/login", {
    email: email.value,
    password: password.value,
  });
  if (success) {
    store.dispatch("ui/addNotification", {
      message: "Welcome back!",
      type: "success",
    });
    router.push("/");
  } else {
    store.dispatch("ui/addNotification", {
      message: "Invalid credentials. Please try again.",
      type: "error",
    });
  }
};
</script>
