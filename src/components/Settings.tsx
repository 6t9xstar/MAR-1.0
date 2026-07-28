import { useNavigate } from "@solidjs/router";
import { useAuth } from "../stores/auth";
import { createSettingsStore } from "../stores/settings";

export default function Settings() {
  const navigate = useNavigate();
  const auth = useAuth();
  const settings = createSettingsStore();

  return (
    <div class="flex-1 overflow-y-auto p-6 max-w-2xl mx-auto w-full">
      <div class="flex items-center justify-between mb-8">
        <h1 class="text-xl font-semibold text-zinc-100">Settings</h1>
        <button
          onClick={() => navigate("/chat")}
          class="px-4 py-2 text-sm text-zinc-400 hover:text-zinc-200 transition-colors"
        >
          Back to Chat
        </button>
      </div>

      <div class="space-y-6">
        <section class="bg-zinc-900/50 border border-zinc-800 rounded-xl p-5">
          <h2 class="text-sm font-semibold text-zinc-300 mb-4 uppercase tracking-wider">Preferences</h2>
          <div class="space-y-4">
            <div>
              <label class="block text-sm text-zinc-400 mb-1.5">Language</label>
              <select
                value={settings.language}
                onChange={(e) => settings.setLanguage(e.currentTarget.value)}
                class="w-full bg-zinc-800 border border-zinc-700 rounded-lg px-3 py-2 text-sm text-zinc-200
                       focus:outline-none focus:ring-2 focus:ring-emerald-500/50"
              >
                <option value="en">English</option>
                <option value="ur">Urdu (اردو)</option>
                <option value="roman-urdu">Roman Urdu</option>
                <option value="pnb">Punjabi (پنجابی)</option>
              </select>
            </div>

            <div>
              <label class="block text-sm text-zinc-400 mb-1.5">Temperature</label>
              <div class="flex items-center gap-3">
                <input
                  type="range"
                  min="0"
                  max="2"
                  step="0.1"
                  value={settings.temperature}
                  onChange={(e) => settings.setTemperature(parseFloat(e.currentTarget.value))}
                  class="flex-1 accent-emerald-500"
                />
                <span class="text-sm text-zinc-400 w-8">{settings.temperature}</span>
              </div>
            </div>

            <div class="flex items-center justify-between">
              <div>
                <label class="text-sm text-zinc-300">Memory</label>
                <p class="text-xs text-zinc-500 mt-0.5">MAR remembers your preferences and context</p>
              </div>
              <button
                onClick={() => settings.setMemoryEnabled(!settings.memoryEnabled)}
                class={`relative w-11 h-6 rounded-full transition-colors ${
                  settings.memoryEnabled ? "bg-emerald-600" : "bg-zinc-700"
                }`}
              >
                <span
                  class={`absolute top-0.5 left-0.5 w-5 h-5 rounded-full bg-white transition-transform ${
                    settings.memoryEnabled ? "translate-x-5" : ""
                  }`}
                />
              </button>
            </div>
          </div>
        </section>

        <section class="bg-zinc-900/50 border border-zinc-800 rounded-xl p-5">
          <h2 class="text-sm font-semibold text-zinc-300 mb-4 uppercase tracking-wider">Account</h2>
          <div class="space-y-3">
            <div class="text-sm">
              <span class="text-zinc-500">Username: </span>
              <span class="text-zinc-300">{auth.user?.username || "-"}</span>
            </div>
            <div class="text-sm">
              <span class="text-zinc-500">Email: </span>
              <span class="text-zinc-300">{auth.user?.email || "-"}</span>
            </div>
            <button
              onClick={() => {
                auth.logout();
                navigate("/", { replace: true });
              }}
              class="px-4 py-2 text-sm text-red-400 hover:bg-red-500/10 rounded-lg transition-colors"
            >
              Sign Out
            </button>
          </div>
        </section>
      </div>
    </div>
  );
}
