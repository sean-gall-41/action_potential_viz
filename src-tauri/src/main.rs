// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn plot_logistic_map(a: f32, b: f32, delta: f32, r: f32) -> (Vec<f32>, Vec<f32>) {
    let mut res = (vec![], vec![]);
    let mut x = a;
    while x <= b {
        res.0.push(x);
        res.1.push(logistic_map(r, x));
        x += delta;
    }
    res
}
pub fn logistic_map(r: f32, x: f32) -> f32 {
    r * x * (1.0 - x)
}

pub struct NaParams {
    pub g_max: f32,
    pub e_rev: f32,
    pub init_act_open_rate: f32,
    pub init_act_close_rate: f32,
    pub act_open_rate: f32,
    pub act_close_rate: f32,
    pub init_inact_open_rate: f32,
    pub init_inact_close_rate: f32,
    pub inact_open_rate: f32,
    pub inact_close_rate: f32,
    pub act_v_offset_open: f32,
    pub act_pre_v_fact_open: f32,
    pub close_act_exp_const: f32,
    pub act_v_offset_close: f32,
    pub open_inact_exp_const: f32,
    pub inact_v_offset_open: f32,
    pub inact_pre_v_fact_close: f32,
    pub inact_v_offset_close: f32
}

impl NaParams {
    pub fn from(
        g_max: f32,
        e_rev: f32,
        init_act_open_rate: f32,
        init_act_close_rate: f32,
        init_inact_open_rate: f32,
        init_inact_close_rate: f32,
        act_v_offset_open: f32,
        act_pre_v_fact_open: f32,
        close_act_exp_const: f32,
        act_v_offset_close: f32,
        open_inact_exp_const: f32,
        inact_v_offset_open: f32,
        inact_pre_v_fact_close: f32,
        inact_v_offset_close: f32) -> Self {
        NaParams {
            g_max: g_max,
            e_rev: e_rev,
            init_act_open_rate: init_act_open_rate,
            init_act_close_rate: init_act_close_rate,
            act_open_rate: init_act_close_rate,
            act_close_rate: init_act_close_rate,
            init_inact_open_rate: init_inact_open_rate,
            init_inact_close_rate: init_inact_close_rate,
            inact_open_rate: init_inact_close_rate,
            inact_close_rate: init_inact_close_rate,
            act_v_offset_open: act_v_offset_open,
            act_pre_v_fact_open: act_pre_v_fact_open,
            close_act_exp_const: close_act_exp_const,
            act_v_offset_close: act_v_offset_close,
            open_inact_exp_const: open_inact_exp_const,
            inact_v_offset_open: inact_v_offset_open,
            inact_pre_v_fact_close: inact_pre_v_fact_close,
            inact_v_offset_close: inact_v_offset_close
        }
    }

    pub fn update_trans_rates(&mut self, voltage: f32) {
        self.act_open_rate = (self.init_act_open_rate
                               * (voltage - self.act_v_offset_open))
                               / (1.0 - (self.act_pre_v_fact_open * (voltage - self.act_v_offset_open)).exp());
        self.act_close_rate = self.init_act_close_rate
                                * (self.close_act_exp_const
                                * (voltage - self.act_v_offset_close)).exp();
        self.inact_open_rate = self.init_inact_open_rate
                             * (self.open_inact_exp_const
                             * (voltage - self.inact_v_offset_open)).exp();
        self.inact_close_rate = self.init_inact_close_rate
                              / (1.0 + (self.inact_pre_v_fact_close * (voltage - self.inact_v_offset_close)).exp());
    }
}

pub struct KParams {
    pub g_max: f32,
    pub e_rev: f32,
    pub init_open_rate: f32,
    pub init_close_rate: f32,
    pub open_rate: f32,
    pub close_rate: f32,
    pub v_offset_open: f32,
    pub pre_v_fact_open: f32,
    pub exp_const_close: f32,
    pub v_offset_close: f32,
}

impl KParams {
    pub fn from(
        g_max: f32,
        e_rev: f32,
        init_open_rate: f32,
        init_close_rate: f32,
        v_offset_open: f32,
        pre_v_fact_open: f32,
        exp_const_close: f32,
        v_offset_close: f32) -> Self {
        KParams {
            g_max: g_max,
            e_rev: e_rev,
            init_open_rate: init_open_rate,
            init_close_rate: init_close_rate,
            open_rate: init_close_rate,
            close_rate: init_close_rate,
            v_offset_open: v_offset_open,
            pre_v_fact_open: pre_v_fact_open,
            exp_const_close: exp_const_close,
            v_offset_close: v_offset_close
        }
    }

    pub fn update_trans_rates(&mut self, voltage: f32) {

        self.open_rate = (self.init_open_rate
                       * (voltage - self.v_offset_open))
                       / (1.0 - (-self.pre_v_fact_open * (voltage - self.v_offset_open)).exp());

        self.close_rate = self.init_close_rate
                        * (self.exp_const_close * (voltage - self.v_offset_close)).exp();
    }
}

pub struct AParams {
    pub g_max: f32,
    pub e_rev: f32,
    pub tau_act: f32,
    pub inf_act: f32,
    pub tau_inact: f32,
    pub inf_inact: f32,
}

impl AParams {
    pub fn from(
        g_max: f32,
        e_rev: f32,
        tau_act: f32,
        inf_act: f32,
        tau_inact: f32,
        inf_inact: f32) -> Self {
        AParams {
            g_max: g_max,
            e_rev: e_rev,
            tau_act: tau_act,
            inf_act: inf_act,
            tau_inact: tau_inact,
            inf_inact: inf_inact
        }
    }

    // TODO: place all these params in AParams :sigh:
    pub fn update_trans_rates(&mut self, voltage: f32) {
        self.inf_act = (0.0761 * (0.0314 * (voltage + 94.22)).exp())
                     / (1.0 + (0.0346 * (voltage + 1.17)).exp());
        self.inf_act = self.tau_act.powf(1.0 / 3.0);

        self.tau_act = 0.3632 + (1.158 / (1.0 + (0.0497 * (voltage + 55.96)).exp()));

        self.inf_inact = 1.0 / (1.0 + (0.0688 * (voltage + 53.3)).exp());
        self.inf_inact = self.inf_inact.powf(4.0);

        self.tau_inact = 1.24 + (2.678 / (1.0 + (0.0624 * (voltage + 50.0)).exp()));
    }
}

/*
 * definition of the Connor-Stevens Model
 *  includes:
 *      - leak conduct
 *      - transient na conduct
 *      - persistent k conduct
 *      - transient k "A-type" current
 */
pub struct CSSim {
    pub num_ts: u32,
    pub stim: Vec<(f32, f32)>,
    pub g_l: f32,
    pub e_l: f32,
    pub na_params: NaParams,
    pub k_params: KParams,
    pub a_params: AParams,
    pub na_act_probs: Vec<f32>,
    pub na_inact_probs: Vec<f32>,
    pub k_probs: Vec<f32>,
    pub a_act_probs: Vec<f32>,
    pub a_inact_probs: Vec<f32>,
    pub voltage: Vec<f32>
}

impl CSSim {
    pub fn from(
        num_ts: u32,
        stim: Vec<(f32, f32)>,
        g_l: f32,
        e_l: f32,
        na_params: NaParams,
        k_params: KParams,
        a_params: AParams) -> Self {
        CSSim {
            num_ts: num_ts,
            stim: vec![(0.0, -60.0), (50.0, 0.0), (100.0, -60.0)],
            g_l: g_l,
            e_l: e_l,
            na_params: na_params,
            k_params: k_params,
            a_params: a_params,
            na_act_probs: vec![0.0; num_ts as usize],
            na_inact_probs: vec![0.0; num_ts as usize],
            k_probs: vec![0.0; num_ts as usize],
            a_act_probs: vec![0.0; num_ts as usize],
            a_inact_probs: vec![0.0; num_ts as usize],
            voltage: vec![0.0; num_ts as usize]
        }
    }

    pub fn run(&mut self) {
        let mut stim_id: usize = 0;
        for ts in 1..self.num_ts {
            if stim_id < self.stim.len() && (ts-1) == (self.stim[stim_id].0) as u32 {
                self.na_params.update_trans_rates(self.stim[stim_id].1);
                self.k_params.update_trans_rates(self.stim[stim_id].1);
                self.a_params.update_trans_rates(self.stim[stim_id].1);
                stim_id += 1;
            }
            let ts: usize = ts as usize;

            self.na_act_probs[ts] = self.na_act_probs[ts-1]
                                  + (1.0 - self.na_act_probs[ts-1]) * self.na_params.act_open_rate
                                  - self.na_act_probs[ts-1] * self.na_params.act_close_rate;

            self.na_inact_probs[ts] = self.na_inact_probs[ts-1]
                                  + (1.0 - self.na_inact_probs[ts-1]) * self.na_params.inact_open_rate
                                  - self.na_inact_probs[ts-1] * self.na_params.inact_close_rate;

            self.k_probs[ts] = self.k_probs[ts-1]
                                  + (1.0 - self.k_probs[ts-1]) * self.k_params.open_rate
                                  - self.k_probs[ts-1] * self.k_params.close_rate;

            // update eqns based off of the asymptotic var inf_act and time const tau
            self.a_act_probs[ts] = self.a_act_probs[ts-1]
                                 + ((self.a_params.inf_act - self.a_act_probs[ts-1])
                                 / self.a_params.tau_act);

            self.a_inact_probs[ts] = self.a_inact_probs[ts-1]
                                 + ((self.a_params.inf_inact - self.a_inact_probs[ts-1])
                                 / self.a_params.tau_inact);

            self.voltage[ts] = self.voltage[ts-1]
                             + self.g_l * (self.stim[stim_id-1].1 - self.e_l)
                             + self.na_params.g_max * self.na_act_probs[ts].powf(3.0)
                             * self.na_inact_probs[ts]
                             * (self.stim[stim_id-1].1 - self.na_params.e_rev)
                             + self.k_params.g_max * self.k_probs[ts].powf(4.0)
                             * (self.stim[stim_id-1].1 - self.k_params.e_rev)
                             + self.a_params.g_max * self.a_act_probs[ts].powf(3.0)
                             * self.a_inact_probs[ts]
                             * (self.stim[stim_id-1].1 - self.a_params.e_rev)
        }
    }

    pub fn plot(&self) -> Vec<(f32, f32)> {
        let mut v_plot: Vec<(f32, f32)> = vec![(0.0, 0.0); self.num_ts as usize];
        for ts in 0..self.num_ts as usize {
            v_plot[ts].0 = ts as f32;
            v_plot[ts].1 = self.voltage[ts];
        }
        v_plot
    }
}

fn main() {

    let na_params = NaParams::from(
        1.2,
        55.0,
        0.38,
        15.2,
        0.266,
        3.8,
        -29.7,
        -0.1,
        -0.0556,
        -54.7,
        -0.05,
        -48.0,
        -0.1,
        -18.0);

    let k_params = KParams::from(
        0.2,
        -72.0,
        0.02,
        0.25,
        -45.7,
        -0.1,
        -0.0125,
        -55.7);

    let a_params = AParams::from(
        0.477,
        -75.0,
        0.0,
        0.0,
        0.0,
        0.0);

    let mut cs_sim = CSSim::from(
        200,
        vec![(0.0, 0.0)],
        0.003,
        -17.0,
        na_params,
        k_params,
        a_params);

    println!("before run");
    cs_sim.run();
    println!("after run");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, plot_logistic_map])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

