use egui::Pos2;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use egui::{Stroke, Color32};
use petgraph::graph::UnGraph;
use petgraph::graph::NodeIndex;
use petgraph::visit::EdgeRef;

// using the following to get current time using js
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
    title: String,
}
type CircleGraph = UnGraph<Circle, ()>;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MindGraph {
    frame_counter: u32,
    editor_text: String,
    show_popup: bool,
    central_circle_initialized: bool,
    

    #[serde(skip)]
    graph: CircleGraph,
    #[serde(skip)]
    current_central_node_index: NodeIndex,
    #[serde(skip)]
    original_central_node_index: NodeIndex,
    #[serde(skip)]
    orbit_radius: f32,

}


impl Default for MindGraph {
    fn default() -> Self {
        let mut graph = CircleGraph::default();
        let central_circle = Circle {
            position: egui::pos2(200.0, 200.0),
            radius: 40.0,
            title: "".to_string(),
            //color: egui::Color32::WHITE,
            //stroke_width: 2.0,
            //stroke: egui::Stroke::new(stroke_width, color),
        };
        let central_node_index = graph.add_node(central_circle);
        Self{
            frame_counter:0,
            editor_text:String::new(),
            show_popup:false,
            graph,
            current_central_node_index: central_node_index,// this is the same as the original on initialization, but will change later
            original_central_node_index: central_node_index,
            orbit_radius: 300.0,
            central_circle_initialized: false,

        }
    }
}


impl MindGraph {
    pub fn add_node(&mut self) {

        // create a Circle 
        let new_node = Circle {
            position: egui::pos2(0.0, 0.0),
            radius: 40.0,
            title: "".to_string(),
        };

        // add the new node to the graph
        let new_node_index = self.graph.add_node(new_node);

        // create edge to central node
        self.graph.add_edge(self.current_central_node_index, new_node_index, ());
        
        // recalculate positions for all nodes
        self.recalculate_node_positions();
    }

    fn recalculate_node_positions(&mut self) {
        // collect the indices of the neighboring nodes
        let neighbors: Vec<NodeIndex> = self.graph.neighbors(self.current_central_node_index).collect();

        let neighbor_count = neighbors.len();
        let angle_increment = 360.0 / neighbor_count as f32;
        // new circles automatically added to the right, this allows you to change that
        let start_angle = 0.0;

        //  iterate over the indices
        for (i, node_index) in neighbors.into_iter().enumerate() {
            let angle_degree = start_angle + angle_increment * i as f32;
            let angle_rad = angle_degree.to_radians();

            let central_circle = &self.graph[self.current_central_node_index];
            let new_x = central_circle.position.x + self.orbit_radius * angle_rad.cos();
            let new_y = central_circle.position.y + self.orbit_radius * angle_rad.sin();

            self.graph[node_index].position = egui::pos2(new_x, new_y);
        }
    }


    pub fn set_new_central_node(&mut self, new_central_node_index: NodeIndex) {
        //self.recalculate_node_positions();

        self.current_central_node_index = new_central_node_index;
        self.recalculate_node_positions();
    }

    pub fn draw_graph(&mut self, ui: &mut egui::Ui, current_time: f64) {
        let painter = ui.painter();
        let circle_color = egui::Color32::WHITE;
        let stroke_width = 2.0;
        let stroke = egui::Stroke::new(stroke_width, circle_color);

        let current_central_circle = &self.graph[self.current_central_node_index];
        painter.circle(current_central_circle.position, current_central_circle.radius, egui::Color32::TRANSPARENT, stroke);


        // get the position of the current central node to use in the for loop
        let mut central_node_position = egui::pos2(0.0, 0.0);
        let mut central_node_title = "".to_string();

        let neighbor_indices: Vec<_> = self.graph.neighbors(self.current_central_node_index).collect();

        // iterate over nodes to draw circles and lines
        for node_index in neighbor_indices {

            let neighbor_node = &self.graph[node_index];

            painter.circle(neighbor_node.position, neighbor_node.radius, egui::Color32::TRANSPARENT, stroke);
            //self.draw_text_boxes(ui);
            

            if let Some(temp_central_node_data) = self.graph.node_weight(self.current_central_node_index) {

                central_node_position = temp_central_node_data.position;
                central_node_title = temp_central_node_data.title.clone();
            }            

                // Define the area for the text input field
            let text_field_rect = egui::Rect::from_min_size(
                egui::pos2(
                    neighbor_node.position.x - neighbor_node.radius,
                    neighbor_node.position.y - neighbor_node.radius - 20.0,
                ),
                egui::vec2(100.0, 20.0),
            );

            // ui.allocate_ui_at_rect(text_field_rect, |ui| {
            //     ui.text_edit_singleline(&mut neighbor_node.title);
            // });





            // draw connections to other nodes
            for edge in self.graph.edges(self.current_central_node_index) {


                let target_node = &self.graph[edge.target()];

                let start_point = central_node_position; 
                let end_point = target_node.position;

                //self.recalculate_node_positions();

                painter.line_segment(
                    [start_point, end_point],
                    (2.0, egui::Color32::WHITE)
                );
            }
        }
        self.recalculate_node_positions();
        
    }
    pub fn draw_text_boxes(&mut self, ui: &mut egui::Ui) {

        let central_node = &mut self.graph[self.current_central_node_index];
    
        // Define the area for the text input field
        let text_field_rect = egui::Rect::from_min_size(
            egui::pos2(
                central_node.position.x - central_node.radius,
                central_node.position.y - central_node.radius - 20.0,
            ),
            egui::vec2(100.0, 20.0),
        );

        ui.allocate_ui_at_rect(text_field_rect, |ui| {
            ui.text_edit_singleline(&mut central_node.title);
        });
        
        // Collect indices first to avoid borrowing issues
        let neighbor_indices: Vec<_> = self.graph.neighbors(self.current_central_node_index).collect();
    
        for node_index in neighbor_indices {
            // Now, borrow the node mutably
            let neighbor_node = &mut self.graph[node_index];
    
            // Define the area for the text input field
            let text_field_rect = egui::Rect::from_min_size(
                egui::pos2(
                    neighbor_node.position.x - neighbor_node.radius,
                    neighbor_node.position.y - neighbor_node.radius - 20.0,
                ),
                egui::vec2(100.0, 20.0),
            );
    
            ui.allocate_ui_at_rect(text_field_rect, |ui| {
                ui.text_edit_singleline(&mut neighbor_node.title);
            });
        }
    }
    
}
            // let text = "hello there";
            // let text_color = egui::Color32::WHITE;
            // let font_size = 20.0;
            // let font_id = egui::FontId::new(font_size, egui::FontFamily::Proportional); 

            
            // let text_position = egui::pos2(
            //     neighbor_node.position.x - text_size.x / 20.0,
            //     neighbor_node.position.y - neighbor_node.radius - text_size.y
            // );

            // ui.label(format!("Frame: {}", self.frame_counter));
            // ui.allocate_space(egui::vec2(0.0, 0.0));

impl MindGraph {
    pub fn go_home_logic(&mut self) {
        //TODO
    }

    fn go_home_ui() {
        //TODO
    }
}

impl eframe::App for MindGraph {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let current_time = get_current_time() / 1000.0;
        self.frame_counter += 1;


        egui::CentralPanel::default().show(ctx, |ui| {
            // frame counter for debugging purposes
            ui.label(format!("Frame: {}", self.frame_counter));
            ui.allocate_space(egui::vec2(0.0, 0.0));

            // make sure central circle is in center
            let panel_size = ui.available_size();
            let central_position = egui::pos2(panel_size.x / 2.0, panel_size.y / 2.0);
            self.graph[self.current_central_node_index].position = central_position;
            // might need the following if we ever change radius / orbit 
            // self.recalculate_node_positions();
    
            // button to add a new circle
            if ui.button("Add Circle").clicked() {
                self.add_node(); 
                self.draw_graph(ui, current_time); 
                self.draw_text_boxes(ui);
            } else {
                // regular drawing of the graph
                self.draw_graph(ui, current_time);
                self.draw_text_boxes(ui);
            }




            // check for mouse click
            ctx.input(|input| {
                if input.pointer.any_pressed() {
                    if let Some(pointer_pos) = input.pointer.interact_pos() {
                        for node_index in self.graph.neighbors(self.current_central_node_index) {
                            let node = &self.graph[node_index];
                            let distance = node.position.distance(pointer_pos);
                            if distance < node.radius {
                                self.set_new_central_node(node_index);
                                break;
                            }
                        }
                    }
                }
            });




            
            // probably performance hog, but needed to keep circles floating
            ctx.request_repaint();
        });
    }
}













// add this if you ever need UI customization, State Persistence, or Unique initialization logic
impl MindGraph {
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