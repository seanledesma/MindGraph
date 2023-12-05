use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use egui::{Stroke, Color32};
use petgraph::graph::UnGraph;
use petgraph::visit::EdgeRef;
use std::collections::HashMap;
use petgraph::visit::{Bfs, Visitable};
use petgraph::graph::NodeIndex;
use petgraph::visit::VisitMap;
use std::collections::VecDeque;

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
    orbit_radius: f32, 
    scale: f32,
    orbit_center: Option<NodeIndex>,
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
    original_central_node_index: NodeIndex,
    #[serde(skip)]
    focused_node_index: Option<NodeIndex>,
}


impl Default for MindGraph {
    fn default() -> Self {
        let mut graph = CircleGraph::default();
        let central_circle = Circle {
            position: egui::pos2(200.0, 200.0),
            radius: 40.0,
            orbit_radius: 300.0,  // Set the initial orbit radius for the central circle
            scale: 1.0,            // Normal scale
            orbit_center: None,
        };
        let central_node_index = graph.add_node(central_circle);
        Self {
            frame_counter: 0,
            editor_text: String::new(),
            show_popup: false,
            graph,
            central_node_index,  // This is the same as the original on initialization
            original_central_node_index: central_node_index,
            focused_node_index: None,
        }
    }
}



impl MindGraph {
    pub fn add_node(&mut self, orbit_center_index: NodeIndex) {
        // Determine the orbit_radius for this new node
        // You may want to set a specific radius or calculate it based on certain criteria
        let orbit_radius = self.calculate_orbit_radius_for_new_node(orbit_center_index);

        // Create a Circle
        let new_node = Circle {
            position: egui::pos2(0.0, 0.0), // Initial position, will be set in recalculate_node_positions
            radius: 20.0, // Example radius for the new node
            orbit_radius,
            scale: 1.0, // Normal scale
            orbit_center: Some(orbit_center_index),
        };

        // Add the new node to the graph and create an edge to the orbit center
        let new_node_index = self.graph.add_node(new_node);
        self.graph.add_edge(orbit_center_index, new_node_index, ());
        
        // Recalculate positions for all nodes
        self.recalculate_node_positions();
    }

    fn calculate_orbit_radius_for_new_node(&self, orbit_center_index: NodeIndex) -> f32 {
        // Example calculation - you can adjust this logic as needed
        if orbit_center_index == self.original_central_node_index {
            100.0 // Default radius for nodes around the original central node
        } else {
            150.0 // Different radius for nodes around other centers
        }
    }

    pub fn set_focus(&mut self, new_focus: NodeIndex) {
        // First, collect the indices of immediate neighbors to avoid borrowing issues
        let neighbors: Vec<NodeIndex> = self.graph
            .neighbors(new_focus)
            .collect();

        // Define orbit radius values for focused and non-focused nodes
        let focused_orbit_radius = 150.0; // Example larger radius for new focus
        let neighbor_orbit_radius = 100.0; // Example radius for neighbors
        let other_orbit_radius = 50.0; // Smaller radius for other nodes

        // Now, iterate over all nodes to update their properties
        for node_index in self.graph.node_indices() {
            let node = &mut self.graph[node_index];
            if node_index == new_focus {
                node.scale = 1.0; // Focused node remains the same size
                node.orbit_radius = focused_orbit_radius;
            } else if neighbors.contains(&node_index) {
                node.scale = 1.0; // Immediate neighbors remain the same size
                node.orbit_radius = neighbor_orbit_radius;
            } else {
                node.scale = 0.25; // Other nodes are scaled down
                node.orbit_radius = other_orbit_radius;
            }
        }

        // Update the focused node index
        self.focused_node_index = Some(new_focus);

        // Recalculate the positions and sizes of the nodes
        self.recalculate_node_positions();
    }

    fn recalculate_node_positions(&mut self) {
        for node_index in self.graph.node_indices() {
            let node = &self.graph[node_index];

            // Check if the node has an orbit center and calculate its position if it does
            if let Some(orbit_center_index) = node.orbit_center {
                let orbit_center = &self.graph[orbit_center_index];

                // Calculate the angle increment and new position based on the orbit center
                let angle_increment = 360.0 / self.graph.node_count() as f32; // This might need adjusting
                let angle_degree = angle_increment * node_index.index() as f32;
                let angle_rad = angle_degree.to_radians();

                // Calculate the new position based on the orbit center and the node's orbit_radius
                let new_x = orbit_center.position.x + node.orbit_radius * angle_rad.cos();
                let new_y = orbit_center.position.y + node.orbit_radius * angle_rad.sin();

                // Update the node's position
                self.graph[node_index].position = egui::pos2(new_x, new_y);
            }
            // For nodes without an orbit center, you might want to set a default position or skip
        }
    }

    // Helper function to count the number of nodes orbiting a specific center
    fn count_orbiting_nodes(&self, center_index: NodeIndex) -> usize {
        self.graph.node_indices()
            .filter(|&n| self.graph[n].orbit_center == Some(center_index))
            .count()
    }


    // Helper function to determine a node's position in its orbit
    fn determine_position_in_orbit(&self, node_index: NodeIndex, center_index: NodeIndex) -> usize {
        self.graph.node_indices()
            .filter(|&n| self.graph[n].orbit_center == Some(center_index))
            .position(|n| n == node_index)
            .unwrap_or(0)
    }

    pub fn set_new_central_node(&mut self, new_central_node_index: NodeIndex) {
        // Update the orbit_center for the new central node
        if let Some(node) = self.graph.node_weight_mut(new_central_node_index) {
            node.orbit_center = None; // Central node does not orbit around any other node
            node.orbit_radius = 300.0; // Example larger radius for the new central node
        }
    
        self.central_node_index = new_central_node_index;
        self.recalculate_node_positions();
    }
    

}

impl MindGraph {
    // Function to calculate distances from the original node
    fn calculate_distances(&self) -> HashMap<NodeIndex, usize> {
        let mut distances = HashMap::new();
        let mut visited = self.graph.visit_map();
        let mut queue = VecDeque::new();

        // Start from the original central node
        distances.insert(self.original_central_node_index, 0);
        queue.push_back((self.original_central_node_index, 0));

        while let Some((node_index, dist)) = queue.pop_front() {
            // Visit all neighbors
            for neighbor in self.graph.neighbors(node_index) {
                if !visited.is_visited(&neighbor) {
                    visited.visit(neighbor);
                    distances.insert(neighbor, dist + 1);
                    queue.push_back((neighbor, dist + 1));
                }
            }
        }

        distances
    }

    // Function to determine color based on distance
    fn color_for_distance(&self, distance: usize) -> Color32 {
        // Example: Different shades based on distance
        // Adjust the logic here to choose colors as you see fit
        match distance {
            0 => Color32::GOLD, // Original node
            1 => Color32::GREEN,
            2 => Color32::BLUE,
            3 => Color32::RED,
            _ => Color32::GRAY,
        }
    }

    pub fn draw_graph(&self, ui: &egui::Ui, current_time: f64) {
        let painter = ui.painter();
        let stroke_width = 2.0;

        // Calculate distances from the original node
        let distances = self.calculate_distances();

        for node_index in self.graph.node_indices() {
            let node_id = node_index.index() as f32;
            let circle = &self.graph[node_index];

            // Calculate the floating position for the circle
            let offset_x = (current_time as f32 + node_id).sin() * 5.0;
            let offset_y = (current_time as f32 + node_id).cos() * 5.0;
            let floating_position = egui::pos2(circle.position.x + offset_x, circle.position.y + offset_y);

            // Determine the color based on the distance
            let distance = distances.get(&node_index).cloned().unwrap_or(usize::MAX);
            let circle_color = self.color_for_distance(distance);
            let stroke = egui::Stroke::new(stroke_width, circle_color);

            painter.circle(floating_position, circle.radius, egui::Color32::TRANSPARENT, stroke);

            
            // Draw connections to other nodes
            for edge in self.graph.edges(node_index) {
                let target_node = &self.graph[edge.target()];

                // Calculate the floating position of the target node
                let target_node_id = edge.target().index() as f32;
                let target_offset_x = (current_time as f32 + target_node_id).sin() * 3.0;
                let target_offset_y = (current_time as f32 + target_node_id).cos() * 3.0;
                let target_floating_position = egui::pos2(target_node.position.x + target_offset_x, target_node.position.y + target_offset_y);
    
                // Calculate direction vectors for the floating positions
                let direction = target_floating_position - floating_position;
                let norm_direction = direction.normalized();

                // Calculate start and end points on the circle edges for floating positions
                let start_point = floating_position + norm_direction * circle.radius;
                let end_point = target_floating_position - norm_direction * target_node.radius;

                painter.line_segment(
                    [start_point, end_point],
                    (2.0, egui::Color32::WHITE)
                );
            }
        }
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
            self.graph[self.central_node_index].position = central_position;
            // might need the following if we ever change radius / orbit 
            // self.recalculate_node_positions();
    
            // Button to add a new circle
            if ui.button("Add Circle").clicked() {
                if let Some(focused_index) = self.focused_node_index {
                    // If there is a focused node, add the new node around it
                    self.add_node(focused_index);
                } else {
                    // If there is no focused node, add the new node around the original central node
                    self.add_node(self.original_central_node_index);
                }
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