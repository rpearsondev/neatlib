use std::{ops::RangeInclusive, fmt::Debug};
use bevy::prelude::*;
use bevy_egui::{egui::{self, emath, WidgetText, Ui, Align2, RichText, Color32, FontId}, EguiContext, EguiPlugin};
use gluesql::core::chrono::{DateTime, Utc};
use crate::{neat::{trainer::{configuration::OffSpringMode, generation_stats::GenerationStats, neat_trainer_host::models::generic_operation::GenericOperation}}, renderer::renderer::{NeatTrainerState}, activation_functions::ActivationFunction, common::NeatFloat};
use super::{sql_query::sql_query_window::SqlQueryPlugin};
use strum::{IntoEnumIterator};

pub struct NeatSettingsGui;
pub struct NeatSettingsGuiState{
    pub is_setting_open: bool,
    pub is_save_open: bool,
    pub is_load_open: bool,
    pub show_network: bool,
    pub show_substrate: bool,
    pub show_events_query: bool
}

impl Plugin for NeatSettingsGui{
    fn build(&self, app: &mut App) {
        app 
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_plugin(SqlQueryPlugin{})
        .add_startup_system(configure_visuals)
        .insert_resource(NeatSettingsGuiState{
            is_setting_open: true,
            show_network: false,
            show_substrate: false,
            show_events_query: false,
            is_save_open: false,
            is_load_open: false
        })
        .add_system(show_menu)
        .add_system(save_run)
        .add_system(load_run)
        .add_system(neat_settings);
    }
}

fn show_menu(mut egui_ctx: ResMut<EguiContext>, mut state: ResMut<NeatSettingsGuiState>, mut trainer_state: ResMut<NeatTrainerState>) {
  
    egui::TopBottomPanel::top("top_panel").show(egui_ctx.ctx_mut(), |ui| {
        egui::menu::bar(ui, |ui| {
            ui.group(|ui| {
                egui::menu::menu_button(ui, "🖵 Windows", |ui| {
                    
                    ui.checkbox(&mut state.show_network , "show network");
                    if trainer_state.configuration.hyperneat_substrate_set.is_some(){
                        ui.checkbox(&mut state.show_substrate , "show substrate");
                    }
                    ui.checkbox(&mut state.show_events_query , "show events query");
                    if ui.button("quit").clicked() {
                        std::process::exit(0);
                    }
                });

                if ui.button("⚙").clicked(){
                    state.is_setting_open = !state.is_setting_open;
                }

                if ui.button("💾").clicked(){
                    state.is_save_open = !state.is_save_open;
                }
                
                if ui.button("📂").clicked(){
                    state.is_load_open = !state.is_load_open;
                }
            });
          
            let is_not_running = trainer_state.run_until == Some(0) || (trainer_state.run_until.is_some() && trainer_state.run_until.unwrap() <= trainer_state.current_generation);
            let is_running = !is_not_running;
            ui.group(|ui| {
                ui.set_enabled(is_not_running);
                if ui.button("▶").clicked() {
                    trainer_state.run_until = None;
                }
                if ui.button("⏵").clicked() {
                    trainer_state.run_until = Some(trainer_state.current_generation + 1);
                };
                if ui.button("⏵ 10").clicked() {
                    trainer_state.run_until = Some(trainer_state.current_generation + 10);
                };
                if ui.button("↺").clicked() {
                    trainer_state.run_until = Some(0);
                    trainer_state.reset_requested = true;
                };
            });
            ui.group(|ui| {
                ui.set_enabled(is_running);
                if ui.button("⏸").clicked() {
                    trainer_state.run_until = Some(trainer_state.current_generation-1);
                };
            });

        });
    
    });
    egui::Area::new("top-right").anchor(Align2::RIGHT_TOP, [0.0,-20.0]).show(egui_ctx.ctx_mut(), |ui| {
       
        ui.horizontal(|ui|{
            ui.label(format!("Generation: {}", trainer_state.current_generation));
        
            let best_so_far = &trainer_state.best_member_so_far;
            if best_so_far.is_some(){
                let member = best_so_far.as_ref().unwrap();
                ui.label(format!("Best: {:.4}", member.genome.objective_fitness.unwrap()));
            }
        });
    });
}

fn configure_visuals(mut egui_ctx: ResMut<EguiContext>) {
    egui_ctx.ctx_mut().set_visuals(egui::Visuals::light());
}

fn neat_settings(
    mut egui_ctx: ResMut<EguiContext>,
    mut trainer_state: ResMut<NeatTrainerState>,
    mut state: ResMut<NeatSettingsGuiState>
) {
    egui::Window::new("Neat Configuration")
    .vscroll(true)
    .open(&mut state.is_setting_open)
    .show(egui_ctx.ctx_mut(), |ui| {
        ui.collapsing("Population", |ui| {
            table(ui, |ui| {
                add_slider_row(ui, "population_size", &mut trainer_state.configuration.population_size, 1..=20000);
                add_slider_row(ui, "survival_threshold", &mut trainer_state.configuration.survival_threshold, 0.0..=1.0);
                add_slider_row(ui, "target_species", &mut trainer_state.configuration.target_species, 1..=500);
                add_slider_row(ui, "genome_minimal_genes_to_connect_ratio", &mut trainer_state.configuration.genome_minimal_genes_to_connect_ratio, 0.0..=1.0);
                add_slider_row(ui, "speciation_drop_species_no_improvement_generations", &mut trainer_state.configuration.speciation_drop_species_no_improvement_generations, 1..=1000);
                add_slider_row(ui, "speciation_new_species_protected_for_generations", &mut trainer_state.configuration.speciation_new_species_protected_for_generations, 2..=1000);
                add_slider_row(ui, "speciation_min_threshold", &mut trainer_state.configuration.speciation_min_threshold, 0.02..=50.0);
                add_slider_row(ui, "speciation_max_threshold", &mut trainer_state.configuration.speciation_max_threshold, 0.02..=50.0);
                add_slider_row(ui, "speciation_cross_species_reproduction_scale", &mut trainer_state.configuration.speciation_cross_species_reproduction_scale, 0.0..=0.5);
                ui.label("speciation_add_new_species_during_run");
                ui.checkbox(&mut trainer_state.configuration.speciation_add_new_species_during_run, "");
                ui.end_row();

                ui.label("speciation_add_best_member_back_in");
                ui.checkbox(&mut trainer_state.configuration.speciation_add_best_member_back_in, "");
                ui.end_row();

                ui.label("offspring_mode");
                egui::ComboBox::from_label("ofs")
                    .selected_text(format!("{:?}", trainer_state.configuration.speciation_offspring_mode))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut trainer_state.configuration.speciation_offspring_mode, OffSpringMode::AdjustedMemberRange, "AdjustedMemberRange");
                        ui.selectable_value(&mut trainer_state.configuration.speciation_offspring_mode, OffSpringMode::AdjustedSpeciesRange, "AdjustedSpeciesRange");
                        ui.selectable_value(&mut trainer_state.configuration.speciation_offspring_mode, OffSpringMode::Average, "AverageFitness");
                    }
                );
                ui.end_row();
                add_slider_row(ui, "speciation_offspring_outcome_novelty_weight", &mut trainer_state.configuration.speciation_offspring_outcome_novelty_weight, 0.0..=1.0);

                ui.label("preserve_elite");
                ui.checkbox(&mut trainer_state.configuration.speciation_preserve_elite, "");
                ui.end_row();
           
            });
        });


        ui.collapsing("Mutation", |ui| {
            table(ui, |ui| {
                add_slider_row(ui, "connection_weight_max_value", &mut trainer_state.configuration.connection_weight_max_value, 0.0..=1.0);
                add_slider_row(ui, "mutation_connection_add_probability", &mut trainer_state.configuration.mutation_connection_add_probability, 0.0..=1.0);
                add_slider_row(ui, "mutation_connection_delete_probability", &mut trainer_state.configuration.mutation_connection_delete_probability, 0.0..=1.0);
                add_slider_row(ui, "mutation_connection_disable_probability", &mut trainer_state.configuration.mutation_connection_disable_probability, 0.0..=1.0);
                add_slider_row(ui, "mutation_connection_weight_change_probability", &mut trainer_state.configuration.mutation_connection_weight_change_probability, 0.0..=1.0);
                add_slider_row(ui, "mutation_connection_weight_change_scale", &mut trainer_state.configuration.mutation_connection_weight_change_scale, 0.0..=1.0);
                add_slider_row(ui, "mutation_connection_weight_replace_probability", &mut trainer_state.configuration.mutation_connection_weight_replace_probability, 0.0..=1.0);
                add_slider_row(ui, "mutation_node_add_probability", &mut trainer_state.configuration.mutation_node_add_probability, 0.0..=1.0);
                add_slider_row(ui, "mutation_node_bias_change_probability", &mut trainer_state.configuration.mutation_node_bias_change_probability, 0.0..=1.0);
                add_slider_row(ui, "mutation_node_bias_change_scale", &mut trainer_state.configuration.mutation_node_bias_change_scale, 0.0..=1.0);
                add_slider_row(ui, "mutation_node_delete_probability", &mut trainer_state.configuration.mutation_node_delete_probability, 0.0..=1.0);
                add_slider_row(ui, "mutation_node_cppn_input_multiplier_change_probability", &mut trainer_state.configuration.mutation_node_cppn_input_multiplier_change_probability, 0.0..=1.0);
                add_slider_row(ui, "mutation_node_cppn_input_multiplier_change_scale", &mut trainer_state.configuration.mutation_node_cppn_input_multiplier_change_scale, 0.0..=20.0);
                add_slider_row(ui, "mutation_node_cppn_input_multiplier_replace_probability", &mut trainer_state.configuration.mutation_node_cppn_input_multiplier_replace_probability, 0.0..=1.0);
                add_slider_row(ui, "node_bias_max_value", &mut trainer_state.configuration.node_bias_max_value, 0.0..=1.0);
                add_slider_row(ui, "node_bias_min_value", &mut trainer_state.configuration.node_bias_min_value, -1.0..=1.0);

                
                ui.collapsing("Activation", |ui| {
                    let mut activations: Vec<(String, bool, ActivationFunction)> = Vec::new();
                    for a in ActivationFunction::get_all() {
                        let is_on =trainer_state.configuration.mutation_node_available_activation_functions & a == a;
                        activations.push((format!("{:?}", a), is_on, a) )
                    }
                    table(ui, |ui| {
                        for (name, mut is_on, bit_value) in activations.iter_mut(){
                            let checkbox = ui.checkbox(&mut is_on, &*name);
                            if checkbox.clicked() {
                                if is_on {
                                    trainer_state.configuration.mutation_node_available_activation_functions = trainer_state.configuration.mutation_node_available_activation_functions | *bit_value;
                                }else{
                                    trainer_state.configuration.mutation_node_available_activation_functions = trainer_state.configuration.mutation_node_available_activation_functions & !(*bit_value);
                                }    
                            }
                            ui.end_row();
                        }
                    });
                });

            });
        });
        
        ui.collapsing("Regulators", |ui| {
            for (index, regulator ) in &mut trainer_state.config_regulators.iter_mut().enumerate(){
                ui.separator();
                ui.push_id(index, |ui| {
                    table(ui, |ui| {
                        ui.label(format!("Regulator {:?}", index));
                        ui.end_row();
    
                        combo_box_enum_row(ui, &mut regulator.signal_name, format!("{:?} signal", index).as_str());
                        add_selectable_value(ui, "max_value", &mut regulator.max_value_of_property);
                        add_selectable_value(ui, "min_value", &mut regulator.min_value_of_property);
                        combo_box_enum_row(ui, &mut regulator.property_to_change, format!("{:?} property", index).as_str());
                        add_selectable_value(ui, "target", &mut regulator.signal_target);
                        add_selectable_value(ui, "when_signal_below_change_factor", &mut regulator.when_signal_below_change_factor);
                        add_selectable_value(ui,  "when_signal_above_change_factor", &mut regulator.when_signal_above_change_factor);
                    });
                });
            }
        });

        ui.collapsing("Species table", |ui| {
            table(ui, |ui| {
                add_species_headers(ui);
                for species in &trainer_state.species_list.species_models{
                    add_species_row(ui, species);
                }
            });
        });

        ui.collapsing("Plots", |ui| {
            use egui::plot::{Line, Plot, PlotPoints};
            
            let list = trainer_state.last_ten_thousand_generations_stats.iter().rev().take(1000).rev().collect::<Vec<&GenerationStats>>();

            ui.label("Max objective_fitness (last 1k gens, positively adjusted from worst ever)");
            let best: PlotPoints =  list.iter().enumerate().map(|(i, val) |{[i as f64, val.max_objective_fitness as f64]}).collect();
            let line = Line::new(best);
            Plot::new("max_obj").boxed_zoom_pointer_button(egui::PointerButton::Primary).view_aspect(2.0).height(150.0).show(ui, |plot_ui| plot_ui.line(line));

            ui.label("Avg. objective_fitness (last 1k gens, positively adjusted from worst ever))");
            let best: PlotPoints =  list.iter().enumerate().map(|(i, val) |{[i as f64, val.avg_positive_objective_fitness as f64]}).collect();
            let line = Line::new(best);
            Plot::new("avg_obj").boxed_zoom_pointer_button(egui::PointerButton::Primary).view_aspect(2.0).height(150.0).show(ui, |plot_ui| plot_ui.line(line));

            ui.label("Max Nov (last 1k gens)");
            let best: PlotPoints =  list.iter().enumerate().map(|(i, val) |{[i as f64, val.max_outcome_novelty as f64]}).collect();
            let line = Line::new(best);
            Plot::new("max_nov").boxed_zoom_pointer_button(egui::PointerButton::Primary).view_aspect(2.0).height(150.0).show(ui, |plot_ui| plot_ui.line(line));

            ui.label("Avg. Nov (last 1k gens)");
            let best: PlotPoints =  list.iter().enumerate().map(|(i, val) |{[i as f64, val.avg_outcome_novelty as f64]}).collect();
            let line = Line::new(best);
            Plot::new("avg_nov").boxed_zoom_pointer_button(egui::PointerButton::Primary).view_aspect(2.0).height(150.0).show(ui, |plot_ui| plot_ui.line(line));
        });
    });
}


fn save_run(
    mut egui_ctx: ResMut<EguiContext>,
    mut trainer_state: ResMut<NeatTrainerState>,
    mut state: ResMut<NeatSettingsGuiState>,
) {
    egui::Window::new("Save")
    .vscroll(true)
    .open(&mut state.is_save_open)
    .show(egui_ctx.ctx_mut(), |ui| {

        ui.label(format!("{}{}-{}.neatrun", trainer_state.configuration.run_save_directory, trainer_state.configuration.run_name, trainer_state.current_generation));
        if ui.button("save").clicked(){
            trainer_state.generic_operations_queue.push(GenericOperation::Save());
        }
    });
}

fn load_run(
    mut egui_ctx: ResMut<EguiContext>,
    mut trainer_state: ResMut<NeatTrainerState>,
    mut state: ResMut<NeatSettingsGuiState>,
) {
    egui::Window::new("Load")
    .vscroll(true)
    .open(&mut state.is_load_open)
    .show(egui_ctx.ctx_mut(), |ui| {
        let directory = trainer_state.configuration.run_save_directory.clone();

        table(ui, |ui| {
            if let Ok(entries) = std::fs::read_dir(directory) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        let file_name = entry.file_name();
                        let file_name = file_name.to_str().unwrap();

                        if !file_name.starts_with(trainer_state.configuration.run_name.as_str()){
                            continue;
                        }

                        ui.label(file_name);
                        if let Ok(metadata) = entry.metadata(){
                            if let Ok(modified) = metadata.modified() {
                                let datetime: DateTime<Utc> = modified.into();
                                ui.label(format!("modified: {}", datetime.format("%d/%m/%Y %T")));
                            }
                        
                            ui.label(file_name);
                            if ui.button("Load").clicked() {
                                trainer_state.generic_operations_queue.push(GenericOperation::Load(entry.path().as_os_str().to_str().unwrap().to_owned()));
                            };
                        }
                        ui.end_row();
                    }
                }
            }
    
        });
    });
}



fn combo_box_enum_row<T>(ui: &mut Ui, enum_value: &mut T, label: &str) where T:Debug + IntoEnumIterator + std::cmp::PartialEq + Copy{
    ui.label(label);
    egui::ComboBox::from_label(label)
        .selected_text(format!("{:?}", enum_value))
        .show_ui(ui, |ui| {
            for value in T::iter(){
                ui.selectable_value(enum_value, value, format!("{:?}", value));
            }
        }
    );
    ui.end_row();
}

fn add_slider_row<Num: emath::Numeric>(ui: &mut Ui, label: impl Into<WidgetText>, value: &mut Num,  range: RangeInclusive<Num>){
    ui.label(label);
    ui.add(egui::Slider::new(value, range));
    ui.end_row();
}

fn add_selectable_value(ui: &mut Ui, label: impl Into<WidgetText>, value: &mut NeatFloat){
    ui.label(label);
    ui.add(
        egui::DragValue::new(value)
        .speed(0.1)
    );
    ui.end_row();
}

fn table<R>(ui: &mut Ui, contents: impl FnOnce(&mut Ui) -> R){
    egui::Grid::new("")
    .num_columns(2)
    .spacing([40.0, 4.0])
    .striped(true)
    .show(ui, contents);
}


pub fn add_species_headers(ui: &mut Ui){
    let font_size = 12.0 as f32;
    ui.label(RichText::new("Id").font(FontId::proportional(font_size)));
    ui.label(RichText::new("Members").font(FontId::proportional(font_size)));
    ui.label(RichText::new("Av. Fit").font(FontId::proportional(font_size)));
    ui.label(RichText::new("Av. Nov").font(FontId::proportional(font_size)));
    ui.label(RichText::new("Fit. Allowed Offspring").font(FontId::proportional(font_size)));
    ui.label(RichText::new("Nov. Allowed Offspring").font(FontId::proportional(font_size)));
    ui.end_row();
}

pub fn add_species_row(ui: &mut Ui, species: &crate::neat::trainer::neat_trainer_host::models::species::Species){
    let font_size = 10.0 as f32;
    let mut cell = |text: String, color: Color32| {
        ui.label(RichText::new(text).font(FontId::proportional(font_size)).color(color));
    };
    let r = (species.color.r() * 258.0) as u8;
    let g = (species.color.g() * 258.0) as u8;
    let b = (species.color.b() * 258.0) as u8;
    let color = Color32::from_rgb(r, g, b);
    cell(format!("{}", species.id), color);
    cell(format!("{}", species.no_of_members), color);
    cell(format!("{:.4}", species.objective_fitness.average), color);
    cell(format!("{:.8}", species.outcome_novelty.average), color);
    cell(format!("{:.4}", species.allowed_number_of_offspring_based_on_objective_fitness), color);
    cell(format!("{:.4}", species.allowed_number_of_offspring_based_on_outcome_novelty), color);
   
    ui.end_row();
}

