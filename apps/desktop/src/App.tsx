import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

type ToolResponse = {
  success?: boolean;
  message?: string;
  data?: {
    app?: string;
    resolvedTarget?: string;
  };
};

const quickApps = [
  { label: "Notepad", value: "notepad" },
  { label: "Chrome", value: "chrome" },
  { label: "Zalo", value: "zalo" },
  { label: "VS Code", value: "code" },
];

function App() {
  const [appName, setAppName] = useState("notepad");
  const [status, setStatus] = useState("Nhap ten app hoac chon nhanh ben duoi.");
  const [isOpening, setIsOpening] = useState(false);

  async function openSelectedApp(nextAppName = appName) {
    const trimmedAppName = nextAppName.trim();

    if (!trimmedAppName) {
      setStatus("Nhap ten app truoc khi mo.");
      return;
    }

    try {
      setIsOpening(true);
      setStatus(`Dang mo ${trimmedAppName}...`);

      const response = await invoke<ToolResponse>("run_tool", {
        toolName: "open_app",
        args: {
          app: trimmedAppName,
        },
      });

      setStatus(
        response.data?.resolvedTarget
          ? `Da mo ${trimmedAppName}: ${response.data.resolvedTarget}`
          : response.message ?? `Da gui lenh mo ${trimmedAppName}.`,
      );
    } catch (error) {
      setStatus(`Loi: ${String(error)}`);
    } finally {
      setIsOpening(false);
    }
  }

  return (
    <main className="app-shell">
      <section className="tool-panel" aria-labelledby="open-app-title">
        <div>
          <p className="eyebrow">Tauri tool test</p>
          <h1 id="open-app-title">Mo ung dung tren may</h1>
          <p className="subtitle">
            Goi tool <code>open_app</code> qua command <code>run_tool</code>.
          </p>
        </div>

        <form
          className="open-form"
          onSubmit={(event) => {
            event.preventDefault();
            void openSelectedApp();
          }}
        >
          <label htmlFor="app-name">Ten app hoac alias</label>
          <div className="input-row">
            <input
              id="app-name"
              value={appName}
              onChange={(event) => setAppName(event.target.value)}
              placeholder="chrome, zalo, word, spotify..."
              autoComplete="off"
            />
            <button type="submit" disabled={isOpening}>
              {isOpening ? "Dang mo..." : "Mo app"}
            </button>
          </div>
        </form>

        <div className="quick-grid" aria-label="Mo nhanh">
          {quickApps.map((app) => (
            <button
              key={app.value}
              type="button"
              className="quick-button"
              disabled={isOpening}
              onClick={() => {
                setAppName(app.value);
                void openSelectedApp(app.value);
              }}
            >
              {app.label}
            </button>
          ))}
        </div>

        <p className="status" role="status">
          {status}
        </p>
      </section>
    </main>
  );
}

export default App;
