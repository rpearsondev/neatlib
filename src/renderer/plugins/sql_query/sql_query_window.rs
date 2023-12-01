use bevy::prelude::{ResMut, Plugin, App};
use bevy_egui::{*, egui::{Ui, RichText, FontId, Color32}};
use gluesql::prelude::Payload;

use crate::{common::event_stream::{listeners::{sql_listener::REPOSITORY, listeners::Listeners}, event::EventType, event_subscription::EventSubscription}, renderer::{renderer::NeatTrainerState, plugins::neat_settings_gui::NeatSettingsGuiState}};


pub struct SqlQueryPlugin{
}

pub struct SqlQueryResource{
    pub code: String,
    pub results: Vec<Payload>
}

impl Plugin for SqlQueryPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SqlQueryResource{
            code: "".to_string(),
            results: Vec::new(), 
        });
        app.add_system(sql_window);
    }

    fn name(&self) -> &str {
        std::any::type_name::<Self>()
    }
}

fn sql_window(
    mut egui_ctx: ResMut<EguiContext>,
    mut sql_resource: ResMut<SqlQueryResource>,
    mut trainer_state: ResMut<NeatTrainerState>,
    mut gui_state: ResMut<NeatSettingsGuiState>
) {
    egui::Window::new("Event Query")
    .vscroll(true)
    .open(&mut gui_state.show_events_query)
    .default_width(1200.0)
    .default_height(1200.0)
    .show(egui_ctx.ctx_mut(), |ui| {
        let theme = super::syntax_highlighting::CodeTheme::from_memory(ui.ctx());
        
        let mut layout = |ui: &egui::Ui, string: &str, wrap_width: f32| {
            let mut layout_job =
                super::syntax_highlighting::highlight(ui.ctx(), &theme, string, "sql");
            layout_job.wrap.max_width = wrap_width;
            ui.fonts().layout_job(layout_job)
        };
   
        ui.collapsing("Event Subscriptions", |ui| {
            let mut activations: Vec<(String, bool, EventType)> = Vec::new();
            
            let mut combined_event_type_subscription: EventType = EventType::empty();
            for event_type_subscription in trainer_state.configuration.event_subscriptions
            .iter()
            .filter(|s| s.listeners & Listeners::SQL == Listeners::SQL)
            .map(|s| s.event_type){
                combined_event_type_subscription = combined_event_type_subscription | event_type_subscription;
            }

            for a in EventType::get_all() {
                let is_on =combined_event_type_subscription & a == a;
                activations.push((format!("{:?}", a), is_on, a) )
            }

            table(ui, |ui| {
                for (name, mut is_on, bit_value) in activations.iter_mut(){
                    let checkbox = ui.checkbox(&mut is_on, &*name);
                    if checkbox.clicked() {
                        if is_on {
                            add_subscription(&mut trainer_state, bit_value.clone());
                        }else{
                            remove_subscription(&mut trainer_state, bit_value.clone());
                        }    
                    }
                    ui.end_row();
                }
            });
        });

        
        let event_types = EventType::get_all();
        ui.horizontal_wrapped(|ui|{
            for event_type in event_types{
                for event_subscription in &trainer_state.configuration.event_subscriptions{
                    if event_type & event_subscription.event_type == event_type{
                        if ui.button(format!("{:?}", event_type)).clicked() {
                            sql_resource.code = format!(
"select * from {:?} 
order by generation desc
limit 10", event_type)
                        }
                    }
                }
            }
        });

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.add(
                egui::TextEdit::multiline(&mut sql_resource.code)
                    .font(egui::TextStyle::Monospace) // for cursor height
                    .code_editor()
                    .desired_rows(3)
                    .lock_focus(true)
                    .desired_width(f32::INFINITY)
                    .layouter(&mut layout),
            );
        });    
        if ui.button("execute").clicked() {
            let mut repository = REPOSITORY.lock().unwrap();
            let results = repository.query(&sql_resource.code);

            match results{
                Ok(results) => {
                    sql_resource.results = results;
                },
                Err(_err) => {
                    sql_resource.results = Vec::new()
                },
            }
        }
           
        for species in sql_resource.results.iter(){
            match species {
                Payload::ShowColumns(_) => {},
                Payload::Create => {},
                Payload::Insert(_) => {},
                Payload::Select { labels, rows } => {
                    egui::Grid::new("some_unique_id").show(ui, |ui| {
                        add_species_headers(ui, labels);
                        for row in rows{
                            add_row(ui, row);
                        }
                    });
                },
                Payload::Delete(_) => {},
                Payload::Update(_) => {},
                Payload::DropTable => {},
                Payload::AlterTable => {},
                Payload::CreateIndex => {},
                Payload::DropIndex => {},
                Payload::StartTransaction => {},
                Payload::Commit => {},
                Payload::Rollback => {},
                Payload::ShowVariable(_) => {}
                Payload::SelectMap(_) => {},
                Payload::DropFunction => {},
                
            }
        }
        
    });
}


fn add_subscription(trainer_state: &mut ResMut<NeatTrainerState>, event_type: EventType){
    let sql_only_subscriptions = trainer_state.configuration.event_subscriptions.iter_mut().filter(|s| s.listeners == Listeners::SQL).last();
    
    if sql_only_subscriptions.is_some(){
        let sub = sql_only_subscriptions.unwrap();
        sub.event_type = sub.event_type | event_type;
    }else{
        trainer_state.configuration.event_subscriptions.push(EventSubscription{
            event_type,
            listeners: Listeners::SQL,
        });
    }
}

fn remove_subscription(trainer_state: &mut ResMut<NeatTrainerState>, event_type: EventType){
    for subscription in &mut trainer_state.configuration.event_subscriptions{
        if subscription.listeners & Listeners::SQL == Listeners::SQL{
            subscription.event_type = subscription.event_type ^ event_type;
        }
    }
}

fn table<R>(ui: &mut Ui, contents: impl FnOnce(&mut Ui) -> R){
    egui::Grid::new("")
    .num_columns(2)
    .spacing([40.0, 4.0])
    .striped(true)
    .show(ui, contents);
}

pub fn add_species_headers(ui: &mut Ui, labels: &Vec<String>){
    let font_size = 12.0 as f32;
    for label in labels{
        ui.label(RichText::new(label).font(FontId::proportional(font_size)));
    }
    ui.end_row();
}

pub fn add_row(ui: &mut Ui, row: &std::vec::Vec<gluesql::prelude::Value>){
    let font_size = 10.0 as f32;
    let mut cell = |text: String, color: Color32| {
        ui.label(RichText::new(text).font(FontId::proportional(font_size)).color(color));
    };

    let values = row;
    for value in values{
        cell(String::from(value), Color32::BLACK);
    }
   
    ui.end_row();
}

