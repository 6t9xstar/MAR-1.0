export interface User {
  id: string;
  username: string;
  email: string;
  display_name: string | null;
  preferred_language: string;
  is_verified: boolean;
  memory_enabled: boolean;
  created_at: string;
}

export interface AuthResponse {
  token: string;
  refresh_token: string;
  expires_at: string;
  user: User;
}

export interface Conversation {
  id: string;
  title: string;
  last_message_preview: string | null;
  message_count: number;
  language: string;
  is_archived: boolean;
  created_at: string;
  updated_at: string;
}

export type MessageRole = "user" | "assistant" | "system";
export type ContentType = "text" | "code" | "image" | "document" | "audio" | "error";

export interface ChatMessage {
  id: string;
  conversation_id: string;
  role: MessageRole;
  content: string;
  content_type: ContentType;
  metadata: Record<string, unknown> | null;
  token_count: number | null;
  latency_ms: number | null;
  sources: SourceCitation[];
  created_at: string;
}

export interface SourceCitation {
  title: string;
  url: string | null;
  snippet: string;
  relevance_score: number;
}

export interface MemoryEntry {
  id: string;
  user_id: string;
  content: string;
  category: string;
  importance: number;
  metadata: Record<string, unknown> | null;
  created_at: string;
  accessed_at: string;
  expires_at: string | null;
}

export interface DocumentSummary {
  id: string;
  filename: string;
  original_filename: string;
  mime_type: string;
  size_bytes: number;
  page_count: number | null;
  status: string;
  created_at: string;
}

export interface PaginatedResponse<T> {
  data: T[];
  page: number;
  per_page: number;
  total: number;
  total_pages: number;
}
