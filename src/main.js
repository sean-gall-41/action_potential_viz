const { invoke } = window.__TAURI__.tauri;

const defaultSimParams = {
  "num-ts": 200, // in ms
  "delta-t": 0.01, // also in ms
};

let simParams = {...defaultSimParams};

const defaultModelParams = {
  "g-leak-max": 0.3,
  "e-leak": -17.0,
  "g-na-max": 120.0,
  "e-na": 55.0,
  "g-k-max": 20.0,
  "e-k": -72.0,
  "g-a-max": 47.7,
  "e-a": -75.0
};

let simParamsAreDefault;
let modelParamsAreDefault;
let stimParamsAreDefault;

const plotArea = document.getElementById('plotting-area');
const plotAreaWrapper = document.getElementById('canvas');
const ctx = plotArea.getContext('2d');

let stim = [[0, 0.0]];
let g_l = 0.3;
let e_l = -17.0;

const default_na_params = {
  g_max: 120.0,
  e_rev: 55.0,
  act_open_scale: 0.38,
  act_close_scale: 15.2,
  act_open_rate: 0.0,
  act_close_rate: 0.0,
  inact_open_scale: 0.266,
  inact_close_scale: 3.8,
  inact_open_rate: 0.0,
  inact_close_rate: 0.0,
  act_v_offset_open: -29.7,
  act_pre_v_fact_open: -0.1,
  close_act_exp_const: -0.0556,
  act_v_offset_close: -54.7,
  open_inact_exp_const: -0.05,
  inact_v_offset_open: -48.0,
  inact_pre_v_fact_close: -0.1,
  inact_v_offset_close: -18.0
};

const default_k_params = {
  g_max: 20.0,
  e_rev: -72.0,
  open_scale: 0.02,
  close_scale: 0.25,
  open_rate: 0.0,
  close_rate: 0.0,
  v_offset_open: -45.7,
  pre_v_fact_open: -0.1,
  exp_const_close: -0.0125,
  v_offset_close: -55.7
};

const default_a_params = {
  g_max: 47.7,
  e_rev: -75.0,
  tau_act: 0.0,
  inf_act: 0.0,
  tau_inact: 0.0,
  inf_inact: 0.0
};

let na_params = {...default_na_params};
let k_params = {...default_k_params};
let a_params = {...default_a_params};

function resetParams() {
  if (!simParamsAreDefault || !modelParamsAreDefault || !stimParamsAreDefault) {
    simParams = {...defaultSimParams};
    // brute-force my way hee-haw
    g_l = defaultModelParams["g-leak-max"];
    e_l = defaultModelParams["e-leak"];

    na_params = {...default_na_params};
    k_params = {...default_k_params};
    a_params = {...default_a_params};

    document.querySelectorAll(".sim-params input").forEach((input) => {
      input.value = input.defaultValue;
    });

    document.querySelectorAll(".model-params input").forEach((input) => {
      input.value = input.defaultValue;
    });

    document.querySelectorAll(".stim-params input").forEach((input) => {
      input.value = input.defaultValue;
    });

    stim = [[0, 0.0]];
    let [stim_ts_0, stim_curr_0] = [
      document.getElementById("stim-ts-0"),
      document.getElementById("stim-curr-0")
    ];
    stim.push([+stim_ts_0.value / defaultSimParams["delta-t"], +stim_curr_0.value]);

    simParamsAreDefault = true;
    modelParamsAreDefault = true;
    stimParamsAreDefault = true;
    invoke_plot_command();
  }
}

function handleSimParamsInputChange(element) {
  simParams[element.id] = +element.value;
  if (simParamsAreDefault) simParamsAreDefault = false;
  invoke_plot_command();
}

function handleModelParamsInputChange(element) {
  const id = element.id;
  const pcs = id.split("-");
  
  const varType = pcs[0];
  let var_name = "";
  if (varType === "g") {
    var_name = varType + "_max";
  } else if (varType === "e") {
    var_name = varType + "_rev";
  }

  switch (pcs[1]) {
    case "leak":
      if (varType === "g") {
        g_l = +element.value;
      } else if (varType === "e") {
        e_l = +element.value;
      }
      break;
    case "na":
      na_params[var_name] = +element.value;
      break;
    case "k":
      k_params[var_name] = +element.value;
      break;
    case "a":
      a_params[var_name] = +element.value;
      break;
    default:
      console.error("bad programmer is bad");
  }
  if (modelParamsAreDefault) modelParamsAreDefault = false;
  invoke_plot_command();
}

function handleStimParamsInputChange(element) {
  const id = element.id;
  const pcs = id.split("-");

  const paramElem = element.parentElement;
  const errorElem = paramElem.querySelector('.error');

  const varType = pcs[1];
  const numInSeq = +pcs[2];
  if (varType === "ts") {
    if (+element.value > simParams["num-ts"]) {
      errorElem.classList.add("error-show");
    } else {
      errorElem.classList.remove("error-show");
      stim[numInSeq+1][0] = +element.value / simParams["delta-t"];
      invoke_plot_command();
    }
  } else if (varType === "curr") {
    stim[numInSeq+1][1] = +element.value;
    invoke_plot_command();
  }
}

async function setDefaultSimParams() {
  const simInputs = document.querySelectorAll(".sim-params input");
  simInputs.forEach((input) => {
    input.defaultValue = defaultSimParams[input.id];
  });
}

async function setDefaultModelParams() {
  const modelInputs = document.querySelectorAll(".model-params input");
  modelInputs.forEach((input) => {
    input.defaultValue = defaultModelParams[input.id];
  });
  modelParamsAreDefault = true;
}

 /*
  * assume that the first value of the stim array is [0, 0.0]
  * also, user sees ts in units of ms
  */
async function setDefaultStimParams() {
  let [stim_ts_0, stim_curr_0] = [
    document.getElementById("stim-ts-0"),
    document.getElementById("stim-curr-0")
  ];

  stim_ts_0.defaultValue = 20;
  stim_curr_0.defaultValue = 13;
  stim.push([+stim_ts_0.value / defaultSimParams["delta-t"], +stim_curr_0.value]);
}

async function invoke_plot_command() {
  await invoke('run_and_plot',
    {
      numTs: simParams["num-ts"],
      deltaT: simParams["delta-t"],
      stim: stim,
      gL: g_l,
      eL: e_l,
      naParams: na_params,
      kParams: k_params,
      aParams: a_params
    }).then((plot) => {
      // plotting of the main current trace
      let abs_min_y = Math.abs(Math.min(...plot.map((row) => row[1])));
      plotArea.width = plotAreaWrapper.getBoundingClientRect().width;
      plotArea.height = plotArea.offsetHeight;
      ctx.strokeStyle = '#bfbfff';
      ctx.lineWidth = 0.5;
      ctx.beginPath();
      plot.forEach(([ts, v_val]) => {
        let x = (ts / plot.length) * plotArea.width;
        let y = -(v_val + abs_min_y) * 0.004 * plotArea.height + 0.70 * plotArea.height;
        ctx.lineTo(x, y);
      });
      ctx.stroke();

      // prepare the stim data to plot
      let stimPlot = [];
      for (let i = 0; i < stim.length; i++) {
        stimPlot.push([stim[i][0] * simParams["delta-t"], stim[i][1]]);
        if (i == stim.length - 1) {
          stimPlot.push([simParams["num-ts"], stim[i][1]]);
        } else {
          stimPlot.push([stim[i+1][0] * simParams["delta-t"], stim[i][1]]);
        }
      }
      
      // plot the stimulus underneath the current trace
      ctx.strokeStyle = '#ffffff';
      ctx.beginPath();
      let scale = 0.9;
      stimPlot.forEach(([ts, i_val]) => {
        let x = (ts / simParams["num-ts"]) * plotArea.width;
        // DEBUG: ensure user inputs a valid value so that current trace and stim
        // trace dont overlap
        let y =  0.95 * plotArea.height - scale * (i_val - stimPlot[0][1]);
        ctx.lineTo(x, y);
      });
      ctx.stroke();
    }).catch((error) => console.error(error));
}

window.addEventListener("DOMContentLoaded", () => {
  setDefaultSimParams();
  setDefaultModelParams();
  setDefaultStimParams();
  invoke_plot_command();
});

window.addEventListener("resize", () => invoke_plot_command());

document.querySelectorAll(".sim-params input").forEach((input) => {
  input.addEventListener("change", () => handleSimParamsInputChange(input));
});

document.querySelectorAll(".model-params input").forEach((input) => {
  input.addEventListener("change", () => handleModelParamsInputChange(input));
});

document.querySelectorAll(".stim-params input").forEach((input) => {
  input.addEventListener("change", () => handleStimParamsInputChange(input));
  input.addEventListener("blur", () => {
    const id = input.id;
    const pcs = id.split("-");

    const paramElem = input.parentElement;
    const errorElem = paramElem.querySelector('.error');

    const varType = pcs[1];
    const numInSeq = +pcs[2];

    if (errorElem.classList.contains('error-show')) {
      errorElem.classList.remove("error-show");
      input.value = stim[numInSeq+1][0] * simParams["delta-t"];
    }
  });
});

document.getElementById("reset-btn").addEventListener("mouseup", () => resetParams());

