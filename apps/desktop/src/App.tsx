import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import "./App.css";

type ToolResponse = {
  success?: boolean;
  message?: string;
  data?: {
    app?: string;
    resolvedTarget?: string;
  };
};

type VoiceEvent = {
  type?: string;
  value?: string;
  model?: string;
  score?: number;
  threshold?: number;
  message?: string;
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
  const [voiceStatus, setVoiceStatus] = useState("Voice worker chua chay.");
  const [isVoiceRunning, setIsVoiceRunning] = useState(false);
  const [assistantMessage, setAssistantMessage] = useState("Bam Start voice roi noi wake word.");

  useEffect(() => {
    const unlistenPromise = listen<VoiceEvent>("voice_event", (event) => {
      const voiceEvent = event.payload;

      if (voiceEvent.type === "status") {
        setVoiceStatus(`Voice: ${voiceEvent.value ?? "unknown"}`);
      }

      if (voiceEvent.type === "wake_model_loaded") {
        setVoiceStatus(
          `Da load wake model: ${voiceEvent.model ?? "unknown"} | threshold ${formatScore(
            voiceEvent.threshold,
          )}`,
        );
      }

      if (voiceEvent.type === "wake_score") {
        setVoiceStatus(`Dang lang nghe... score ${formatScore(voiceEvent.score)}`);
      }

      if (voiceEvent.type === "wake_word_detected") {
        setAssistantMessage("Chao ban");
        setVoiceStatus(`Da nghe thay wake word (${formatScore(voiceEvent.score)})`);
      }

      if (voiceEvent.type === "worker_error" || voiceEvent.type === "error") {
        setVoiceStatus(`Voice loi: ${voiceEvent.message ?? "unknown error"}`);
        setAssistantMessage("Voice worker dang gap loi.");
      }
    });

    return () => {
      void unlistenPromise.then((unlisten) => unlisten());
    };
  }, []);

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

  async function startVoiceWorker() {
    try {
      setVoiceStatus("Dang start voice worker...");
      setAssistantMessage("Dang lang nghe wake word...");
      await invoke("start_voice_worker");
      setIsVoiceRunning(true);
    } catch (error) {
      setVoiceStatus(`Voice loi: ${String(error)}`);
    }
  }

  async function stopVoiceWorker() {
    try {
      await invoke("stop_voice_worker");
      setIsVoiceRunning(false);
      setVoiceStatus("Voice worker da dung.");
      setAssistantMessage("Bam Start voice roi noi wake word.");
    } catch (error) {
      setVoiceStatus(`Voice loi: ${String(error)}`);
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

        <section className="voice-panel" aria-labelledby="voice-title">
          <div>
            <p className="eyebrow">Wake word test</p>
            <h2 id="voice-title">Python voice worker</h2>
          </div>

          <div className="voice-actions">
            <button
              type="button"
              disabled={isVoiceRunning}
              onClick={() => void startVoiceWorker()}
            >
              Start voice
            </button>
            <button
              type="button"
              className="secondary-button"
              disabled={!isVoiceRunning}
              onClick={() => void stopVoiceWorker()}
            >
              Stop voice
            </button>
          </div>

          <p className="status" role="status">
            {voiceStatus}
          </p>

          <div className="assistant-message" role="status" aria-live="polite">
            {assistantMessage}
          </div>
        </section>
      </section>
    </main>
  );
}

function formatScore(score?: number) {
  return typeof score === "number" ? score.toFixed(3) : "n/a";
}

export default App;
