use std::cmp::Ordering;
use std::collections::BinaryHeap;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// Rust BinaryHeap es max-heap por defecto.
// Invertimos el orden para simular un min-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Cada arista tiene un nodo destino y un costo.
#[derive(Clone)]
struct Edge {
    node: usize,
    cost: usize,
}

fn dijkstra(graph: &Vec<Vec<Edge>>, start: usize) -> Vec<usize> {
    let n = graph.len();
    let mut dist = vec![usize::MAX; n];
    let mut heap = BinaryHeap::new();

    dist[start] = 0;

    heap.push(State {
        cost: 0,
        position: start,
    });

    while let Some(State { cost, position }) = heap.pop() {
        // Si ya encontramos una mejor distancia antes, ignoramos esta.
        if cost > dist[position] {
            continue;
        }

        for edge in &graph[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            if next.cost < dist[next.position] {
                dist[next.position] = next.cost;
                heap.push(next);
            }
        }
    }

    dist
}

fn print_result(distances: Vec<usize>) {
    for (node, distance) in distances.iter().enumerate() {
        if *distance == usize::MAX {
            println!("Nodo {}: inalcanzable", node);
        } else {
            println!("Nodo {}: distancia minima = {}", node, distance);
        }
    }
}

fn main() {
    println!("Prueba 1: si el grafo es basico");
    let graph1 = vec![
        vec![Edge { node: 1, cost: 4 }, Edge { node: 2, cost: 1 }],
        vec![Edge { node: 3, cost: 1 }],
        vec![Edge { node: 1, cost: 2 }, Edge { node: 3, cost: 5 }],
        vec![],
    ];
    let distances1 = dijkstra(&graph1, 0);
    print_result(distances1);

    println!("\nPrueba 2: si el nodo es inalcanzable");
    let graph2 = vec![
        vec![Edge { node: 1, cost: 3 }],
        vec![Edge { node: 2, cost: 4 }],
        vec![],
        vec![],
    ];
    let distances2 = dijkstra(&graph2, 0);
    print_result(distances2);

    println!("\nPrueba 3: si el grafo tiene multiples caminos posibles");
    let graph3 = vec![
        vec![Edge { node: 1, cost: 10 }, Edge { node: 2, cost: 2 }],
        vec![Edge { node: 3, cost: 1 }],
        vec![Edge { node: 1, cost: 3 }, Edge { node: 3, cost: 9 }],
        vec![],
    ];
    let distances3 = dijkstra(&graph3, 0);
    print_result(distances3);
}