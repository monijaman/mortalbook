import { ActionContext, Module } from "vuex";
import { RootState } from "../index";

export type NotificationType = "info" | "success" | "error" | "warning";

export interface Notification {
  id: number;
  message: string;
  type: NotificationType;
}

export interface UIState {
  sidebarOpen: boolean;
  notifications: Notification[];
}

const ui: Module<UIState, RootState> = {
  namespaced: true,

  state: (): UIState => ({
    sidebarOpen: true,
    notifications: [],
  }),

  getters: {
    notifications: (state: UIState): Notification[] => state.notifications,
    sidebarOpen: (state: UIState): boolean => state.sidebarOpen,
  },

  mutations: {
    TOGGLE_SIDEBAR(state: UIState) {
      state.sidebarOpen = !state.sidebarOpen;
    },
    ADD_NOTIFICATION(state: UIState, notification: Notification) {
      state.notifications.push(notification);
    },
    REMOVE_NOTIFICATION(state: UIState, id: number) {
      state.notifications = state.notifications.filter(
        (n: Notification) => n.id !== id,
      );
    },
  },

  actions: {
    addNotification(
      { commit }: ActionContext<UIState, RootState>,
      { message, type = "info" }: { message: string; type?: NotificationType },
    ) {
      const id = Date.now();
      commit("ADD_NOTIFICATION", { id, message, type });
      setTimeout(() => commit("REMOVE_NOTIFICATION", id), 3000);
    },
    toggleSidebar({ commit }: ActionContext<UIState, RootState>) {
      commit("TOGGLE_SIDEBAR");
    },
  },
};

export default ui;
