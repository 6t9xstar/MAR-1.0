import { createEffect, createSignal, For, Show } from "solid-js";
import { useNavigate } from "@solidjs/router";
import Sidebar from "./Sidebar";
import ChatMessage from "./ChatMessage";
import ChatInput from "./ChatInput";
import { createChatStore } from "../stores/chat";
import { useAuth } from "../stores/auth";
import { createSettingsStore } from "../stores/settings";

export default function ChatView() {
  const navigate = useNavigate();
  const auth = useAuth();
  const settings = createSettingsStore();
  const chat = createChatStore();
  const [sidebarOpen, setSidebarOpen] = createSignal(true);

  createEffect(() => {
    if (!auth.token) {
      navigate("/", { replace: true });
      return;
    }
    chat.loadConversations();
  });

  const handleSend = async (content: string) => {
    if (!chat.currentConversationId) {
      const conv = await chat.createConversation(settings.language);
      if (!conv) return;
    }
    await chat.sendMessage(content, settings.language);
  };

  return (
    <div class="flex h-full">
      <Sidebar
        open={sidebarOpen()}
        conversations={chat.conversations}
        currentId={chat.currentConversationId}
        onSelect={(id) => chat.selectConversation(id)}
        onNew={() => chat.createConversation(settings.language)}
        onToggle={() => setSidebarOpen(!sidebarOpen())}
      />

      <div class="flex-1 flex flex-col min-w-0">
        <header class="h-14 border-b border-zinc-800 flex items-center px-4 gap-3 shrink-0">
          <button
            onClick={() => setSidebarOpen(!sidebarOpen())}
            class="p-2 hover:bg-zinc-800 rounded-lg transition-colors"
          >
            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width={2} d="M4 6h16M4 12h16M4 18h16" />
            </svg>
          </button>
          <span class="text-sm font-medium text-zinc-400 truncate">
            {chat.conversations.find((c) => c.id === chat.currentConversationId)?.title || "New conversation"}
          </span>
          <div class="ml-auto flex items-center gap-2">
            <button
              onClick={() => navigate("/settings")}
              class="p-2 hover:bg-zinc-800 rounded-lg transition-colors"
              title="Settings"
            >
              <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width={2}
                  d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.066 2.573c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.573 1.066c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.066-2.573c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width={2} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
              </svg>
            </button>
          </div>
        </header>

        <div class="flex-1 overflow-y-auto px-4 py-6 space-y-4">
          <Show when={chat.messages.length === 0 && !chat.loading}>
            <div class="flex-1 flex items-center justify-center">
              <div class="text-center max-w-md">
                <div class="w-16 h-16 bg-emerald-500/10 rounded-2xl flex items-center justify-center mx-auto mb-4">
                  <svg class="w-8 h-8 text-emerald-400" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width={2}
                      d="M8 10h.01M12 10h.01M16 10h.01M9 16H5a2 2 0 01-2-2V6a2 2 0 012-2h14a2 2 0 012 2v8a2 2 0 01-2 2h-5l-5 5v-5z" />
                  </svg>
                </div>
                <h2 class="text-xl font-semibold text-zinc-300 mb-2">Welcome to MAR 1.0</h2>
                <p class="text-zinc-500">
                  Your AI assistant built for Pakistan. Ask me anything — in English, Urdu, Roman Urdu, or Punjabi.
                </p>
              </div>
            </div>
          </Show>

          <Show when={chat.loading}>
            <div class="flex justify-center py-4">
              <div class="w-6 h-6 border-2 border-emerald-500 border-t-transparent rounded-full animate-spin" />
            </div>
          </Show>

          <For each={chat.messages}>
            {(msg) => <ChatMessage message={msg} />}
          </For>

          <Show when={chat.streaming}>
            <div class="flex gap-3 px-4 py-3">
              <div class="w-8 h-8 rounded-full bg-emerald-600 flex items-center justify-center shrink-0">
                <span class="text-xs font-bold text-white">M</span>
              </div>
              <div class="flex-1 min-w-0">
                <div class="text-sm text-zinc-300 prose prose-invert max-w-none">
                  {chat.streamContent}
                  <span class="inline-block w-2 h-4 bg-emerald-400 animate-pulse ml-0.5" />
                </div>
              </div>
            </div>
          </Show>

          <Show when={chat.error}>
            <div class="flex gap-3 px-4 py-3 bg-red-500/10 border border-red-500/20 rounded-lg">
              <svg class="w-5 h-5 text-red-400 shrink-0 mt-0.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width={2}
                  d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
              </svg>
              <p class="text-sm text-red-300">{chat.error}</p>
            </div>
          </Show>
        </div>

        <ChatInput
          onSend={handleSend}
          onCancel={chat.cancelStream}
          streaming={chat.streaming}
          disabled={chat.loading}
        />
      </div>
    </div>
  );
}
