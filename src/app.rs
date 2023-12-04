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
    //color: egui::Color32,
    //stroke_width: f32,
    //stroke: egui::Stroke,


}
type CircleGraph = UnGraph<Circle, ()>;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MindGraph {
    frame_counter: u32,
    editor_text: String,
    show_popup: bool,
    

    #[serde(skip)]
    graph: CircleGraph,
    #[serde(skip)]
    central_node_index: NodeIndex,
    #[serde(skip)]
    orbit_radius: f32,

}


impl Default for MindGraph {
    fn default() -> Self {
        let mut graph = CircleGraph::default();
        let central_circle = Circle {
            position: egui::pos2(200.0, 200.0),
            radius: 20.0,
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
            central_node_index,
            orbit_radius: 100.0,

        }
    }
}


impl MindGraph {
    pub fn add_node(&mut self) {
        // determine the position for new node
        //let new_node_position = self.recalculate_node_positions();

        // create a Circle 
        let new_node = Circle {
            position: egui::pos2(0.0, 0.0),
            radius: 20.0,
        };

        // add the new node to the graph
        let new_node_index = self.graph.add_node(new_node);

        // create edge to central node
        self.graph.add_edge(self.central_node_index, new_node_index, ());
        
        // Recalculate positions for all nodes
        self.recalculate_node_positions();
    }

    fn recalculate_node_positions(&mut self) {
        let total_nodes = self.graph.node_count() - 1; // Exclude central node
        let angle_increment = 360.0 / total_nodes as f32;

        for (i, node_index) in self.graph.node_indices().enumerate() {
            // Skip the central node
            if node_index != self.central_node_index {
                let angle_degree = angle_increment * i as f32;
                let angle_rad = angle_degree.to_radians();

                let central_circle = &self.graph[self.central_node_index];
                let new_x = central_circle.position.x + self.orbit_radius * angle_rad.cos();
                let new_y = central_circle.position.y + self.orbit_radius * angle_rad.sin();

                self.graph[node_index].position = egui::pos2(new_x, new_y);
            }
        }
    }


    // fn calculate_new_node_position(&self) -> egui::Pos2{
    //     // determine angle increment based on total num of nodes
    //     let total_nodes = self.graph.node_count();
    //     let angle_increment = 360.0 / total_nodes as f32;

    //     // calculate the angle for the new node
    //     let angle_degree = angle_increment * (total_nodes - 1) as f32;
    //     let angle_rad = angle_degree.to_radians();

    //     // calculate the position of the new node
    //     let central_circle = &self.graph[self.central_node_index];
    //     let new_x = central_circle.position.x + self.orbit_radius * angle_rad.cos();
    //     let new_y = central_circle.position.y + self.orbit_radius * angle_rad.sin();


    //     egui::pos2(new_x, new_y)
    // }
}


impl eframe::App for MindGraph {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let current_time = get_current_time() / 1000.0;


        egui::CentralPanel::default().show(ctx, |ui| {
            // Button to add a new circle
            if ui.button("Add Circle").clicked() {
                self.add_node(); // Method to add a new node
            }
            
            let painter = ui.painter();
            let circle_color = egui::Color32::WHITE;
            let stroke_width = 2.0;
            let stroke = egui::Stroke::new(stroke_width, circle_color);

            // Iterate over nodes to draw circles and lines
            for node_index in self.graph.node_indices() {
                let circle = &self.graph[node_index];
                painter.circle(circle.position, circle.radius, egui::Color32::TRANSPARENT, stroke);
                
                // Draw connections to other nodes
                for edge in self.graph.edges(node_index) {
                    let target_node = &self.graph[edge.target()];
                    painter.line_segment(
                        [circle.position, target_node.position], 
                        (2.0, egui::Color32::WHITE)
                    );
                }
            }





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