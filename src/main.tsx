import ReactDOM from "react-dom/client";
import App from "./app";
import { StrictMode } from "react";
import "./shared/libraries/i18n/index"

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
