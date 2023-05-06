use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use egui::{Align, Layout, ScrollArea};
use strum_macros::EnumIter;
use super::sysinfo::{HWSystem, HWKind};
use strum::{IntoEnumIterator};
use egui::plot::{Plot, PlotPoint, Line, PlotBounds};
use egui::Ui;
use queues::*;

// ---- Global Gui States/Resources ----

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum HWState {
    #[default]
    Splash,
    Details,
    Graphs,
    Settings,
}

#[derive(Resource)]
struct UpdateTimer(Timer);

#[derive(Resource)]
struct SplashTimer(Timer);

#[derive(Resource)]
struct HWResource {
    cpu_entities: Vec<Entity>,
    gpu_entities: Vec<Entity>,
    ram_entities: Vec<Entity>,
    //TODO
}

impl HWResource {
    fn new() -> Self {
        Self {
            cpu_entities: Vec::new(),
            gpu_entities: Vec::new(),
            ram_entities: Vec::new(),
        }
    }
}

pub struct HWGui {
    app: App,
}

impl HWGui {
    pub fn new() -> Self {
        Self {
            app: App::new()
        }
    }

    pub fn run(&mut self) {
        self.app
            .add_plugins(DefaultPlugins)
            .add_plugin(EguiPlugin)

            .add_state::<HWState>()
            .insert_resource(SplashTimer(Timer::from_seconds(2.0, TimerMode::Once)))
            .add_startup_system(startup)
            .insert_resource(HWResource::new())
            .insert_resource(HWSystem::new())
            .insert_resource(UpdateTimer(Timer::from_seconds(1.0, TimerMode::Repeating)))

            .add_system(splash_system.in_set(OnUpdate( HWState::Splash )))
            .add_systems(
                (console_system, 
                 refresh_system, 
                 details_system)
                .chain()
                .in_set(OnUpdate( HWState::Details ))
            )
            .add_systems(
                (console_system,
                 refresh_system,
                 graphs_system)
                .chain()
                .in_set(OnUpdate( HWState::Graphs ))
            )
            .add_systems(
                (console_system,
                 refresh_system,
                 settings_system)
                .chain()
                .in_set(OnUpdate( HWState::Settings ))
            )
            .run();
    }
}

// ---- Top Level Systems ----

fn startup(mut commands: Commands) {
}

fn splash_system(mut egui_ctxs: EguiContexts, mut next_state: ResMut<NextState<HWState>>, time: Res<Time>, mut timer: ResMut<SplashTimer>) {
    if timer.0.tick(time.delta()).just_finished()  {
        next_state.set(HWState::Details);
    }
    egui::CentralPanel::default().show(egui_ctxs.ctx_mut(), |ui| {
        ui.label("SPLASH");
    });
}

fn console_system(mut egui_ctxs: EguiContexts, mut next_state: ResMut<NextState<HWState>>) {
    let ctx = egui_ctxs.ctx_mut();

    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.with_layout(Layout::left_to_right(Align::Center),|ui| {
            if ui.selectable_label(false, "Details").clicked() {
                next_state.set(HWState::Details);
            }
            if ui.selectable_label(false, "Graphs").clicked() {
                next_state.set(HWState::Graphs);
            }
            if ui.selectable_label(false, "Settings").clicked() {
                next_state.set(HWState::Settings);
            }
        });

    });
}

fn refresh_system(mut commands: Commands, mut hwsystem: ResMut<HWSystem>, time: Res<Time>, mut timer: ResMut<UpdateTimer>, query: Query<&GraphData>) {
    if timer.0.tick(time.delta()).just_finished()  {

        hwsystem.refresh_all();
        for cpu in hwsystem.cpus() {
            let entity_id = commands.spawn_empty().id();
            commands.get_or_spawn(entity_id).
        }
        // for graphdata in &query {
        //     match graphdata.kind {
        //         GraphKind::CpuUsage(_) => ,
        //         GraphKind::GpuUsage => todo!(),
        //         GraphKind::MemoryUsed => todo!(),
        //     }
        // }
    }
}

// TODO: move each of the following systems into their own files

// ---- Details ----

#[derive(Component)]
struct DetailsData {

}

fn details_system(mut egui_ctxs: EguiContexts, mut hwsystem: ResMut<HWSystem>) {
    let ctx = egui_ctxs.ctx_mut();
    egui::CentralPanel::default().show(ctx, |_ui| {
        for kind in HWKind::iter() {
            setup_details(ctx, kind, &mut hwsystem);
        }
    });
}

fn setup_details(ctx: &egui::Context, kind: HWKind, hwsystem: &mut HWSystem) {

    // TODO: something like this ; also replace strings with generic structs
    // egui::Window::new(format!("{}", kind)).show(ctx, |ui| {
    //     for string in hwsystem.kind_strings(kind) {
    //         ui.label(string);
    //     }
    // });

    match kind {
        HWKind::CPU => {
            egui::Window::new(format!("{}", kind)).show(ctx, |ui| {
                for string in hwsystem.cpu_strings() {
                    ui.label(string);
                }
            });
        },
        HWKind::GPU => {
            egui::Window::new(format!("{}", kind)).show(ctx, |ui| {
                for string in hwsystem.gpu_strings() {
                    ui.label(string);
                }
            });
        },
        HWKind::RAM => {
            egui::Window::new(format!("{}", kind)).show(ctx, |ui| {
                for string in hwsystem.ram_and_swap_strings() {
                    ui.label(string);
                }
            });
        },
        HWKind::DISK => {
            egui::Window::new(format!("{}", kind)).show(ctx, |ui| {
                for string in hwsystem.disk_strings() {
                    ui.label(string);
                }
            });
        },
        HWKind::NETWORK => {
            egui::Window::new(format!("{}", kind)).show(ctx, |ui| {
                for string in hwsystem.network_strings() {
                    ui.label(string);
                }
            });
        },
        HWKind::SYSTEM => {
            egui::Window::new(format!("{}", kind)).show(ctx, |ui| {
                for string in hwsystem.system_strings() {
                    ui.label(string);
                }
            });
        },
        HWKind::COMPONENTS => {
            egui::Window::new(format!("{}", kind)).show(ctx, |ui| {
                for string in hwsystem.components_strings() {
                    ui.label(string);
                }
            });
        },
        HWKind::PROCESSES => {
            egui::Window::new(format!("{}", kind)).show(ctx, |ui| {
                for string in hwsystem.processes_strings() {
                    ui.label(string);
                }
            });
        },
        HWKind::MISC => {

        },
    }
}

// ---- Graphs ----

#[derive(EnumIter)]
enum GraphKind {
    CpuUsage(usize),
    GpuUsage,
    MemoryUsed,
    //TODO
}

#[derive(Component)]
struct GraphData {
    kind: GraphKind,
    data: queues::Queue<f32>,
    capacity: usize,
}

impl GraphData {
    fn new(kind: GraphKind, capacity: usize) -> Self {
        Self {
            kind,
            data: queues::queue![],
            capacity,
        }
    }

    fn add_data(&mut self, data: f32) {
        self.data.add(data);

        if self.data.size() > self.capacity {
            self.data.remove();
        }
    }
}

fn graphs_system(mut egui_ctxs: EguiContexts, mut hwsystem: ResMut<HWSystem>, mut query: Query<&mut GraphData>) {
    let ctx = egui_ctxs.ctx_mut();
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::new([false, true]).show(ui, |ui| {
            ui.vertical_centered(|ui| {
                for mut graphdata in &mut query {
                    setup_graphs(ctx, ui, &mut graphdata, &mut hwsystem);
                }
            });
        });
    });
}

fn setup_graphs(ctx: &egui::Context, ui: &mut Ui, graphdata: &mut GraphData, hwsystem: &mut HWSystem) {

}

// let plot = Plot::new("Processes")
// .allow_double_click_reset(false)
// .allow_drag(false)
// .allow_scroll(false)
// .allow_zoom(false)
// .center_x_axis(false)
// .center_y_axis(false)
// .show_x(false)
// .show_y(false);

// plot.show(ui, |ui| {

// });

// ---- Settings ----

#[derive(Component)]
struct SettingsData {

}

fn settings_system(mut egui_ctxs: EguiContexts, mut hwsystem: ResMut<HWSystem>) {
    println!("SETTINGS!");
}