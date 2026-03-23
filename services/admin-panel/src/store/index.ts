import { createStore } from "vuex";
import auth, { AuthState } from "./modules/auth";
import memorials, { MemorialsState } from "./modules/memorials";
import ui, { UIState } from "./modules/ui";

export interface RootState {
  auth: AuthState;
  memorials: MemorialsState;
  ui: UIState;
}

export default createStore<RootState>({
  modules: { auth, memorials, ui },
  strict: import.meta.env.DEV,
});
