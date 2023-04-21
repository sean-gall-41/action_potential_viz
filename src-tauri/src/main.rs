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
    pub g_na: f32,
    pub e_na: f32,
    pub g_k: f32,
    pub e_k: f32,
    pub g_a: f32,
    pub e_a: f32,
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
        g_na: f32,
        e_na: f32,
        g_k: f32,
        e_k: f32,
        g_a: f32,
        e_a: f32,
    ) -> Self {
        CSSim {
            num_ts: num_ts,
            stim: vec![(0.0, -60.0), (50.0, 0.0), (100, -60.0)],
            g_l: g_l,
            e_l: e_l,
            g_na: g_na,
            e_na: e_na,
            g_k: g_k,
            e_k: e_k,
            g_a: g_a,
            e_a: e_a,
            na_act_probs: vec![0.0; num_ts],
            na_inact_probs: vec![0.0; num_ts],
            k_probs: vec![0.0; num_ts],
            a_act_probs: vec![0.0; num_ts],
            a_inact_probs: vec![0.0; num_ts],
            voltage: vec![0.0; num_ts]
        }
    }

    pub update_probs(&mut self, voltage: f32) {
        todo!("IMPLEMENT THIS FUNCTION");
    }

    pub run(&mut self) {
        let mut stim_id: usize = 0;
        for ts in 1..self.num_ts {
            if stim_id < self.stim.len() && (ts-1) == (self.stim[stim_id].0) as u32 {
                self.update_probs(self.stim[stim_id].1);
                stim_id += 1;
            }
            let ts: usize = ts as usize;

            self.na_act_probs[ts] = self.na_act_probs[ts-1]
                                  + (1.0 - self.na_act_probs[ts-1])
                                  - self.na_act_probs[ts-1];

            self.na_inact_probs[ts] = self.na_inact_probs[ts-1]
                                  + (1.0 - self.na_inact_probs[ts-1])
                                  - self.na_inact_probs[ts-1];

            self.k_probs[ts] = self.k_probs[ts-1]
                                  + (1.0 - self.k_probs[ts-1])
                                  - self.k_probs[ts-1];

            self.a_act_probs[ts] = self.a_act_probs[ts-1]
                                 + (1.0 - self.a_act_probs[ts-1])
                                 - self.a_act_probs[ts-1];

            self.a_inact_probs[ts] = self.a_inact_probs[ts-1]
                                  + (1.0 - self.a_inact_probs[ts-1])
                                  - self.a_inact_probs[ts-1];

            self.voltage[ts] = self.voltage[ts-1]
                             + self.g_l * (self.stim[stim_id].1 - self.e_l)
                             + self.g_na * self.na_act_probs[ts].powf(3.0)
                             * self.na_inact_probs[ts]
                             * (self.stim[stim_id].1 - self.e_na)
                             + self.g_k * self.k_probs[ts].powf(4.0)
                             * (self.stim[stim_id].1 - self.e_k)
                             + self.g_a * self.a_act_probs[ts].powf(3.0)
                             * self.a_inact_probs[ts]
                             * (self.stim[stim_id].1 - self.e_a)
        }
    }

    pub plot(&self) -> Vec<(f32, f32)> {
        let mut v_plot: Vec<(f32, f32)> = vec![(0.0, 0.0); self.num_ts];
        for ts in 0..self.num_ts {
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

