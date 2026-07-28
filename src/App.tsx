import { Routes, Route, useNavigate } from "@solidjs/router";
import { createEffect, lazy, Suspense } from "solid-js";
import { useAuth } from "./stores/auth";

const ChatView = lazy(() => import("./components/ChatView"));
const AuthScreen = lazy(() => import("./components/AuthScreen"));
const Settings = lazy(() => import("./components/Settings"));

export default function App() {
  const auth = useAuth();
  const navigate = useNavigate();

  createEffect(() => {
    if (auth.token) {
      navigate("/chat", { replace: true });
    }
  });

  return (
    <div class="h-screen w-screen flex flex-col overflow-hidden bg-zinc-950">
      <Suspense
        fallback={
          <div class="flex-1 flex items-center justify-center">
            <div class="w-8 h-8 border-2 border-emerald-500 border-t-transparent rounded-full animate-spin" />
          </div>
        }
      >
        <Routes>
          <Route path="/" component={AuthScreen} />
          <Route path="/chat" component={ChatView} />
          <Route path="/chat/:id" component={ChatView} />
          <Route path="/settings" component={Settings} />
        </Routes>
      </Suspense>
    </div>
  );
}
