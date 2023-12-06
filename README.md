# MindGraph

MindGraph is a way to map out your thoughts. 

Have you ever experienced the need to document a pivotal idea, only to realize you lack the proper place to record it? Or maybe you found yourself deeply engrossed in contemplation, arriving at a significant insight, but were unable to recall the path that led you there? Often, our most critical thoughts emerge when we delve deeply into complex subjects, and it can be challenging to capture these insights effectively without understanding the context that guided you to them.

MindGraph is designed to address this challenge. Utilizing egui, an immediate mode graphical user interface, MindGraph places you at a single node, providing a foundation from which to organize your thoughts. You have the freedom to structure your ideas as you see fit. For instance, you might begin by creating a node labeled 'Hobbies.' From this starting point, the possibilities are endless. You can expand by adding related nodes, such as 'Programming,' and attach specific notes to these nodes. Further branching out, you could create additional nodes named 'C++', 'Rust', 'Python,' each accompanied by its unique set of notes, allowing for a comprehensive and structured exploration of your ideas.

MindGraph's primary objective is to offer a user-friendly interface for adding and selecting nodes with ease. Users can click to select a node, assign it a title, and access notes associated with that node. Navigating the graph swiftly and intuitively is a key aspect of the design. Additionally, the interface includes straightforward mechanisms to return to either the previous node or the home node, ensuring seamless navigation.

Stretch goals include: having the option to see the entire map of the graph made so far, and showing more than a nodes immediate neighbors.

To date, significant progress has been made on MindGraph's development. Key functionalities implemented include the capability to render a central circle on the screen, add nodes to the graph that are visually represented, and draw edges to connect these nodes. Additionally, I have enabled a feature that allows users to interactively select a node by clicking on it, which then becomes the central focus of the interface. This interactive functionality further extends to adding more nodes to the selected central node.

Currently, navigation can be somewhat confusing, as there are no explicit markers to indicate the origin node from which a user has transitioned. Additionally, there are no options for adding titles or notes to nodes, nor does it provide a direct option to return to the initial or 'home' node. 


# Getting Started
Currently using Trunk to build for web target.

1. Install the required target with rustup target add wasm32-unknown-unknown.
2. Install Trunk with ```cargo install --locked trunk```.
3. Run trunk serve to build and serve on http://127.0.0.1:8080. Trunk will rebuild automatically if you edit the project.
4. Open http://127.0.0.1:8080/index.html#dev in a browser.

