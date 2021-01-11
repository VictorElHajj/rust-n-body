# rust-n-body
Goal is to implement a N-Body simulation in the Rust language using the Barnes-Hut algorithm and velocity verlet integration.

Optimization steps:
Replace self.size/2 twice with new_size = self.size/2
Make inside_region function to avoid creating new rectangle to check if inside

Todo:
Track total system energy
Better smoothing
Generalize octree insertion (iterator over regions?)
