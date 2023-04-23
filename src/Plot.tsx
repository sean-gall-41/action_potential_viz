import { useState, useEffect } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/tauri";
import Plotly from 'plotly.js-dist';

async function getData(a: number, b: number, r: number)
  : Promise<{ x: number[], y: number[] }> {
  const [x, y] = await invoke(
    'plot_logistic_map',
    { a, b, delta: 0.001, r }
  ) as [number[], number[]];
  return { x, y };
}

const PlotComponent = () => {
  const [rVal, setRVal] = useState(1);
  const pVal = 0.1;
  const mVal = 0.4;

  const handleChange = async (e) => {
    const r = +e.target.value;
    setRVal(r);
  };

  const plotLayout = {
    margin: { t: 10 },
    yaxis: {
      range: [0, 1]
    }
  };

  useEffect(() => {
    const plotDiv = document.getElementById('plot');

    const generatePlot = async () => {
      const plotData = [await getData(0, 1, rVal)];
      Plotly.newPlot(plotDiv, plotData, plotLayout);
    };

    if (plotDiv) {
      generatePlot();
    }
  }, [rVal]);

  return (
    <div>
      <div className="row">
        <label htmlFor="rParam">r:</label>
        <input type="number" step="0.01" id="rParam" value={rVal} onChange={handleChange} />
        <label htmlFor="pParam">p:</label>
        <input type="number" step="0.01" id="pParam" value={pVal} onChange={handleChange} />
        <label htmlFor="mParam">m:</label>
        <input type="number" step="0.01" id="mParam" value={mVal} onChange={handleChange} />
      </div>

      <div className="row">
        <div id="plot"></div>
      </div>
    </div>
  );
};

export default PlotComponent;
