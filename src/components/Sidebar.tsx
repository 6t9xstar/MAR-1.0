import { For, Show } from "solid-js";
import type { Conversation } from "../types";

interface Props {
  open: boolean;
  conversations: Conversation[];
  currentId: string | null;
  onSelect: (id: string) => void;
  onNew: () => void;
  onToggle: () => void;
}

export default function Sidebar(props: Props) {
  return (
    <Show when={props.open}>
      <div class="w-72 border-r border-zinc-800 bg-zinc-900/50 flex flex-col shrink-0">
        <div class="p-4 border-b border-zinc-800">
          <button
            onClick={props.onNew}
            class="w-full px-4 py-2.5 bg-emerald-600 hover:bg-emerald-500 text-white 
                   text-sm font-medium rounded-xl transition-colors flex items-center gap-2 justify-center"
          >
            <svg class="w-4 h-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width={2} d="M12 4v16m8-8H4" />
            </svg>
            New Chat
          </button>
        </div>

        <div class="flex-1 overflow-y-auto p-2 space-y-1">
          <For each={props.conversations}>
            {(conv) => (
              <button
                onClick={() => props.onSelect(conv.id)}
                class={`w-full text-left px-3 py-2.5 rounded-lg text-sm transition-colors
                  ${conv.id === props.currentId
                    ? "bg-emerald-500/10 text-emerald-400 border border-emerald-500/20"
                    : "text-zinc-400 hover:bg-zinc-800 hover:text-zinc-200 border border-transparent"
                  }`}
              >
                <div class="truncate font-medium">{conv.title}</div>
                <div class="text-xs text-zinc-600 mt-0.5">
                  {conv.message_count} messages &middot; {conv.language}
                </div>
              </button>
            )}
          </For>

          <Show when={props.conversations.length === 0}>
            <div class="px-3 py-8 text-center">
              <p class="text-sm text-zinc-600">No conversations yet</p>
              <p class="text-xs text-zinc-700 mt-1">Start a new chat to begin</p>
            </div>
          </Show>
        </div>

        <div class="p-3 border-t border-zinc-800">
          <div class="flex items-center gap-2 px-2">
            <div class="w-2 h-2 rounded-full bg-emerald-500" />
            <span class="text-xs text-zinc-500">MAR 1.0 ready</span>
          </div>
        </div>
      </div>
    </Show>
  );
}
