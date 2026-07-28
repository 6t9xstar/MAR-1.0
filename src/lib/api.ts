const API_BASE = import.meta.env.VITE_API_URL || "http://localhost:8080";

class ApiClient {
  private token: string | null = null;

  setToken(token: string | null) {
    this.token = token;
    if (token) {
      localStorage.setItem("mar_token", token);
    } else {
      localStorage.removeItem("mar_token");
    }
  }

  getToken(): string | null {
    return this.token || localStorage.getItem("mar_token");
  }

  private async request<T>(
    method: string,
    path: string,
    body?: unknown,
    options?: { stream?: boolean; signal?: AbortSignal },
  ): Promise<T> {
    const headers: Record<string, string> = {
      "Content-Type": "application/json",
    };
    const token = this.getToken();
    if (token) {
      headers["Authorization"] = `Bearer ${token}`;
    }

    const response = await fetch(`${API_BASE}${path}`, {
      method,
      headers,
      body: body ? JSON.stringify(body) : undefined,
      signal: options?.signal,
    });

    if (!response.ok) {
      const error = await response.json().catch(() => ({ error: { message: "Request failed" } }));
      throw new Error(error.error?.message || `HTTP ${response.status}`);
    }

    return response.json();
  }

  stream(
    path: string,
    body: unknown,
    onChunk: (text: string) => void,
    onDone: () => void,
    onError: (error: Error) => void,
    signal?: AbortSignal,
  ): void {
    const token = this.getToken();
    const headers: Record<string, string> = {
      "Content-Type": "application/json",
    };
    if (token) {
      headers["Authorization"] = `Bearer ${token}`;
    }

    fetch(`${API_BASE}${path}`, {
      method: "POST",
      headers,
      body: JSON.stringify(body),
      signal,
    })
      .then(async (response) => {
        if (!response.ok) {
          const error = await response.json().catch(() => ({ error: { message: "Stream failed" } }));
          onError(new Error(error.error?.message || `HTTP ${response.status}`));
          return;
        }

        const reader = response.body?.getReader();
        if (!reader) {
          onError(new Error("No response body"));
          return;
        }

        const decoder = new TextDecoder();
        const processStream = async () => {
          while (true) {
            const { done, value } = await reader.read();
            if (done) {
              onDone();
              break;
            }
            const text = decoder.decode(value, { stream: true });
            const lines = text.split("\n");
            for (const line of lines) {
              if (line.startsWith("data: ")) {
                const data = line.slice(6);
                if (data === "[DONE]") {
                  onDone();
                  return;
                }
                onChunk(data);
              }
            }
          }
        };
        processStream().catch(onError);
      })
      .catch(onError);
  }

  // Auth
  register = (data: { username: string; email: string; password: string }) =>
    this.request<import("../types").AuthResponse>("POST", "/api/auth/register", data);

  login = (data: { username_or_email: string; password: string }) =>
    this.request<import("../types").AuthResponse>("POST", "/api/auth/login", data);

  getProfile = () => this.request<import("../types").User>("GET", "/api/auth/me");

  updateProfile = (data: { display_name?: string; preferred_language?: string }) =>
    this.request<import("../types").User>("PUT", "/api/auth/me", data);

  // Conversations
  listConversations = (page = 1, perPage = 50) =>
    this.request<import("../types").PaginatedResponse<import("../types").Conversation>>(
      "GET", `/api/conversations?page=${page}&per_page=${perPage}`,
    );

  createConversation = (data?: { title?: string; language?: string }) =>
    this.request<import("../types").Conversation>("POST", "/api/conversations", data || {});

  getConversation = (id: string) =>
    this.request<import("../types").Conversation>("GET", `/api/conversations/${id}`);

  getMessages = (convId: string, page = 1, perPage = 100) =>
    this.request<{ data: import("../types").ChatMessage[] }>(
      "GET", `/api/conversations/${convId}/messages?page=${page}&per_page=${perPage}`,
    );

  // Chat
  sendMessage = (data: {
    conversation_id?: string;
    content: string;
    language?: string;
    stream?: boolean;
  }) => this.request<{ message: import("../types").ChatMessage; conversation_id: string }>(
    "POST", "/api/chat", data,
  );

  streamMessage = (
    data: { conversation_id?: string; content: string; language?: string },
    onChunk: (text: string) => void,
    onDone: () => void,
    onError: (error: Error) => void,
    signal?: AbortSignal,
  ) => {
    this.stream("/api/chat/stream", { ...data, stream: true }, onChunk, onDone, onError, signal);
  };

  // Memory
  listMemories = (page = 1, perPage = 50) =>
    this.request<import("../types").PaginatedResponse<import("../types").MemoryEntry>>(
      "GET", `/api/memory?page=${page}&per_page=${perPage}`,
    );

  searchMemories = (query: string, limit = 10) =>
    this.request<import("../types").MemoryEntry[]>("POST", "/api/memory/search", { query, limit });

  deleteMemory = (id: string) =>
    this.request<{ deleted: boolean }>("DELETE", `/api/memory/${id}`);

  clearMemories = () =>
    this.request<{ cleared: boolean }>("DELETE", "/api/memory");

  // Documents
  listDocuments = (page = 1, perPage = 20) =>
    this.request<import("../types").PaginatedResponse<import("../types").DocumentSummary>>(
      "GET", `/api/documents?page=${page}&per_page=${perPage}`,
    );

  uploadDocument = async (file: File, conversationId?: string) => {
    const formData = new FormData();
    formData.append("file", file);
    if (conversationId) formData.append("conversation_id", conversationId);

    const token = this.getToken();
    const response = await fetch(`${API_BASE}/api/documents`, {
      method: "POST",
      headers: token ? { Authorization: `Bearer ${token}` } : {},
      body: formData,
    });

    if (!response.ok) {
      const error = await response.json().catch(() => ({ error: { message: "Upload failed" } }));
      throw new Error(error.error?.message || "Upload failed");
    }
    return response.json();
  };

  deleteDocument = (id: string) =>
    this.request<{ deleted: boolean }>("DELETE", `/api/documents/${id}`);
}

export const api = new ApiClient();
