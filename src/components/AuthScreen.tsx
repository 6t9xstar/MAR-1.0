import { createSignal, Show } from "solid-js";
import { useNavigate } from "@solidjs/router";
import { useAuth } from "../stores/auth";

export default function AuthScreen() {
  const auth = useAuth();
  const navigate = useNavigate();
  const [isLogin, setIsLogin] = createSignal(true);
  const [username, setUsername] = createSignal("");
  const [email, setEmail] = createSignal("");
  const [password, setPassword] = createSignal("");
  const [error, setError] = createSignal("");

  const handleSubmit = async (e: Event) => {
    e.preventDefault();
    setError("");

    try {
      if (isLogin()) {
        await auth.login(username(), password());
      } else {
        await auth.register(username(), email(), password());
      }
      navigate("/chat", { replace: true });
    } catch (err: any) {
      setError(err.message || "Authentication failed");
    }
  };

  return (
    <div class="flex-1 flex items-center justify-center p-4">
      <div class="w-full max-w-sm">
        <div class="text-center mb-8">
          <div class="w-16 h-16 bg-emerald-500/10 rounded-2xl flex items-center justify-center mx-auto mb-4">
            <span class="text-2xl font-bold text-emerald-400">M</span>
          </div>
          <h1 class="text-2xl font-bold text-zinc-100">MAR 1.0</h1>
          <p class="text-sm text-zinc-500 mt-1">
            {isLogin() ? "Welcome back" : "Create your account"}
          </p>
        </div>

        <form onSubmit={handleSubmit} class="space-y-4">
          <div>
            <input
              type="text"
              placeholder="Username"
              value={username()}
              onInput={(e) => setUsername(e.currentTarget.value)}
              required
              minLength={3}
              class="w-full bg-zinc-900 border border-zinc-700 rounded-xl px-4 py-3
                     text-sm text-zinc-100 placeholder-zinc-500
                     focus:outline-none focus:ring-2 focus:ring-emerald-500/50"
            />
          </div>

          <Show when={!isLogin()}>
            <div>
              <input
                type="email"
                placeholder="Email"
                value={email()}
                onInput={(e) => setEmail(e.currentTarget.value)}
                required
                class="w-full bg-zinc-900 border border-zinc-700 rounded-xl px-4 py-3
                       text-sm text-zinc-100 placeholder-zinc-500
                       focus:outline-none focus:ring-2 focus:ring-emerald-500/50"
              />
            </div>
          </Show>

          <div>
            <input
              type="password"
              placeholder="Password"
              value={password()}
              onInput={(e) => setPassword(e.currentTarget.value)}
              required
              minLength={8}
              class="w-full bg-zinc-900 border border-zinc-700 rounded-xl px-4 py-3
                     text-sm text-zinc-100 placeholder-zinc-500
                     focus:outline-none focus:ring-2 focus:ring-emerald-500/50"
            />
          </div>

          <Show when={error()}>
            <div class="text-sm text-red-400 bg-red-500/10 rounded-lg px-4 py-2">{error()}</div>
          </Show>

          <button
            type="submit"
            disabled={auth.loading}
            class="w-full py-3 bg-emerald-600 hover:bg-emerald-500 disabled:bg-zinc-800
                   disabled:text-zinc-500 text-white font-medium rounded-xl
                   transition-all disabled:cursor-not-allowed"
          >
            {auth.loading ? (
              <span class="flex items-center justify-center gap-2">
                <span class="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin" />
                {isLogin() ? "Signing in..." : "Creating account..."}
              </span>
            ) : (
              isLogin() ? "Sign In" : "Create Account"
            )}
          </button>
        </form>

        <p class="text-sm text-zinc-600 text-center mt-6">
          {isLogin() ? "Don't have an account?" : "Already have an account?"}{" "}
          <button
            onClick={() => { setIsLogin(!isLogin()); setError(""); }}
            class="text-emerald-400 hover:text-emerald-300 font-medium"
          >
            {isLogin() ? "Sign up" : "Sign in"}
          </button>
        </p>
      </div>
    </div>
  );
}
