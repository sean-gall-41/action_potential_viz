const { invoke } = window.__TAURI__.tauri;

const defaultModelParams = {
  "g-leak-max": 0.003,
  "e-leak": -17.0,
  "g-na-max": 1.2,
  "e-na": 55.0,
  "g-k-max": 0.2,
  "e-k": -72.0,
  "g-a-max": 0.477,
  "e-a": -75.0
};

const plotArea = document.getElementById('plotting-area');
const plotAreaWrapper = document.getElementById('canvas');
const ctx = plotArea.getContext('2d');

let num_ts = 200; // in ms
let stim = [[0, 0.0], [5000, 20.0], [15000, 0.0]];
let g_l = 0.3;
let e_l = -17.0;

let na_params = {
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

let k_params = {
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

let a_params = {
  g_max: 47.7,
  e_rev: -75.0,
  tau_act: 0.0,
  inf_act: 0.0,
  tau_inact: 0.0,
  inf_inact: 0.0
};

async function setDefaultModelParams() {
  let g_l = document.getElementById("g-leak-max");
  let e_l = document.getElementById("e-leak");

  let g_na = document.getElementById("g-na-max");
  let e_na = document.getElementById("e-na");

  let g_k = document.getElementById("g-k-max");
  let e_k = document.getElementById("e-k");

  let g_a = document.getElementById("g-a-max");
  let e_a = document.getElementById("e-a");

  g_l.defaultValue = defaultModelParams["g-leak-max"];
  e_l.defaultValue = defaultModelParams["e-leak"];

  g_na.defaultValue = defaultModelParams["g-na-max"];
  e_na.defaultValue = defaultModelParams["e-na"];

  g_k.defaultValue = defaultModelParams["g-k-max"];
  e_k.defaultValue = defaultModelParams["e-k"];

  g_a.defaultValue = defaultModelParams["g-a-max"];
  e_a.defaultValue = defaultModelParams["e-a"];
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
      plotArea.height = plotAreaWrapper.getBoundingClientRect().height;
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
  invoke_plot_command();
});

window.addEventListener("resize", () => invoke_plot_command());
