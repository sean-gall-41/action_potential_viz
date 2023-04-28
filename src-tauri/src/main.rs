// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct NaParams {
    g_max: f32,
    e_rev: f32,
    act_open_scale: f32,
    act_close_scale: f32,
    act_open_rate: f32,
    act_close_rate: f32,
    inact_open_scale: f32,
    inact_close_scale: f32,
    inact_open_rate: f32,
    inact_close_rate: f32,
    act_v_offset_open: f32,
    act_pre_v_fact_open: f32,
    close_act_exp_const: f32,
    act_v_offset_close: f32,
    open_inact_exp_const: f32,
    inact_v_offset_open: f32,
    inact_pre_v_fact_close: f32,
    inact_v_offset_close: f32
}

impl NaParams {
    fn update_trans_rates(&mut self, voltage: f32) {
        self.act_open_rate = (self.act_open_scale
                               * (voltage - self.act_v_offset_open))
                               / (1.0 - (self.act_pre_v_fact_open * (voltage - self.act_v_offset_open)).exp());
        self.act_close_rate = self.act_close_scale
                                * (self.close_act_exp_const
                                * (voltage - self.act_v_offset_close)).exp();
        self.inact_open_rate = self.inact_open_scale
                             * (self.open_inact_exp_const
                             * (voltage - self.inact_v_offset_open)).exp();
        self.inact_close_rate = self.inact_close_scale
                              / (1.0 + (self.inact_pre_v_fact_close * (voltage - self.inact_v_offset_close)).exp());
    }
}

#[derive(Deserialize, Debug)]
struct KParams {
    g_max: f32,
    e_rev: f32,
    open_scale: f32,
    close_scale: f32,
    open_rate: f32,
    close_rate: f32,
    v_offset_open: f32,
    pre_v_fact_open: f32,
    exp_const_close: f32,
    v_offset_close: f32,
}

impl KParams {
    fn update_trans_rates(&mut self, voltage: f32) {

        self.open_rate = (self.open_scale
                       * (voltage - self.v_offset_open))
                       / (1.0 - (self.pre_v_fact_open * (voltage - self.v_offset_open)).exp());

        self.close_rate = self.close_scale
                        * (self.exp_const_close * (voltage - self.v_offset_close)).exp();
    }
}

#[derive(Deserialize, Debug)]
struct AParams {
    g_max: f32,
    e_rev: f32,
    tau_act: f32,
    inf_act: f32,
    tau_inact: f32,
    inf_inact: f32,
}

impl AParams {
    // TODO: place all these params in AParams :sigh:
    fn update_trans_rates(&mut self, voltage: f32) {
        self.inf_act = (0.0761 * (0.0314 * (voltage + 94.22)).exp())
                     / (1.0 + (0.0346 * (voltage + 1.17)).exp());
        self.inf_act = self.inf_act.powf(1.0 / 3.0);

        self.tau_act = 0.3632 + 1.158 / (1.0 + (0.0497 * (voltage + 55.96)).exp());

        self.inf_inact = 1.0 / (1.0 + (0.0688 * (voltage + 53.3)).exp());
        self.inf_inact = self.inf_inact.powf(4.0);

        self.tau_inact = 1.24 + 2.678 / (1.0 + (0.0624 * (voltage + 50.0)).exp());
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
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn run_and_plot(
    num_ts: u32,
    stim: Vec<(f32, f32)>,
    g_l: f32,
    e_l: f32,
    mut na_params: NaParams,
    mut k_params: KParams,
    mut a_params: AParams) -> Vec<(f32, f32)> {

    let numts = (num_ts as f32 / 0.01) as u32;
    let mut v_plot: Vec<(f32, f32)> = vec![(0.0, 0.0); numts as usize];
    let mut na_act_prob = 0.0159f32;
    let mut na_inact_prob = 0.9437f32;
    let mut k_prob = 0.196f32;
    let mut a_act_prob = 0.0559f32;
    let mut a_inact_prob = 0.2175f32;
    let mut voltage = -64.453f32;
    let mem_c = 1.0f32;

    let mut stim_id = 0usize;
    for ts in 0..numts {
        if stim_id < stim.len() && ts == (stim[stim_id].0) as u32 {
            stim_id += 1;
        }
        na_params.update_trans_rates(voltage);
        k_params.update_trans_rates(voltage);
        a_params.update_trans_rates(voltage);
        let ts: usize = ts as usize;
    
        na_act_prob += ((1.0 - na_act_prob) * na_params.act_open_rate
                     - na_act_prob * na_params.act_close_rate) * 0.01;
    
        na_inact_prob += ((1.0 - na_inact_prob) * na_params.inact_open_rate
                       - na_inact_prob * na_params.inact_close_rate) * 0.01;
    
        k_prob += ((1.0 - k_prob) * k_params.open_rate
                    - k_prob * k_params.close_rate) * 0.01;
    
        // update eqns based off of the asymptotic var inf_act and time const tau
        a_act_prob += 0.01 * (a_params.inf_act - a_act_prob) / a_params.tau_act;
    
        a_inact_prob += 0.01 * (a_params.inf_inact - a_inact_prob) / a_params.tau_inact;

        voltage += (
                -g_l * (voltage - e_l)
                - na_params.g_max * na_act_prob.powf(3.0) * na_inact_prob * (voltage - na_params.e_rev)
                - k_params.g_max * k_prob.powf(4.0) * (voltage - k_params.e_rev)
                - a_params.g_max * a_act_prob.powf(3.0) * a_inact_prob * (voltage - a_params.e_rev)
                + stim[stim_id-1].1
                ) * 0.01;
        voltage /= mem_c;

        v_plot[ts].0 = ts as f32;
        v_plot[ts].1 = voltage;
    }
    v_plot
}

fn main() {
    let context = tauri::generate_context!();
    tauri::Builder::default()
        .menu(tauri::Menu::os_default(&context.package_info().name))
        .invoke_handler(tauri::generate_handler![run_and_plot])
        .run(context)
        .expect("error while running tauri application");
}

