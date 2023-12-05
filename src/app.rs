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
        };

        // add the new node to the graph
        let new_node_index = self.graph.add_node(new_node);

        // create edge to central node
        self.graph.add_edge(self.current_central_node_index, new_node_index, ());
        
        // Recalculate positions for all nodes
        self.recalculate_node_positions();
    }

    fn recalculate_node_positions(&mut self) {
        

        // Collect the indices of the neighboring nodes
        let neighbors: Vec<NodeIndex> = self.graph.neighbors(self.current_central_node_index).collect();

        let neighbor_count = neighbors.len();
        let angle_increment = 360.0 / neighbor_count as f32;

        // Now iterate over the collected indices
        for (i, node_index) in neighbors.into_iter().enumerate() {
            let angle_degree = angle_increment * i as f32;
            let angle_rad = angle_degree.to_radians();

            let central_circle = &self.graph[self.current_central_node_index];
            let new_x = central_circle.position.x + self.orbit_radius * angle_rad.cos();
            let new_y = central_circle.position.y + self.orbit_radius * angle_rad.sin();

            // Now it's safe to mutate the graph since we're not iterating over it directly
            self.graph[node_index].position = egui::pos2(new_x, new_y);
        }





        // for (i, node_index) in self.graph.node_indices().enumerate() {
        //     // Skip the central node
        //     if node_index != self.current_central_node_index {
        //         let angle_degree = angle_increment * i as f32;
        //         let angle_rad = angle_degree.to_radians();
    
        //         let central_circle = &self.graph[self.current_central_node_index];
        //         let new_x = central_circle.position.x + self.orbit_radius * angle_rad.cos();
        //         let new_y = central_circle.position.y + self.orbit_radius * angle_rad.sin();
    
        //         self.graph[node_index].position = egui::pos2(new_x, new_y);
        //     }
        // }
    }


    pub fn set_new_central_node(&mut self, new_central_node_index: NodeIndex) {
        //self.recalculate_node_positions();

        self.current_central_node_index = new_central_node_index;
        self.recalculate_node_positions();
    }

    pub fn draw_graph(&mut self, ui: &egui::Ui, current_time: f64) {
        let painter = ui.painter();
        let circle_color = egui::Color32::WHITE;
        let stroke_width = 2.0;
        let stroke = egui::Stroke::new(stroke_width, circle_color);

        let current_central_circle = &self.graph[self.current_central_node_index];
        painter.circle(current_central_circle.position, current_central_circle.radius, egui::Color32::TRANSPARENT, stroke);

        //let current_circle_position = self.current_central_node_index;

        // Iterate over nodes to draw circles and lines
        for node_index in self.graph.neighbors(self.current_central_node_index) {
            let node_id = node_index.index() as f32;
            let circle = &self.graph[node_index];

            // get the position of the current central node to use in the for loop
            let mut central_node_position = egui::pos2(0.0, 0.0);
            if let Some(temp_central_node_data) = self.graph.node_weight(self.current_central_node_index) {
                // Access the position of the central node
                central_node_position = temp_central_node_data.position;
                //println!("Central node position: {:?}", central_node_position);
            }

            
            // Draw connections to other nodes
            for edge in self.graph.edges(self.current_central_node_index) {


                let target_node = &self.graph[edge.target()];

                // Calculate start and end points on the circle edges for floating positions
                let start_point = central_node_position; // trying to get position of central node
                let end_point = target_node.position;

                //self.recalculate_node_positions();

                painter.line_segment(
                    [start_point, end_point],
                    (2.0, egui::Color32::WHITE)
                );
            }


            painter.circle(circle.position, circle.radius, egui::Color32::TRANSPARENT, stroke);

            
        }
        self.recalculate_node_positions();

        
    }
}

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
    
            // Button to add a new circle
            if ui.button("Add Circle").clicked() {
                self.add_node(); // Method to add a new node
                self.draw_graph(ui, current_time); // Draw the graph after adding a node
            } else {
                // Regular drawing of the graph
                self.draw_graph(ui, current_time);
            }




            // Check for mouse click
            ctx.input(|input| {
                if input.pointer.any_pressed() {
                    if let Some(pointer_pos) = input.pointer.interact_pos() {
                        for node_index in self.graph.node_indices() {
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