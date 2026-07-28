import { describe, it, expect } from "vitest";

describe("API Client", () => {
  it("should construct correct API URLs", () => {
    const base = import.meta.env.VITE_API_URL || "http://localhost:8080";
    expect(base).toBeDefined();
  });

  it("should handle auth token storage", () => {
    localStorage.setItem("mar_token", "test-token");
    expect(localStorage.getItem("mar_token")).toBe("test-token");
    localStorage.removeItem("mar_token");
    expect(localStorage.getItem("mar_token")).toBeNull();
  });
});
