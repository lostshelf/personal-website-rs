mod palette;

use eframe::egui;

#[derive(PartialEq, Clone, Copy)]
enum Page {
    Home,
    About,
    Projects,
    Contact,
    NotFound
}

pub struct App {
    cli_input: String,
    current_page: Page,
    error_msg: Option<String>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            cli_input: String::new(),
            current_page: Page::Home,
            error_msg: None,
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
        let cmd = self.cli_input.trim().to_lowercase();

        self.current_page = match cmd.as_str() {
            "home" | "clear" => Page::Home,
            "about" => Page::About,
            "projects" => Page::Projects,
            "contact" => Page::Contact,
            "help" => { self.error_msg = Some("Available commands are: about, projects, contact and clear".into()); self.current_page }
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
}

impl eframe::App for App {
    /// Called each time the UI needs repainting, which may be many times per second.
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
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
                }
            });
    }
}