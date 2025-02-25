use egui::{CtxRef, Resize, ScrollArea, Ui, Window};

// ----------------------------------------------------------------------------

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
struct Demos {
    /// open, view
    #[serde(skip)] // TODO: serialize the `open` state.
    demos: Vec<(bool, Box<dyn super::Demo>)>,
}
impl Default for Demos {
    fn default() -> Self {
        Self {
            demos: vec![
                (false, Box::new(super::FontBook::default())),
                (false, Box::new(super::Painting::default())),
                (false, Box::new(super::DancingStrings::default())),
                (false, Box::new(super::DragAndDropDemo::default())),
                (false, Box::new(super::Tests::default())),
                (false, Box::new(super::WindowOptions::default())),
            ],
        }
    }
}
impl Demos {
    pub fn checkboxes(&mut self, ui: &mut Ui) {
        for (ref mut open, demo) in &mut self.demos {
            ui.checkbox(open, demo.name());
        }
    }

    pub fn show(&mut self, ctx: &CtxRef) {
        for (ref mut open, demo) in &mut self.demos {
            demo.show(ctx, open);
        }
    }
}

// ----------------------------------------------------------------------------

/// A menu bar in which you can select different demo windows to show.
#[derive(Default, serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct DemoWindows {
    open_windows: OpenWindows,

    demo_window: super::DemoWindow,

    /// open, title, view
    demos: Demos,
}

impl DemoWindows {
    /// Show the app ui (menu bar and windows).
    /// `sidebar_ui` can be used to optionally show some things in the sidebar
    pub fn ui(&mut self, ctx: &CtxRef) {
        egui::SidePanel::left("side_panel", 200.0).show(ctx, |ui| {
            ui.heading("✒ Egui Demo");

            ui.separator();

            ScrollArea::auto_sized().show(ui, |ui| {
                ui.label("Egui is an immediate mode GUI library written in Rust.");
                ui.add(
                    egui::Hyperlink::new("https://github.com/emilk/egui").text(" Egui home page"),
                );

                ui.label("Egui can be run on the web, or natively on 🐧");

                ui.separator();

                ui.heading("Windows:");
                ui.indent("windows", |ui| {
                    self.open_windows.checkboxes(ui);
                    self.demos.checkboxes(ui);
                });

                ui.separator();

                if ui.button("Organize windows").clicked {
                    ui.ctx().memory().reset_areas();
                }
            });
        });

        egui::TopPanel::top("menu_bar").show(ctx, |ui| {
            show_menu_bar(ui);
        });

        self.windows(ctx);
    }

    /// Show the open windows.
    fn windows(&mut self, ctx: &CtxRef) {
        let Self {
            open_windows,
            demo_window,
            demos,
            ..
        } = self;

        Window::new("✨ Demo")
            .open(&mut open_windows.demo)
            .scroll(true)
            .show(ctx, |ui| {
                demo_window.ui(ui);
            });

        Window::new("🔧 Settings")
            .open(&mut open_windows.settings)
            .scroll(true)
            .show(ctx, |ui| {
                ctx.settings_ui(ui);
            });

        Window::new("🔍 Inspection")
            .open(&mut open_windows.inspection)
            .scroll(true)
            .show(ctx, |ui| {
                ctx.inspection_ui(ui);
            });

        Window::new("📝 Memory")
            .open(&mut open_windows.memory)
            .resizable(false)
            .show(ctx, |ui| {
                ctx.memory_ui(ui);
            });

        demos.show(ctx);

        self.resize_windows(ctx);
    }

    fn resize_windows(&mut self, ctx: &CtxRef) {
        let open = &mut self.open_windows.resize;

        Window::new("resizable")
            .open(open)
            .scroll(false)
            .resizable(true)
            .show(ctx, |ui| {
                ui.label("scroll:    NO");
                ui.label("resizable: YES");
                ui.label(crate::LOREM_IPSUM);
            });

        Window::new("resizable + embedded scroll")
            .open(open)
            .scroll(false)
            .resizable(true)
            .default_height(300.0)
            .show(ctx, |ui| {
                ui.label("scroll:    NO");
                ui.label("resizable: YES");
                ui.heading("We have a sub-region with scroll bar:");
                ScrollArea::auto_sized().show(ui, |ui| {
                    ui.label(crate::LOREM_IPSUM_LONG);
                    ui.label(crate::LOREM_IPSUM_LONG);
                });
                // ui.heading("Some additional text here, that should also be visible"); // this works, but messes with the resizing a bit
            });

        Window::new("resizable + scroll")
            .open(open)
            .scroll(true)
            .resizable(true)
            .default_height(300.0)
            .show(ctx, |ui| {
                ui.label("scroll:    YES");
                ui.label("resizable: YES");
                ui.label(crate::LOREM_IPSUM_LONG);
            });

        Window::new("auto_sized")
            .open(open)
            .auto_sized()
            .show(ctx, |ui| {
                ui.label("This window will auto-size based on its contents.");
                ui.heading("Resize this area:");
                Resize::default().show(ui, |ui| {
                    ui.label(crate::LOREM_IPSUM);
                });
                ui.heading("Resize the above area!");
            });
    }
}

// ----------------------------------------------------------------------------

#[derive(serde::Deserialize, serde::Serialize)]
struct OpenWindows {
    demo: bool,

    // egui stuff:
    settings: bool,
    inspection: bool,
    memory: bool,
    resize: bool,
}

impl Default for OpenWindows {
    fn default() -> Self {
        Self {
            demo: true,
            ..OpenWindows::none()
        }
    }
}

impl OpenWindows {
    fn none() -> Self {
        Self {
            demo: false,

            settings: false,
            inspection: false,
            memory: false,
            resize: false,
        }
    }

    fn checkboxes(&mut self, ui: &mut Ui) {
        let Self {
            demo,
            settings,
            inspection,
            memory,
            resize,
        } = self;
        ui.label("Egui:");
        ui.checkbox(settings, "🔧 Settings");
        ui.checkbox(inspection, "🔍 Inspection");
        ui.checkbox(memory, "📝 Memory");
        ui.separator();
        ui.checkbox(demo, "✨ Demo");
        ui.separator();
        ui.checkbox(resize, "↔ Resize examples");
        ui.separator();
        ui.label("Misc:");
    }
}

fn show_menu_bar(ui: &mut Ui) {
    use egui::*;

    menu::bar(ui, |ui| {
        menu::menu(ui, "File", |ui| {
            if ui.button("Organize windows").clicked {
                ui.ctx().memory().reset_areas();
            }
            if ui
                .button("Clear Egui memory")
                .on_hover_text("Forget scroll, collapsing headers etc")
                .clicked
            {
                *ui.ctx().memory() = Default::default();
            }
        });
    });
}
