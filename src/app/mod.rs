mod palette;

use eframe::egui;

const PASSWORD: &str = "password{Y0U_AR3_W0RTHY}";

#[derive(PartialEq, Clone, Copy)]
enum Page {
    Home,
    About,
    Projects,
    Contact,
    NotFound,
    Help
}

pub struct App {
    cli_input: String,
    current_page: Page,
    error_msg: Option<String>,
    unlocked: bool,
    password_input: String,
}

impl Default for App {
    fn default() -> Self {
        Self {
            cli_input: String::new(),
            current_page: Page::Home,
            error_msg: None,
            unlocked: false,
            password_input: String::new(),
        }
    }
}

fn setup_theme(ctx: &egui::Context) {
    setup_visuals(ctx);
    setup_fonts(ctx);
}

fn setup_visuals(ctx: &egui::Context) {
    use palette::*;
    
    let mut visuals = egui::Visuals::dark();

    visuals.panel_fill = BG_BASE;
    visuals.window_fill = BG_BASE;
    visuals.extreme_bg_color = BG_DEEP;

    visuals.selection.bg_fill = ACCENT;
    visuals.selection.stroke.color = ACCENT_BRIGHT;
    visuals.hyperlink_color = ACCENT_BRIGHT;

    visuals.widgets.noninteractive.fg_stroke.color = TEXT_SECONDARY;
    visuals.widgets.inactive.bg_stroke.color = ACCENT_DIM;
    visuals.widgets.hovered.bg_stroke.color = ACCENT_BRIGHT;
    visuals.widgets.active.bg_stroke.color = ACCENT;

    ctx.set_visuals(visuals);
}

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    fonts.font_data.insert(
        "geo".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../../assets/fonts/Geo-Regular.ttf"
        )))
    );

    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .insert(0, "geo".to_owned());

    ctx.set_fonts(fonts);
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_theme(&cc.egui_ctx);
        setup_fonts(&cc.egui_ctx);

        Default::default()
    }

    fn run_command(&mut self) {
        if !self.unlocked {
            return;
        }

        let cmd = self.cli_input.trim().to_lowercase();

        self.current_page = match cmd.as_str() {
            "home" | "clear" => Page::Home,
            "about" => Page::About,
            "projects" => Page::Projects,
            "contact" => Page::Contact,
            "help" => Page::Help,
            _ => Page::NotFound,
        };

        log::info!("Running command: {}", cmd);

        self.cli_input.clear();
    }

    fn render_home(&self, ui: &mut egui::Ui) {
        ui.label("home page");
    }

    fn render_about(&self, ui: &mut egui::Ui) {
        ui.label("about page");
    }

    fn render_contact(&self, ui: &mut egui::Ui) {
        ui.label("contact page");
    }

    fn render_projects(&self, ui: &mut egui::Ui) {
        ui.label("projects page");
    }

    fn render_not_found(&self, ui: &mut egui::Ui) {
        ui.label("not found page");
    }

    fn render_help(&self, ui: &mut egui::Ui) {
        ui.label("help page");
    }

    fn check_password(&mut self) {
        let input = self.password_input.trim();

        if input == PASSWORD {
            self.unlocked = true;
        } else {
            
        }

        self.password_input.clear();
    }
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        if self.unlocked {
            egui::Panel::top("input_bar")
                .frame(egui::Frame::default()
                    .fill(palette::BG_BASE)
                    .inner_margin(egui::Margin::symmetric(12, 10)))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.label(egui::RichText::new(">").color(palette::ACCENT).monospace().size(20.0));

                        let response = ui.add_sized(
                            ui.available_size(),
                            egui::TextEdit::singleline(&mut self.cli_input)
                                .frame(egui::Frame::default().stroke(egui::Stroke { color: palette::ACCENT_DIM, ..Default::default()}))
                                .font(egui::FontId::monospace(20.0))
                                .vertical_align(egui::Align::Center)
                                .text_color(palette::TEXT_PRIMARY)
                                .desired_width(f32::INFINITY)
                                .hint_text(
                                    egui::RichText::new("type 'help' to see all available commands")
                                        .color(palette::TEXT_SECONDARY)
                                        .size(20.0)
                                        .monospace()
                                )
                        );

                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            self.run_command();
                            response.request_focus();
                        }

                        if ui.ctx().memory(|m| m.focused().is_none()) {
                            response.request_focus();
                        }
                    })
                });

            egui::CentralPanel::default()
                .frame(egui::Frame::default().fill(palette::BG_BASE).inner_margin(20))
                .show(ui, |ui| {
                    match self.current_page {
                        Page::Home => self.render_home(ui),
                        Page::About => self.render_about(ui),
                        Page::Projects => self.render_projects(ui),
                        Page::Contact => self.render_contact(ui),
                        Page::NotFound => self.render_not_found(ui),
                        Page::Help => self.render_help(ui),
                    }
                });
        } else {
            egui::CentralPanel::default()
                .frame(egui::Frame::default().fill(palette::BG_BASE).inner_margin(20))
                .show(ui, |ui| {
                    egui::Window::new("login_screen")
                        .title_bar(false)
                        .resizable(false)
                        .collapsible(false)
                        .frame(egui::Frame::default().fill(palette::BG_BASE).inner_margin(24.0))
                        .fixed_size(egui::vec2(800.0, 160.0))
                        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
                        .collapsible(false)
                        .title_bar(false)
                        .show(ui, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.add_space(12.0);
                                ui.label(egui::RichText::new("PROVE YOUR WORTH").monospace().color(palette::TEXT_SECONDARY).size(80.0));
                                ui.add_space(16.0);
                                ui.horizontal(|ui| {
                                    let response = ui.add(
                                        egui::TextEdit::singleline(&mut self.password_input)
                                            .frame(egui::Frame::default().stroke(egui::Stroke { width: 1.0, color: palette::ACCENT_DIM, ..Default::default()}).inner_margin(10.0))
                                            .font(egui::FontId::monospace(20.0))
                                            .vertical_align(egui::Align::Center)
                                            .text_color(palette::TEXT_PRIMARY)
                                            .desired_width(f32::INFINITY)
                                            .hint_text(
                                                egui::RichText::new("Enter password here")
                                                    .color(palette::TEXT_SECONDARY)
                                                    .size(20.0)
                                                    .monospace()
                                            )
                                    );
                                    

                                    if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                        self.check_password();
                                    }
                                });
                            });
                        });
            });
        }
    }
}