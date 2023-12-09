use petgraph::graph::NodeIndex;
use petgraph::graph::UnGraph;
use petgraph::visit::EdgeRef;

/// Represents a node in MindGraph, visualized as a circle.
///
/// Each `Circle` corresponds to an idea or concept and contains a title and notes for details.
/// Users can navigate between these circles to explore interconnected thoughts or ideas.
/// ChatGPT assisted in writing these docs
///
/// # Examples
///
/// ```
/// use mind_graph::app::Circle;
///
/// let circle = Circle {
///     position: egui::pos2(50.0, 50.0),
///     radius: 10.0,
///     title: "My Circle".to_string(),
///     notes: "Some notes".to_string(),
/// };
/// assert_eq!(circle.title, "My Circle");
/// ```
pub struct Circle {
    pub position: egui::Pos2,
    pub radius: f32,
    pub title: String,
    pub notes: String,
}
type CircleGraph = UnGraph<Circle, ()>;

/// A mind map interface using an undirected graph to represent interconnected ideas or concepts.
///
/// MindGraph allows users to add, select, and navigate nodes with ease.
/// Each node represents a distinct concept or idea, which can be expanded with detailed notes.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct MindGraph {
    editor_text: String,
    show_popup: bool,

    #[serde(skip)]
    pub graph: CircleGraph,
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
            radius: 65.0,
            title: "".to_string(),
            notes: "".to_string(),
        };
        let central_node_index = graph.add_node(central_circle);

        Self {
            editor_text: String::new(),
            show_popup: false,
            graph,
            current_central_node_index: central_node_index, // this is the same as the original on initialization, but will change later
            original_central_node_index: central_node_index, // this may be useful for home logic later
            orbit_radius: 300.0,
        }
    }
}

// Dealing with graph logic
impl MindGraph {
    /// Adds a new idea or concept to the mind map as a `Circle` node.
    ///
    /// This method connects the new node to the current central node and recalculates the positions of all nodes.
    /// It represents the expansion of thoughts from the central concept.
    /// # Examples
    ///
    /// ```
    /// use mind_graph::app::{MindGraph, Circle};
    ///
    /// let mut graph = MindGraph::default();
    /// assert_eq!(graph.graph.node_count(), 1); // Only the central node exists initially
    ///
    /// graph.add_node();
    /// assert_eq!(graph.graph.node_count(), 2); // Verify a new node has been added
    /// ```
    pub fn add_node(&mut self) {
        // create a Circle
        let new_node = Circle {
            position: egui::pos2(0.0, 0.0),
            radius: 65.0,
            title: "".to_string(),
            notes: "".to_string(),
        };

        // add the new node to the graph
        let new_node_index = self.graph.add_node(new_node);

        // create an edge to the current central node
        self.graph
            .add_edge(self.current_central_node_index, new_node_index, ());

        // recalculate positions for all nodes
        self.recalculate_node_positions();
    }

    /// Changes the focus of the mind map to a new central idea.
    ///
    /// This method is useful for exploring different branches of thought from a new perspective.
    /// # Arguments
    /// * `new_central_node_index` - The index of the node to be set as the new central focus.
    pub fn set_new_central_node(&mut self, new_central_node_index: NodeIndex) {
        self.current_central_node_index = new_central_node_index;
        self.recalculate_node_positions();
    }

    fn recalculate_node_positions(&mut self) {
        // get all the indices of the neighboring nodes
        let neighbors: Vec<NodeIndex> = self
            .graph
            .neighbors(self.current_central_node_index)
            .collect();

        let neighbor_count = neighbors.len();
        let angle_increment = 360.0 / neighbor_count as f32;
        // new circles automatically added to the right, this allows you to change that
        let start_angle = 0.0;

        // iterate over the indices
        for (i, node_index) in neighbors.into_iter().enumerate() {
            let angle_degree = start_angle + angle_increment * i as f32;
            let angle_rad = angle_degree.to_radians();

            let central_circle = &self.graph[self.current_central_node_index];
            let new_x = central_circle.position.x + self.orbit_radius * angle_rad.cos();
            let new_y = central_circle.position.y + self.orbit_radius * angle_rad.sin();

            self.graph[node_index].position = egui::pos2(new_x, new_y);
        }
    }
}

// Dealing with UI
impl MindGraph {
    /// Renders the graphical representation of the mind map onto the UI.
    ///
    /// This function is responsible for drawing each node in the graph as a circle, along with lines connecting
    /// each node to its neighbors, creating a visual representation of the mind map. It first draws the central
    /// node and then iterates over its neighbors to draw them and the connecting lines.
    ///
    /// # Arguments
    ///
    /// * `ui` - A mutable reference to an `egui::Ui` context, which provides the necessary tools to draw on the UI.
    ///
    /// # Implementation Details
    ///
    /// The function utilizes `egui`'s painting facilities to draw circles for each node and lines for the edges
    /// between nodes. It calculates the positions for the start and end points of the lines based on the positions
    /// and radii of the nodes to ensure that lines connect the nodes' perimeters rather than their centers.
    ///
    /// The central node is drawn first, followed by its neighboring nodes. The positions of these nodes are
    /// determined by the internal graph structure and the current layout calculated by the graph logic.
    ///
    /// This function is typically called within the `update` method
    ///
    pub fn draw_graph(&mut self, ui: &mut egui::Ui) {
        let painter = ui.painter();
        let circle_color = egui::Color32::WHITE;
        let stroke_width = 2.0;
        let stroke = egui::Stroke::new(stroke_width, circle_color);

        // paint central circle first, then neighbors later
        let current_central_circle = &self.graph[self.current_central_node_index];
        painter.circle(
            current_central_circle.position,
            current_central_circle.radius,
            egui::Color32::TRANSPARENT,
            stroke,
        );

        // get the position of the current central node to use in the for loop
        let mut central_node_position = egui::pos2(0.0, 0.0);

        // get this first to avoid borrow checker woes
        let neighbor_indices: Vec<_> = self
            .graph
            .neighbors(self.current_central_node_index)
            .collect();

        // iterate over nodes to draw circles and lines
        for node_index in neighbor_indices {
            let neighbor_node = &self.graph[node_index];

            painter.circle(
                neighbor_node.position,
                neighbor_node.radius,
                egui::Color32::TRANSPARENT,
                stroke,
            );
            //self.draw_text_boxes(ui);

            if let Some(temp_central_node_data) =
                self.graph.node_weight(self.current_central_node_index)
            {
                // I forgot why I declared these outside the for loop, probably ran into borrowing problems
                central_node_position = temp_central_node_data.position;
            }

            // draw connections to other nodes
            for edge in self.graph.edges(self.current_central_node_index) {
                let target_node = &self.graph[edge.target()];

                // need the following to stop painting edges all the way to center of circles
                let direction = target_node.position - central_node_position;
                let norm_direction = direction.normalized();
                // using target_node.radius for convenience, since all nodes share same radius (for now)
                let start_point = central_node_position + norm_direction * target_node.radius;
                let end_point = target_node.position - norm_direction * target_node.radius;

                painter.line_segment([start_point, end_point], (2.0, egui::Color32::WHITE));
            }
        }
    }

    /// Draws text boxes for editing titles of nodes in the mind map.
    ///
    /// This function creates interactive text fields above each node, allowing users to edit the titles
    /// of both the central node and its neighboring nodes. The text boxes are positioned relative to
    /// each node's coordinates on the UI.
    ///
    /// # Arguments
    ///
    /// * `ui` - A mutable reference to an `egui::Ui` context, which is used to draw and manage UI elements.
    ///
    /// The text fields are styled with the `HEADING` font style and sized to fit the node titles.
    /// Positions are calculated based on the node's position to ensure text boxes appear near their respective nodes.
    ///
    /// # Examples
    ///
    /// This function is typically used within the `update` method
    ///
    pub fn draw_text_boxes(&mut self, ui: &mut egui::Ui) {
        let central_node = &mut self.graph[self.current_central_node_index];

        // place the text field for ONLY the current central node
        let text_field_size = egui::vec2(100.0, 10.0);
        let text_field_rect = egui::Rect::from_min_size(
            egui::pos2(
                // funny way of saying 45.45? but useful to know for dynanicism.
                central_node.position.x - text_field_size.x / 2.2,
                // ugly hard-coded number, totally dependent on circle size, change later
                central_node.position.y - 95.0,
            ),
            text_field_size,
        );

        // actually add the text edit single line above circle, w/ HEADING font style
        ui.allocate_ui_at_rect(text_field_rect, |ui| {
            ui.add(
                egui::TextEdit::singleline(&mut central_node.title).font(egui::TextStyle::Heading),
            );
        });

        // get all indices first to avoid borrow checker getting mad
        let neighbor_indices: Vec<_> = self
            .graph
            .neighbors(self.current_central_node_index)
            .collect();

        for node_index in neighbor_indices {
            // important to borrow mutably
            let neighbor_node = &mut self.graph[node_index];

            // now place the text fields for the neighbors
            let text_field_size = egui::vec2(100.0, 10.0);
            let text_field_rect = egui::Rect::from_min_size(
                egui::pos2(
                    // funny way of saying 45.45? but useful to know for dynanicism.
                    neighbor_node.position.x - text_field_size.x / 2.2,
                    // ugly hard-coded number, totally dependent on circle size, change later
                    neighbor_node.position.y - 95.0,
                ),
                text_field_size,
            );

            // actually add the text edit single line above circle, w/ HEADING font style
            ui.allocate_ui_at_rect(text_field_rect, |ui| {
                ui.add(
                    egui::TextEdit::singleline(&mut neighbor_node.title)
                        .font(egui::TextStyle::Heading),
                );
            });
        }
    }
}

//
impl eframe::App for MindGraph {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Updates the UI elements of the MindGraph application in each frame.
    ///
    /// This function is called by the `eframe` framework on every frame. It's responsible for
    /// rendering the UI elements, handling user interactions, and updating the state of the application.
    ///
    /// It includes the following functionalities:
    /// - Rendering the title and layout of the central panel.
    /// - Positioning the central node based on the available panel size.
    /// - Creating and handling the "Add Circle" and "Open Notes" buttons.
    /// - Managing the notes window for the current central node.
    /// - Detecting mouse clicks on neighboring nodes to change the central node.
    ///
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // title
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.heading("MindGraph");
            });

            // make sure central circle is in center
            let panel_size = ui.available_size();
            let central_position = egui::pos2(panel_size.x / 2.0, panel_size.y / 2.0);
            self.graph[self.current_central_node_index].position = central_position;

            // place the buttons on the top left and right corners
            ui.horizontal(|ui| {
                // button to add a new circle
                if ui.button("Add Circle").clicked() {
                    self.recalculate_node_positions();

                    self.add_node();
                    self.draw_graph(ui);
                    self.draw_text_boxes(ui);
                } else {
                    // regular drawing of the graph
                    self.recalculate_node_positions();

                    self.draw_graph(ui);
                    self.draw_text_boxes(ui);
                }

                // getting the open notes button to the top right
                ui.with_layout(egui::Layout::top_down(egui::Align::Max), |ui| {
                    // create button to open central node's notes
                    //ui.allocate_space(egui::vec2(0.0, 0.0));
                    if ui.button("Open Notes").clicked() {
                        self.show_popup = true;
                    }
                });
            });

            // notes window logic
            if self.show_popup {
                if let Some(central_node) =
                    self.graph.node_weight_mut(self.current_central_node_index)
                {
                    egui::Window::new("Notes")
                        .open(&mut self.show_popup) // Bind the window's open state to show_text_editor
                        .show(ctx, |ui| {
                            ui.text_edit_multiline(&mut central_node.notes);
                        });
                }
            }

            // check for mouse click on any of neighboring circles
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recalculate_node_positions() {
        // creating a graph with one central node
        let mut mind_graph = MindGraph::default();
        let central_circle = &mind_graph.graph[mind_graph.current_central_node_index];
        let central_node_position = central_circle.position;

        // adding one node
        let new_node = Circle {
            position: egui::pos2(0.0, 0.0),
            radius: 65.0,
            title: "Test Node".to_string(),
            notes: "Test notes".to_string(),
        };
        let new_node_index = mind_graph.graph.add_node(new_node);
        mind_graph
            .graph
            .add_edge(mind_graph.current_central_node_index, new_node_index, ());

        // call recalculate_node_positions
        mind_graph.recalculate_node_positions();

        // assert that the new node is 300 directly to the right (default orbit)
        let expected_x = central_node_position.x + 300.0; // 500.0
        let expected_y = central_node_position.y;
        let recalculated_node = &mind_graph.graph[new_node_index];
        assert!((recalculated_node.position.x - expected_x).abs() < f32::EPSILON);
        assert!((recalculated_node.position.y - expected_y).abs() < f32::EPSILON);
    }
}
