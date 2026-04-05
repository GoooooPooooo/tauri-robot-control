import React from "react";
import { invoke } from "@tauri-apps/api/core";

interface RobotState {
  speed: number;
}

const useRobotState = () => {
  const [state, setState] = React.useState<RobotState>({ speed: 1 });
  const [direction, setDirection] = React.useState<string | null>(null);

  const refreshStatus = React.useCallback(async () => {
    try {
      const status = await invoke<{ speed: number }>("get_status");
      setState({ speed: status.speed });
    } catch (e) {
      console.error("Failed to get status:", e);
    }
  }, []);

  const setSpeed = React.useCallback(async (speed: number) => {
    try {
      await invoke("set_speed", { speed });
      setState({ speed });
    } catch (e) {
      console.error("Failed to set speed:", e);
    }
  }, []);

  const move = React.useCallback(async (dir: string) => {
    try {
      setDirection(dir);
      await invoke("move_direction", { direction: dir });
    } catch (e) {
      console.error("Failed to move:", e);
    }
  }, []);

  const stop = React.useCallback(async () => {
    try {
      setDirection(null);
      await invoke("stop");
    } catch (e) {
      console.error("Failed to stop:", e);
    }
  }, []);

  return { state, direction, refreshStatus, setSpeed, move, stop };
};

function App() {
  const { direction, refreshStatus, setSpeed, move, stop } = useRobotState();
  const [speedSlider, setSpeedSlider] = React.useState(1);
  const [message, setMessage] = React.useState({ text: "", type: "" });

  React.useEffect(() => {
    refreshStatus();
    
    const handleKeyDown = (e: KeyboardEvent) => {
      switch (e.key) {
        case "ArrowUp":
        case "w":
        case "W":
          move("forward");
          break;
        case "ArrowDown":
        case "s":
        case "S":
          move("backward");
          break;
        case "ArrowLeft":
        case "a":
        case "A":
          move("left");
          break;
        case "ArrowRight":
        case "d":
        case "D":
          move("right");
          break;
        case " ":
        case "Escape":
          stop();
          break;
      }
    };

    const handleKeyUp = (e: KeyboardEvent) => {
      if (["ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight", "w", "W", "a", "A", "s", "S", "d", "D"].includes(e.key)) {
        stop();
      }
    };

    window.addEventListener("keydown", handleKeyDown);
    window.addEventListener("keyup", handleKeyUp);
    
    return () => {
      window.removeEventListener("keydown", handleKeyDown);
      window.removeEventListener("keyup", handleKeyUp);
    };
  }, [move, stop]);

  const showMessage = (text: string, type: "success" | "error") => {
    setMessage({ text, type });
    setTimeout(() => setMessage({ text: "", type: "" }), 2000);
  };

  const handleSpeed = () => {
    setSpeed(speedSlider);
    showMessage(`Speed: ${speedSlider}`, "success");
  };

  return (
    <div className="container">
      <h1>Robot Car Control</h1>

      <div className="control-panel speed-panel">
        <h2>Speed</h2>
        <div className="slider-row">
          <input 
            type="range" 
            min="1" 
            max="10" 
            step="1" 
            value={speedSlider} 
            onChange={e => setSpeedSlider(Number(e.target.value))}
          />
          <span className="speed-value">{speedSlider}</span>
        </div>
        <button className="primary" onClick={handleSpeed}>Set Speed</button>
      </div>

      <div className="control-panel direction-panel">
        <h2>Direction</h2>
        <div className="keyboard-hint">Keyboard: Arrow keys or WASD | Space/Esc to stop</div>
        
        <div className="direction-grid">
          <div></div>
          <button 
            className={`dir-btn up ${direction === "forward" ? "active" : ""}`}
            onMouseDown={() => move("forward")}
            onMouseUp={stop}
            onMouseLeave={stop}
            onTouchStart={(e) => { e.preventDefault(); move("forward"); }}
            onTouchEnd={(e) => { e.preventDefault(); stop(); }}
          >
            <span className="arrow">▲</span>
            <span className="label">Forward</span>
          </button>
          <div></div>

          <button 
            className={`dir-btn left ${direction === "left" ? "active" : ""}`}
            onMouseDown={() => move("left")}
            onMouseUp={stop}
            onMouseLeave={stop}
            onTouchStart={(e) => { e.preventDefault(); move("left"); }}
            onTouchEnd={(e) => { e.preventDefault(); stop(); }}
          >
            <span className="arrow">◄</span>
            <span className="label">Left</span>
          </button>
          
          <button className="stop-btn" onMouseDown={stop} onMouseUp={stop}>
            <span className="stop-icon">■</span>
            <span className="label">STOP</span>
          </button>

          <button 
            className={`dir-btn right ${direction === "right" ? "active" : ""}`}
            onMouseDown={() => move("right")}
            onMouseUp={stop}
            onMouseLeave={stop}
            onTouchStart={(e) => { e.preventDefault(); move("right"); }}
            onTouchEnd={(e) => { e.preventDefault(); stop(); }}
          >
            <span className="arrow">►</span>
            <span className="label">Right</span>
          </button>

          <div></div>
          <button 
            className={`dir-btn down ${direction === "backward" ? "active" : ""}`}
            onMouseDown={() => move("backward")}
            onMouseUp={stop}
            onMouseLeave={stop}
            onTouchStart={(e) => { e.preventDefault(); move("backward"); }}
            onTouchEnd={(e) => { e.preventDefault(); stop(); }}
          >
            <span className="arrow">▼</span>
            <span className="label">Backward</span>
          </button>
          <div></div>
        </div>
      </div>

      {message.text && <div className={`message ${message.type}`}>{message.text}</div>}
    </div>
  );
}

export default App;
