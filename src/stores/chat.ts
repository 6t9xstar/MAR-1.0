import { createStore } from "solid-js/store";
import type { ChatMessage, Conversation } from "../types";
import { api } from "../lib/api";

interface ChatState {
  conversations: Conversation[];
  currentConversationId: string | null;
  messages: ChatMessage[];
  streaming: boolean;
  streamContent: string;
  loading: boolean;
  error: string | null;
}

export function createChatStore() {
  const [state, setState] = createStore<ChatState>({
    conversations: [],
    currentConversationId: null,
    messages: [],
    streaming: false,
    streamContent: "",
    loading: false,
    error: null,
  });

  let abortController: AbortController | null = null;

  const loadConversations = async () => {
    try {
      const res = await api.listConversations();
      setState("conversations", res.data);
    } catch (e: any) {
      setState("error", e.message);
    }
  };

  return {
    ...state,
    loadConversations,
    selectConversation: async (id: string) => {
      setState("currentConversationId", id);
      setState("loading", true);
      try {
        const res = await api.getMessages(id);
        setState("messages", res.data);
      } catch (e: any) {
        setState("error", e.message);
      } finally {
        setState("loading", false);
      }
    },
    sendMessage: async (content: string, language = "en") => {
      setState("error", null);
      setState("streaming", true);
      setState("streamContent", "");

      const userMsg: ChatMessage = {
        id: crypto.randomUUID(),
        conversation_id: state.currentConversationId || "",
        role: "user",
        content,
        content_type: "text",
        metadata: null,
        token_count: null,
        latency_ms: null,
        sources: [],
        created_at: new Date().toISOString(),
      };

      setState("messages", (prev) => [...prev, userMsg]);

      abortController = new AbortController();

      return new Promise<void>((resolve) => {
        api.streamMessage(
          {
            conversation_id: state.currentConversationId || undefined,
            content,
            language,
          },
          (chunk) => {
            setState("streamContent", (prev) => prev + chunk);
          },
          async () => {
            setState("streaming", false);
            const assistantMsg: ChatMessage = {
              id: crypto.randomUUID(),
              conversation_id: state.currentConversationId || "",
              role: "assistant",
              content: state.streamContent,
              content_type: "text",
              metadata: null,
              token_count: null,
              latency_ms: null,
              sources: [],
              created_at: new Date().toISOString(),
            };
            setState("messages", (prev) => [...prev, assistantMsg]);
            setState("streamContent", "");
            await loadConversations();
            resolve();
          },
          (error) => {
            setState("streaming", false);
            setState("error", error.message);
            resolve();
          },
          abortController!.signal,
        );
      });
    },
    cancelStream: () => {
      if (abortController) {
        abortController.abort();
        abortController = null;
        setState("streaming", false);
      }
    },
    createConversation: async (language = "en") => {
      try {
        const conv = await api.createConversation({ language });
        setState("currentConversationId", conv.id);
        setState("messages", []);
        await loadConversations();
        return conv;
      } catch (e: any) {
        setState("error", e.message);
        return null;
      }
    },
    clearError: () => setState("error", null),
  };
}
