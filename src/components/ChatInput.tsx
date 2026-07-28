import { createSignal, onMount, Show } from "solid-js";

interface Props {
  onSend: (content: string) => void;
  onCancel: () => void;
  streaming: boolean;
  disabled: boolean;
}

export default function ChatInput(props: Props) {
  const [input, setInput] = createSignal("");
  let textareaRef: HTMLTextAreaElement | undefined;

  const adjustHeight = () => {
    if (textareaRef) {
      textareaRef.style.height = "auto";
      textareaRef.style.height = `${Math.min(textareaRef.scrollHeight, 200)}px`;
    }
  };

  const handleSubmit = () => {
    const text = input().trim();
    if (!text || props.streaming || props.disabled) return;
    props.onSend(text);
    setInput("");
    if (textareaRef) {
      textareaRef.style.height = "auto";
    }
  };

  const handleKeyDown = (e: KeyboardEvent) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSubmit();
    }
  };

  return (
    <div class="border-t border-zinc-800 p-4 shrink-0">
      <div class="max-w-4xl mx-auto flex items-end gap-3">
        <div class="flex-1 relative">
          <textarea
            ref={textareaRef}
            value={input()}
            onInput={(e) => {
              setInput(e.currentTarget.value);
              adjustHeight();
            }}
            onKeyDown={handleKeyDown}
            placeholder="Ask MAR anything..."
            rows={1}
            disabled={props.streaming || props.disabled}
            class="w-full bg-zinc-900 border border-zinc-700 rounded-xl px-4 py-3 pr-12
                   text-sm text-zinc-100 placeholder-zinc-500 resize-none
                   focus:outline-none focus:ring-2 focus:ring-emerald-500/50 focus:border-emerald-500
                   disabled:opacity-50 disabled:cursor-not-allowed
                   transition-all min-h-[44px] max-h-[200px]"
          />
        </div>

        <Show
          when={props.streaming}
          fallback={
            <button
              onClick={handleSubmit}
              disabled={!input().trim() || props.disabled}
              class="p-3 bg-emerald-600 hover:bg-emerald-500 disabled:bg-zinc-800
                     disabled:text-zinc-600 text-white rounded-xl transition-all
                     disabled:cursor-not-allowed shrink-0"
            >
              <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width={2}
                  d="M12 19V5m0 0l-7 7m7-7l7 7" />
              </svg>
            </button>
          }
        >
          <button
            onClick={props.onCancel}
            class="p-3 bg-red-600 hover:bg-red-500 text-white rounded-xl transition-all shrink-0"
          >
            <svg class="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width={2}
                d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </Show>
      </div>
    </div>
  );
}
