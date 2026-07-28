import { describe, it, expect } from "vitest";
import { render, screen } from "@solidjs/testing-library";
import ChatMessage from "../ChatMessage";

describe("ChatMessage", () => {
  const userMessage = {
    id: "1",
    conversation_id: "conv-1",
    role: "user" as const,
    content: "Hello MAR",
    content_type: "text" as const,
    metadata: null,
    token_count: null,
    latency_ms: null,
    sources: [],
    created_at: new Date().toISOString(),
  };

  const assistantMessage = {
    ...userMessage,
    id: "2",
    role: "assistant" as const,
    content: "Hello! How can I help you today?",
  };

  it("should render user message", () => {
    render(() => <ChatMessage message={userMessage} />);
    expect(screen.getByText("Hello MAR")).toBeDefined();
  });

  it("should render assistant message", () => {
    render(() => <ChatMessage message={assistantMessage} />);
    expect(screen.getByText("Hello! How can I help you today?")).toBeDefined();
  });

  it("should show MAR label for assistant", () => {
    render(() => <ChatMessage message={assistantMessage} />);
    expect(screen.getByText("MAR")).toBeDefined();
  });

  it("should show You label for user", () => {
    render(() => <ChatMessage message={userMessage} />);
    expect(screen.getByText("You")).toBeDefined();
  });
});
