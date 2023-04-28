const { invoke } = window.__TAURI__.tauri;

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

let modelParamsAreDefault;

const plotArea = document.getElementById('plotting-area');
const plotAreaWrapper = document.getElementById('canvas');
const ctx = plotArea.getContext('2d');

let num_ts = 200; // in ms
let stim = [[0, 0.0], [5000, 20.0], [15000, 0.0]];
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

function resetModelParams() {
  if (!modelParamsAreDefault) {
    // brute-force my way hee-haw
    g_l = defaultModelParams["g-leak-max"];
    e_l = defaultModelParams["e-leak"];

    na_params = {...default_na_params};
    k_params = {...default_k_params};
    a_params = {...default_a_params};

    const modelInputs = document.querySelectorAll(".model-params input");
    modelInputs.forEach((input) => {
      input.value = input.defaultValue;
    });
    modelParamsAreDefault = true;
    invoke_plot_command();
  }
}

function handleModelParamsInputChange(element) {
  const id = element.id;
  const pcs = id.split("-");
  
  const var_type = pcs[0];
  let var_name = "";
  if (var_type === "g") {
    var_name = var_type + "_max";
  } else if (var_type === "e") {
    var_name = var_type + "_rev";
  }

  switch (pcs[1]) {
    case "leak":
      if (var_type === "g") {
        g_l = +element.value;
      } else if (var_type === "e") {
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
  // TODO: write this function :-)
}

async function setDefaultModelParams() {
  const modelInputs = document.querySelectorAll(".model-params input");
  modelInputs.forEach((input) => {
    input.defaultValue = defaultModelParams[input.id];
  });
  modelParamsAreDefault = true;
}

async function setDefaultStimParams() {
  let stim_0 = document.getElementById("stim-0");
  let stim_1 = document.getElementById("stim-1");

  stim_0.defaultValue = 0.0;
  stim_1.defaultValue = 0.0;
}

async function invoke_plot_command() {
  await invoke('run_and_plot',
    {
      numTs: num_ts,
      stim: stim,
      gL: g_l,
      eL: e_l,
      naParams: na_params,
      kParams: k_params,
      aParams: a_params
    }).then((plot) => {
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
    }).catch((error) => console.error(error));
}

window.addEventListener("DOMContentLoaded", () => {
  setDefaultModelParams();
  setDefaultStimParams();
  invoke_plot_command();
});

window.addEventListener("resize", () =>invoke_plot_command());

document.querySelectorAll(".model-params input").forEach((input) => {
  input.addEventListener("change", () => handleModelParamsInputChange(input));
});

document.querySelectorAll(".stim-params input").forEach((input) => {
  input.addEventListener("change", () => handleStimParamsInputChange(input));
});

document.getElementById("reset-btn").addEventListener("mouseup", () => resetModelParams());

