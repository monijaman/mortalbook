import { ActionContext, Module } from "vuex";
import { memorialService } from "../../services/api";
import { RootState } from "../index";

const PAGE_SIZE = 15;

export interface Memorial {
  id: string;
  name: string;
  date_of_birth: string | null;
  date_of_death: string | null;
  biography: string;
  status: string;
  image_url?: string;
  youtube_video_id?: string | null;
  created_by?: string;
  created_at?: string;
  updated_at?: string;
}

export interface MemorialsState {
  items: Memorial[];
  total: number;
  currentPage: number;
  searchQuery: string;
  isLoading: boolean;
  isSaving: boolean;
  selectedPerson: Memorial | null;
  isEditModalOpen: boolean;
}

const memorials: Module<MemorialsState, RootState> = {
  namespaced: true,

  state: (): MemorialsState => ({
    items: [],
    total: 0,
    currentPage: 1,
    searchQuery: "",
    isLoading: false,
    isSaving: false,
    selectedPerson: null,
    isEditModalOpen: false,
  }),

  getters: {
    totalPages: (state: MemorialsState): number =>
      state.total ? Math.ceil(state.total / PAGE_SIZE) : 1,
    skip: (state: MemorialsState): number =>
      (state.currentPage - 1) * PAGE_SIZE,
  },

  mutations: {
    SET_ITEMS(state: MemorialsState, items: Memorial[]) {
      state.items = items;
    },
    SET_TOTAL(state: MemorialsState, total: number) {
      state.total = total;
    },
    SET_PAGE(state: MemorialsState, page: number) {
      state.currentPage = page;
    },
    SET_SEARCH(state: MemorialsState, query: string) {
      state.searchQuery = query;
    },
    SET_LOADING(state: MemorialsState, loading: boolean) {
      state.isLoading = loading;
    },
    SET_SAVING(state: MemorialsState, saving: boolean) {
      state.isSaving = saving;
    },
    SET_SELECTED(state: MemorialsState, person: Memorial | null) {
      state.selectedPerson = person;
    },
    SET_MODAL_OPEN(state: MemorialsState, open: boolean) {
      state.isEditModalOpen = open;
    },
    PREPEND_ITEM(state: MemorialsState, item: Memorial) {
      state.items.unshift(item);
      state.total += 1;
    },
    UPDATE_ITEM(state: MemorialsState, updated: Memorial) {
      const idx = state.items.findIndex((m: Memorial) => m.id === updated.id);
      if (idx !== -1) state.items[idx] = { ...state.items[idx], ...updated };
    },
    REMOVE_ITEM(state: MemorialsState, id: string) {
      state.items = state.items.filter((m: Memorial) => m.id !== id);
      state.total = Math.max(0, state.total - 1);
    },
    UPDATE_ITEM_STATUS(
      state: MemorialsState,
      { id, status }: { id: string; status: string },
    ) {
      const idx = state.items.findIndex((m: Memorial) => m.id === id);
      if (idx !== -1) state.items[idx].status = status;
    },
  },

  actions: {
    async fetchMemorials({
      state,
      commit,
      getters,
      dispatch,
    }: ActionContext<MemorialsState, RootState>) {
      commit("SET_LOADING", true);
      try {
        const data = await memorialService.getMemorials(
          getters.skip,
          PAGE_SIZE,
          state.searchQuery,
        );
        commit("SET_ITEMS", data.data ?? []);
        commit("SET_TOTAL", data.total ?? 0);
      } catch {
        dispatch(
          "ui/addNotification",
          { message: "Failed to load memorials.", type: "error" },
          { root: true },
        );
      } finally {
        commit("SET_LOADING", false);
      }
    },

    search(
      { commit, dispatch }: ActionContext<MemorialsState, RootState>,
      query: string,
    ) {
      commit("SET_SEARCH", query);
      commit("SET_PAGE", 1);
      dispatch("fetchMemorials");
    },

    goToPage(
      { commit, dispatch, getters }: ActionContext<MemorialsState, RootState>,
      page: number,
    ) {
      if (page < 1 || page > getters.totalPages) return;
      commit("SET_PAGE", page);
      dispatch("fetchMemorials");
    },

    openCreate({ commit }: ActionContext<MemorialsState, RootState>) {
      commit("SET_SELECTED", null);
      commit("SET_MODAL_OPEN", true);
    },

    openEdit(
      { commit }: ActionContext<MemorialsState, RootState>,
      person: Memorial,
    ) {
      commit("SET_SELECTED", { ...person });
      commit("SET_MODAL_OPEN", true);
    },

    closeEdit({ commit }: ActionContext<MemorialsState, RootState>) {
      commit("SET_SELECTED", null);
      commit("SET_MODAL_OPEN", false);
    },

    async createMemorial(
      { commit, dispatch }: ActionContext<MemorialsState, RootState>,
      payload: Partial<Memorial>,
    ) {
      commit("SET_SAVING", true);
      try {
        const created: Memorial = await memorialService.createMemorial(payload);
        commit("PREPEND_ITEM", created);
        dispatch(
          "ui/addNotification",
          { message: `${created.name} created successfully.`, type: "success" },
          { root: true },
        );
        dispatch("closeEdit");
      } catch {
        dispatch(
          "ui/addNotification",
          { message: "Failed to create memorial.", type: "error" },
          { root: true },
        );
        throw new Error("create failed");
      } finally {
        commit("SET_SAVING", false);
      }
    },

    async saveEdit(
      { commit, dispatch }: ActionContext<MemorialsState, RootState>,
      { id, payload }: { id: string; payload: Partial<Memorial> },
    ) {
      commit("SET_SAVING", true);
      try {
        const updated: Memorial = await memorialService.updateMemorial(
          id,
          payload,
        );
        commit("UPDATE_ITEM", updated);
        dispatch(
          "ui/addNotification",
          {
            message: `${updated.name ?? "Memorial"} updated successfully.`,
            type: "success",
          },
          { root: true },
        );
        dispatch("closeEdit");
      } catch {
        dispatch(
          "ui/addNotification",
          { message: "Failed to save changes.", type: "error" },
          { root: true },
        );
        throw new Error("save failed");
      } finally {
        commit("SET_SAVING", false);
      }
    },

    async deletePerson(
      { commit, dispatch }: ActionContext<MemorialsState, RootState>,
      id: string,
    ) {
      try {
        await memorialService.deleteMemorial(id);
        commit("REMOVE_ITEM", id);
        dispatch(
          "ui/addNotification",
          { message: "Memorial deleted.", type: "success" },
          { root: true },
        );
      } catch {
        dispatch(
          "ui/addNotification",
          { message: "Failed to delete memorial.", type: "error" },
          { root: true },
        );
      }
    },

    async updateStatus(
      { commit, dispatch }: ActionContext<MemorialsState, RootState>,
      { id, status }: { id: string; status: string },
    ) {
      try {
        const updated: Memorial = await memorialService.updateMemorialStatus(
          id,
          status,
        );
        commit("UPDATE_ITEM_STATUS", { id, status: updated.status ?? status });
        dispatch(
          "ui/addNotification",
          { message: "Status updated.", type: "success" },
          { root: true },
        );
      } catch {
        dispatch(
          "ui/addNotification",
          { message: "Failed to update status.", type: "error" },
          { root: true },
        );
      }
    },
  },
};

export default memorials;
