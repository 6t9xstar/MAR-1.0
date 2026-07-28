import { render } from "solid-js/web";
import { Router } from "@solidjs/router";
import { AuthProvider } from "./stores/auth";
import App from "./App";
import "./index.css";

const root = document.getElementById("root");
if (!root) throw new Error("Root element not found");

render(
  () => (
    <Router>
      <AuthProvider>
        <App />
      </AuthProvider>
    </Router>
  ),
  root,
);
