import { Match, Switch } from "solid-js";
import type { ChatMessage as ChatMessageType } from "../types";
import MarkdownRenderer from "./MarkdownRenderer";

interface Props {
  message: ChatMessageType;
}

export default function ChatMessage(props: Props) {
  const isUser = () => props.message.role === "user";
  const isAssistant = () => props.message.role === "assistant";

  return (
    <div class={`flex gap-3 px-4 py-3 ${isUser() ? "" : "bg-zinc-900/50"} rounded-lg`}>
      <div
        class={`w-8 h-8 rounded-full flex items-center justify-center shrink-0 ${
          isUser() ? "bg-zinc-700" : "bg-emerald-600"
        }`}
      >
        <span class="text-xs font-bold text-white">
          {isUser() ? "U" : "M"}
        </span>
      </div>

      <div class="flex-1 min-w-0">
        <div class="text-xs text-zinc-500 mb-1 font-medium">
          {isUser() ? "You" : "MAR"}
        </div>

        <Switch>
          <Match when={isUser()}>
            <p class="text-sm text-zinc-200 whitespace-pre-wrap">{props.message.content}</p>
          </Match>
          <Match when={isAssistant()}>
            <div class="text-sm text-zinc-200">
              <MarkdownRenderer content={props.message.content} />
            </div>
          </Match>
        </Switch>

        {props.message.sources.length > 0 && (
          <div class="mt-2 space-y-1">
            <div class="text-xs text-zinc-500 font-medium">Sources:</div>
            {props.message.sources.map((source) => (
              <div class="text-xs text-zinc-400 flex items-start gap-1">
                <span class="text-emerald-500 shrink-0">&#8226;</span>
                <span>{source.title}</span>
              </div>
            ))}
          </div>
        )}
      </div>
    </div>
  );
}
