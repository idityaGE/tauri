import { listen } from "@tauri-apps/api/event"
import { invoke } from "@tauri-apps/api/core"
import { useEffect } from "react";

const EventDemo = () => {
  const startListen = async () => {
    const unlistenStart = await listen("download-started", (event) => {
      console.log("Download started:", event.payload);
    });

    const unlistenProgress = await listen("download-progress", (event) => {
      console.log("Download progress:", event.payload);
    });

    const unlistenComplete = await listen("download-complete", (event) => {
      console.log("Download complete:", event.payload);
    });

    return { unlistenStart, unlistenProgress, unlistenComplete };
  }

  const handleEvent = async () => {
    await invoke("download", { url: "https://iditya.tech/resume.pdf" });
  }

  useEffect(() => {
    let unlistenFunctions: { unlistenStart: () => void; unlistenProgress: () => void; unlistenComplete: () => void } | null = null;

    const setupListeners = async () => {
      unlistenFunctions = await startListen();
    };

    setupListeners();

    return () => {
      if (unlistenFunctions) {
        console.log("Unsubscribing from events");
        unlistenFunctions.unlistenStart();
        unlistenFunctions.unlistenProgress();
        unlistenFunctions.unlistenComplete();
      }
    };
  }, []);

  return (
    <div>
      <h2>Event Demo</h2>
      <button onClick={handleEvent}>Start Download</button>
    </div>
  )
}

export default EventDemo
