struct Node {
    identity: i32,
    address: u32,
    activation: u32,
    bias: f32,
    sum: f32,
    connections_index_from: u32,
    connections_index_to_exc: u32
}

struct Connection {
    from_state_index: u32,
    weight: f32
}

@group(0) 
@binding(0)
var<storage, read_write> state: array<f32>;

@group(0)
@binding(1)
var<storage, read_write> nodes: array<Node>;

@group(0)
@binding(2)
var<storage, read_write> all_nodes: array<Node>;

@group(0)
@binding(3)
var<storage, read> connections: array<Connection>;

fn sigmoid(v: f32) -> f32{
    return 1.0 / (1.0 + exp(v*5.0));
}
fn act_tanh(v: f32) -> f32{
    return tanh(v * 5.0);
}

@compute @workgroup_size(32, 1, 1)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let node = nodes[global_id.x];
    let y = connections[0];
    let x = all_nodes[0];
    
    // state[node.address]= 1.0;

    let has_connections = node.connections_index_from != node.connections_index_to_exc;
    if (has_connections){
        for(var i: u32 = node.connections_index_from; i < node.connections_index_to_exc; i++) {
            let connection = connections[i];
            let from_node = state[connection.from_state_index];
            nodes[global_id.x].sum = nodes[global_id.x].sum + (from_node * connection.weight);
        }
        if (node.activation > 0u) {
            state[node.address] = act_tanh(nodes[global_id.x].sum + node.bias);
        }else{
            state[node.address] = nodes[global_id.x].sum + node.bias;
        }
    }
}