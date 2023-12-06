use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use egui::{Stroke, Color32};
use petgraph::graph::{NodeIndex, UnGraph};


#[wasm_bindgen]
extern "C" {
    type Performance;
    static performance: Performance;
    #[wasm_bindgen(method)]
    fn now(this: &Performance) -> f64;
}

fn get_current_time() -> f64 {
    performance.now()
}

struct Circle {
    position: egui::Pos2,
    radius: f32,

}
type CircleGraph = UnGraph<Circle, ()>;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    label: String,
    editor_text: String,
    show_popup: bool,


    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
    #[serde(skip)]
    frame_counter: u32,
    #[serde(skip)]
    graph: CircleGraph,
    #[serde(skip)]
    central_node_index: NodeIndex,
    #[serde(skip)]
    orbit_radius: f32,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "put a label here?".to_owned(),
            value: 2.9,
            frame_counter: 0,
            editor_text: String::new(),
            show_popup: false,
            

        }
    }
}

// add this if you ever need UI customization, State Persistence, or Unique initialization logic
impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui
        
        let current_time = get_current_time() / 1000.0;
        self.frame_counter += 1;


        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
                {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            _frame.close();
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("I just changed this text");

            //ui.add(egui::TextEdit::singleline(&mut self.editor_text));
            
                        // Button to open the pop-up window
            if ui.button("Open Pop-up").clicked() {
                self.show_popup = true;
            }

            // Pop-up window
            if self.show_popup {
                egui::Window::new("Text Input Pop-up")
                    .open(&mut self.show_popup) // Bind the window's open state to show_popup
                    .show(ctx, |ui| {
                        ui.text_edit_multiline(&mut self.editor_text);
                        
                        
                        // Add more UI elements as needed
                    });
            }
            // Add an invisible label with a constantly changing text (like a frame counter)
            // This can trick eframe into redrawing each frame
            ui.label(format!("Frame: {}", self.frame_counter));
            ui.allocate_space(egui::vec2(0.0, 0.0));

            let painter = ui.painter();

            let panel_size = ui.available_size();
            let base_circle_center = egui::pos2(panel_size.x / 2.0, panel_size.y / 2.0);
            //let mut circle_center = egui::pos2(panel_size.x / 2.0, panel_size.y / 2.0);

            let node_id1: f32 = 1.0;
            let node_id2: f32 = 2.0;

            // Calculate the position offsets using sine and cosine with current time
            let offset_x1 = (current_time as f32 + node_id1).sin() * 5.0; // Adjust multiplier for range
            let offset_y1 = (current_time as f32 + node_id1).cos() * 5.0;
            let offset_x2 = (current_time as f32 + node_id2).sin() * 5.0;
            let offset_y2 = (current_time as f32 + node_id2).cos() * 5.0;

            // Apply offsets to the base position
            let circle_center = egui::pos2(base_circle_center.x + offset_x1, base_circle_center.y + offset_y1);
            let circle2_center = egui::pos2(base_circle_center.x - 350.0 + offset_x2, base_circle_center.y + offset_y2);


            let circle_radius = 50.0;
            let circle_color = egui::Color32::WHITE;
            let stroke_width = 2.0;
            let stroke = egui::Stroke::new(stroke_width, circle_color);
            //let mut circle2_center = egui::pos2(circle_center.x + 250.0, circle_center.y);

            //let circle2_color = egui::Color32::RED;

            painter.circle(circle_center, circle_radius, egui::Color32::TRANSPARENT, stroke);
            painter.circle(circle2_center, circle_radius, egui::Color32::TRANSPARENT, stroke);



            // getting the lines to only connect to outer circle edge
            let direction = circle2_center - circle_center;
            let norm_direction = direction.normalized();
            let start_point = circle_center + norm_direction * circle_radius;
            let end_point = circle2_center - norm_direction * circle_radius;






            let line_color = egui::Color32::WHITE;
            let line_width = 2.0;
            painter.line_segment([start_point, end_point], (line_width, line_color));

            ctx.request_repaint();
        });
    }
}




impl TemplateApp {
    fn add_circle(&mut self) {
        // Determine the position for the new circle
        // ...

        let new_node = Circle { position: new_position, radius: 20.0 };
        let new_node_index = self.graph.add_node(new_node);

        // Example: Connect the new node with the central node
        self.graph.add_edge(self.central_node_index, new_node_index, ());
    }
}



impl MyApp {
    fn add_circle(&mut self) {
        let mut best_position = None;
        let mut max_distance = 0.0;

        // Check points around the orbit
        for angle_degree in (0..360).step_by(5) {  // Adjust step for finer checks
            let angle_rad = angle_degree as f32 * std::f32::consts::PI / 180.0;
            let new_x = self.central_circle.position.x + self.orbit_radius * angle_rad.cos();
            let new_y = self.central_circle.position.y + self.orbit_radius * angle_rad.sin();
            let new_position = egui::pos2(new_x, new_y);

            // Calculate minimum distance to existing circles
            let mut min_distance_to_existing = f32::MAX;
            for circle in &self.surrounding_circles {
                let distance = new_position.distance(circle.position) - circle.radius;
                min_distance_to_existing = min_distance_to_existing.min(distance);
            }

            // If this position is better, update best_position
            if min_distance_to_existing > max_distance {
                max_distance = min_distance_to_existing;
                best_position = Some(new_position);
            }
        }

        // Add the new circle at the best found position
        if let Some(position) = best_position {
            self.surrounding_circles.push(Circle {
                position,
                radius: 20.0, // Example radius for surrounding circles
            });
        }
    }
}
