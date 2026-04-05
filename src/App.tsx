import React from "react";
import { invoke } from "@tauri-apps/api/core";

interface Position {
  x: number;
  y: number;
  z: number;
}

interface RobotStatus {
  position: Position;
  speed: number;
}

interface RobotState {
  position: Position;
  speed: number;
}

const useRobotState = () => {
  const [state, setState] = React.useState<RobotState>({
    position: { x: 0, y: 0, z: 0 },
    speed: 1,
  });

  const refreshStatus = React.useCallback(async () => {
    try {
      const status = await invoke<RobotStatus>("get_status");
      setState(status);
    } catch (e) {
      console.error("Failed to get status:", e);
    }
  }, []);

  const moveRobot = React.useCallback(async (x: number, y: number, z: number) => {
    try {
      await invoke("move_robot", { x, y, z });
      await refreshStatus();
    } catch (e) {
      console.error("Failed to move robot:", e);
    }
  }, [refreshStatus]);

  const setSpeed = React.useCallback(async (speed: number) => {
    try {
      await invoke("set_speed", { speed });
      await refreshStatus();
    } catch (e) {
      console.error("Failed to set speed:", e);
    }
  }, [refreshStatus]);

  return { state, refreshStatus, moveRobot, setSpeed };
};

function App() {
  const { state, refreshStatus, moveRobot, setSpeed } = useRobotState();
  const [inputX, setInputX] = React.useState(0);
  const [inputY, setInputY] = React.useState(0);
  const [inputZ, setInputZ] = React.useState(0);
  const [speedSlider, setSpeedSlider] = React.useState(1);
  const [message, setMessage] = React.useState({ text: "", type: "" });

  React.useEffect(() => {
    refreshStatus();
  }, [refreshStatus]);

  const showMessage = (text: string, type: "success" | "error") => {
    setMessage({ text, type });
    setTimeout(() => setMessage({ text: "", type: "" }), 3000);
  };

  const handleMove = () => {
    moveRobot(inputX, inputY, inputZ);
    showMessage(`Robot moved to (${inputX}, ${inputY}, ${inputZ})`, "success");
  };

  const handleSpeed = () => {
    setSpeed(speedSlider);
    showMessage(`Speed set to ${speedSlider}`, "success");
  };

  return (
    <div className="container">
      <h1>Robot SSR Control</h1>

      <div className="status-panel">
        <h2>Status</h2>
        <div className="coords">
          <span>X: <span className="value">{state.position.x}</span></span>
          <span>Y: <span className="value">{state.position.y}</span></span>
          <span>Z: <span className="value">{state.position.z}</span></span>
          <span>Speed: <span className="value">{state.speed}</span></span>
        </div>
        <button onClick={refreshStatus}>Refresh</button>
      </div>

      <div className="control-panel">
        <h2>Move Robot</h2>
        <div className="input-group">
          <label>
            X: <input type="number" value={inputX} onChange={e => setInputX(Number(e.target.value))} />
          </label>
          <label>
            Y: <input type="number" value={inputY} onChange={e => setInputY(Number(e.target.value))} />
          </label>
          <label>
            Z: <input type="number" value={inputZ} onChange={e => setInputZ(Number(e.target.value))} />
          </label>
        </div>
        <button className="primary" onClick={handleMove}>Move</button>
      </div>

      <div className="control-panel">
        <h2>Set Speed</h2>
        <div className="input-group">
          <label>
            Speed: 
            <input 
              type="range" 
              min="0.1" 
              max="10" 
              step="0.1" 
              value={speedSlider} 
              onChange={e => setSpeedSlider(Number(e.target.value))}
            />
            <span className="speed-value">{speedSlider}</span>
          </label>
        </div>
        <button className="primary" onClick={handleSpeed}>Apply</button>
      </div>

      {message.text && <div className={`message ${message.type}`}>{message.text}</div>}
    </div>
  );
}

export default App;
