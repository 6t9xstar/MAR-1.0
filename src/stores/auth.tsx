import { createStore } from "solid-js/store";
import { createContext, useContext } from "solid-js";
import type { User } from "../types";
import { api } from "../lib/api";

interface AuthState {
  token: string | null;
  user: User | null;
  loading: boolean;
}

const defaultState: AuthState = {
  token: api.getToken(),
  user: null,
  loading: false,
};

const AuthContext = createContext<ReturnType<typeof createAuthStore>>();

export function createAuthStore() {
  const [state, setState] = createStore<AuthState>(defaultState);

  return {
    ...state,
    setToken: (token: string | null) => {
      api.setToken(token);
      setState("token", token);
    },
    setUser: (user: User | null) => setState("user", user),
    setLoading: (loading: boolean) => setState("loading", loading),
    login: async (usernameOrEmail: string, password: string) => {
      setState("loading", true);
      try {
        const res = await api.login({ username_or_email: usernameOrEmail, password });
        api.setToken(res.token);
        setState({ token: res.token, user: res.user, loading: false });
        return res;
      } catch (e) {
        setState("loading", false);
        throw e;
      }
    },
    register: async (username: string, email: string, password: string) => {
      setState("loading", true);
      try {
        const res = await api.register({ username, email, password });
        api.setToken(res.token);
        setState({ token: res.token, user: res.user, loading: false });
        return res;
      } catch (e) {
        setState("loading", false);
        throw e;
      }
    },
    logout: () => {
      api.setToken(null);
      setState({ token: null, user: null, loading: false });
    },
    loadProfile: async () => {
      try {
        const user = await api.getProfile();
        setState("user", user);
        return user;
      } catch {
        api.setToken(null);
        setState({ token: null, user: null });
        return null;
      }
    },
  };
}

export function useAuth() {
  const ctx = useContext(AuthContext);
  if (!ctx) throw new Error("useAuth must be used within AuthProvider");
  return ctx;
}

export function AuthProvider(props: { children: any }) {
  const store = createAuthStore();
  return (
    <AuthContext.Provider value={store}>
      {props.children}
    </AuthContext.Provider>
  );
}
