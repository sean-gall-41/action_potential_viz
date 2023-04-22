// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
pub fn plot_logistic_map(a: f32, b: f32, delta: f32, r: f32) -> (Vec<f32>, Vec<f32>) {
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
    pub open_exp_const: f32,
    pub close_exp_const: f32
}

impl NaParams {
    pub fn from(
        g_max: f32,
        e_rev: f32,
        init_act_open_rate: f32,
        init_act_close_rate: f32,
        init_inact_open_rate: f32,
        init_inact_close_rate: f32,
        open_exp_const: f32,
        close_exp_const: f32) -> Self {
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
            open_exp_const: open_exp_const,
            close_exp_const: close_exp_const
        }
    }

    pub fn update_trans_rates(&mut self, voltage: f32) {
        self.act_open_rate = self.init_act_open_rate
                       * (self.open_exp_const * voltage).exp();
        self.act_close_rate = self.init_act_close_rate
                        * (self.close_exp_const * voltage).exp();

        self.inact_open_rate = self.init_inact_open_rate
                       * (self.open_exp_const * voltage).exp();
        self.inact_close_rate = self.init_inact_close_rate
                        * (self.close_exp_const * voltage).exp();
    }
}

pub struct KParams {
    pub g_max: f32,
    pub e_rev: f32,
    pub init_open_rate: f32,
    pub init_close_rate: f32,
    pub open_rate: f32,
    pub close_rate: f32,
    pub open_exp_const: f32,
    pub close_exp_const: f32
}

impl KParams {
    pub fn from(
        g_max: f32,
        e_rev: f32,
        init_open_rate: f32,
        init_close_rate: f32,
        open_exp_const: f32,
        close_exp_const: f32) -> Self {
        KParams {
            g_max: g_max,
            e_rev: e_rev,
            init_open_rate: init_open_rate,
            init_close_rate: init_close_rate,
            open_rate: init_close_rate,
            close_rate: init_close_rate,
            open_exp_const: open_exp_const,
            close_exp_const: close_exp_const
        }
    }

    pub fn update_trans_rates(&mut self, voltage: f32) {
        self.open_rate = self.init_open_rate
                       * (self.open_exp_const * voltage).exp();
        self.close_rate = self.init_close_rate
                        * (self.close_exp_const * voltage).exp();

    }
}

pub struct AParams {
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
    pub open_exp_const: f32,
    pub close_exp_const: f32
}

impl AParams {
    pub fn from(
        g_max: f32,
        e_rev: f32,
        init_act_open_rate: f32,
        init_act_close_rate: f32,
        init_inact_open_rate: f32,
        init_inact_close_rate: f32,
        open_exp_const: f32,
        close_exp_const: f32) -> Self {
        AParams {
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
            open_exp_const: open_exp_const,
            close_exp_const: close_exp_const
        }
    }

    pub fn update_trans_rates(&mut self, voltage: f32) {
        self.act_open_rate = self.init_act_open_rate
                       * (self.open_exp_const * voltage).exp();
        self.act_close_rate = self.init_act_close_rate
                        * (self.close_exp_const * voltage).exp();

        self.inact_open_rate = self.init_inact_open_rate
                       * (self.open_exp_const * voltage).exp();
        self.inact_close_rate = self.init_inact_close_rate
                        * (self.close_exp_const * voltage).exp();
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

            self.a_act_probs[ts] = self.a_act_probs[ts-1]
                                 + (1.0 - self.a_act_probs[ts-1]) * self.a_params.act_open_rate
                                 - self.a_act_probs[ts-1] * self.a_params.act_close_rate;

            self.a_inact_probs[ts] = self.a_inact_probs[ts-1]
                                  + (1.0 - self.a_inact_probs[ts-1]) * self.a_params.inact_open_rate
                                  - self.a_inact_probs[ts-1] * self.a_params.inact_close_rate;

            self.voltage[ts] = self.voltage[ts-1]
                             + self.g_l * (self.stim[stim_id].1 - self.e_l)
                             + self.na_params.g_max * self.na_act_probs[ts].powf(3.0)
                             * self.na_inact_probs[ts]
                             * (self.stim[stim_id].1 - self.na_params.e_rev)
                             + self.k_params.g_max * self.k_probs[ts].powf(4.0)
                             * (self.stim[stim_id].1 - self.k_params.e_rev)
                             + self.a_params.g_max * self.a_act_probs[ts].powf(3.0)
                             * self.a_inact_probs[ts]
                             * (self.stim[stim_id].1 - self.a_params.e_rev)
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
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, plot_logistic_map])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

