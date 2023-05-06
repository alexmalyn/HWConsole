mod gui;
mod sysinfo;

fn main() {
    let mut app = gui::HWGui::new();
    app.run();
}