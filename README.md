# MindGraph

MindGraph is a way to map out your thoughts. 
[Check out a live demo!](http://seanledesma.com/mind_graph/)

![mind_graph_photo](https://github.com/seanledesma/MindGraph/assets/87875153/ea7155af-8994-47ed-b3b7-0dcec3245419)


Have you ever experienced the need to document a pivotal idea, only to realize you lack the proper place to record it? Or maybe you found yourself deeply engrossed in contemplation, arriving at a significant insight, but were unable to recall the path that led you there? Often, our most critical thoughts emerge when we delve deeply into complex subjects, and it can be challenging to capture these insights effectively without understanding the context that guided you to them.

MindGraph is designed to address this challenge. Utilizing egui, an immediate mode graphical user interface, MindGraph places you at a single node, providing a foundation from which to organize your thoughts. You have the freedom to structure your ideas as you see fit. For instance, you might begin by creating a node labeled 'Hobbies.' From this starting point, the possibilities are endless. You can expand by adding related nodes, such as 'Programming,' and attach specific notes to these nodes. Further branching out, you could create additional nodes named 'C++', 'Rust', 'Python,' each accompanied by its unique set of notes, allowing for a comprehensive and structured exploration of your ideas.

MindGraph's primary objective is to offer a user-friendly interface for adding and selecting nodes with ease. Users can click to select a node, assign it a title, and access notes associated with that node. Navigating the graph swiftly and intuitively is a key aspect of the design. Additionally, the interface includes straightforward mechanisms to return to the previous node, ensuring seamless navigation.

Readme done in collaboration with ChatGPT


# Getting Started
Currently using Trunk to build for web target.

1. Install the required target with rustup target add wasm32-unknown-unknown.
2. Install Trunk with ```cargo install --locked trunk```.
3. Run ```trunk serve``` to build and serve on http://127.0.0.1:8080. Trunk will rebuild automatically if you edit the project.
4. Open http://localhost:8080/index.html#dev in a browser.


# Stretch Goals

1. color node leading home
2. when entering new node, put new node on left side, instead of right. Keep placement after adding node the same.
3. Go home button
4. persistent note storage
