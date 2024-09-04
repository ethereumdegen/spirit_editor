 
use spirit_edit_core::zones::create_zone;
use spirit_edit_core::zones::load_zone;
use spirit_edit_core::zones::load_all_zones;
use crate::zones::egui::RichText;
use spirit_edit_core::zones::NotInScene;
use bevy_editor_pls_core::editor_window::EditorWindow;
use bevy_editor_pls_core::editor_window::EditorWindowContext;
use bevy_inspector_egui::egui;
//use spirit_edit_core::zones::ZoneResource;
use spirit_edit_core::zones::ZoneEvent;
use std::fs;
use bevy::prelude::*;
 

use std::path::Path;


 

const DEFAULT_FILENAME: &str = "zone01";
 
 
#[derive(Default)]
pub struct ZoneWindowState {
    create_filename: String,
    load_filename: String,
    zone_create_result: Option<Result<(), Box<dyn std::error::Error + Send + Sync>>>,
}

pub struct ZoneWindow;

impl EditorWindow for ZoneWindow {
    type State = ZoneWindowState;
    const NAME: &'static str = "Zones";

    fn ui(world: &mut World, mut cx: EditorWindowContext, ui: &mut egui::Ui) {
        let state = cx.state_mut::<ZoneWindow>().unwrap();

       

      ui.horizontal(|ui| {

        ui.vertical(|ui| {
            

            //create zone
            ui.horizontal(|ui| {
                let res = egui::TextEdit::singleline(&mut state.create_filename)
                    .hint_text(DEFAULT_FILENAME)
                    .desired_width(120.0)
                    .show(ui);

                if res.response.changed() {
                    state.zone_create_result = None;
                }

               // let enter_pressed = ui.input(|input| input.key_pressed(egui::Key::Enter));

                if ui.button("Create Zone").clicked()   {
                    let create_filename = if state.create_filename.is_empty() {
                        DEFAULT_FILENAME
                    } else {
                        &state.create_filename
                    };
                    let mut query = world.query_filtered::<Entity, Without<NotInScene>>();
                    // let entitys = query.iter(world).collect();
                    state.zone_create_result = Some(create_zone(world, create_filename));
                }
            });

            ui.horizontal(|ui| {
                let res = egui::TextEdit::singleline(&mut state.load_filename)
                    .hint_text(DEFAULT_FILENAME)
                    .desired_width(120.0)
                    .show(ui);

                if res.response.changed() {
                    state.zone_create_result = None;
                }

                let enter_pressed = ui.input(|input| input.key_pressed(egui::Key::Enter));

                if ui.button("Load Zone").clicked()   {
                    let load_filename = if state.load_filename.is_empty() {
                        DEFAULT_FILENAME
                    } else {
                        &state.load_filename
                    };
                    let mut query = world.query_filtered::<Entity, Without<NotInScene>>();
                    // let entitys = query.iter(world).collect();
                    state.zone_create_result = Some(load_zone(world, load_filename));
                }
            })
            // ----- h
        }); // ---- v

           ui.vertical(|ui| {

              ui.label(format!("--- " ) );

                if ui.button("Load All Zones").clicked()   {
                   
                    // let entitys = query.iter(world).collect();
                    state.zone_create_result = Some(load_all_zones(world));
                }


             }); // ---- v

      }); // ---- H


        if let Some(status) = &state.zone_create_result {
            match status {
                Ok(()) => {
                    ui.label(RichText::new("Success!").color(egui::Color32::GREEN));
                }
                Err(error) => {
                    ui.label(RichText::new(error.to_string()).color(egui::Color32::RED));
                }
            }
        }
    }
}
 
 