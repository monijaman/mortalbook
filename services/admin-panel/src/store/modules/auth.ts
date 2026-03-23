import axios from "axios";
import { ActionContext, Module } from "vuex";
import { RootState } from "../index";

const API_URL = import.meta.env.VITE_API_URL || "http://localhost:5002";
const _client = axios.create({ baseURL: API_URL });

export interface AuthState {
  token: string;
  refreshToken: string;
  isLoading: boolean;
}

const auth: Module<AuthState, RootState> = {
  namespaced: true,

  state: (): AuthState => ({
    token: localStorage.getItem("token") ?? "",
    refreshToken: localStorage.getItem("refreshToken") ?? "",
    isLoading: false,
  }),

  getters: {
    isAuthenticated: (state: AuthState): boolean => !!state.token,
    token: (state: AuthState): string => state.token,
  },

  mutations: {
    SET_TOKEN(state: AuthState, token: string) {
      state.token = token;
      localStorage.setItem("token", token);
    },
    SET_REFRESH_TOKEN(state: AuthState, token: string) {
      state.refreshToken = token;
      localStorage.setItem("refreshToken", token);
    },
    SET_LOADING(state: AuthState, loading: boolean) {
      state.isLoading = loading;
    },
    CLEAR_AUTH(state: AuthState) {
      state.token = "";
      state.refreshToken = "";
      localStorage.removeItem("token");
      localStorage.removeItem("refreshToken");
    },
  },

  actions: {
    async login(
      { commit }: ActionContext<AuthState, RootState>,
      { email, password }: { email: string; password: string },
    ): Promise<boolean> {
      commit("SET_LOADING", true);
      try {
        const res = await _client.post("/api/v1/auth/login", {
          email,
          password,
        });
        commit("SET_TOKEN", res.data.access_token);
        commit("SET_REFRESH_TOKEN", res.data.refresh_token);
        return true;
      } catch {
        return false;
      } finally {
        commit("SET_LOADING", false);
      }
    },

    logout({ commit }: ActionContext<AuthState, RootState>) {
      commit("CLEAR_AUTH");
    },
  },
};

export default auth;
