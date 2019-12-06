use petgraph::algo::astar;
use petgraph::graph::NodeIndex;
use petgraph::{Graph, Undirected};
use std::collections::HashMap;

#[derive(Default, Clone, Debug)]
pub struct Map {
  pub orbits: HashMap<String, Vec<String>>,
}

impl Map {
  pub fn number_of_orbits(&self, planet: &str) -> i32 {
    let planets_that_orbits = match self.orbits.get(planet) {
      Some(planets) => planets.clone(),
      None => Vec::new(),
    };
    planets_that_orbits.iter().fold(0, |acc, other_planet| {
      acc + self.number_of_orbits(other_planet) + planets_that_orbits.len() as i32
    })
  }

  // pub fn number_of_steps_til_santa(&self, planet: &str) -> i32 {
  //   astar(graph: G, start: G::NodeId, mut is_goal: IsGoal, mut edge_cost: F, mut estimate_cost: H)
  // }
}

#[aoc_generator(day6, part1)]
pub fn input_generator_part1(input: &str) -> Map {
  input.lines().fold(Map::default(), |map, l| {
    let mut new_map = map.clone();
    let mut iter = l.split(")");
    let value = iter.next().unwrap().to_string().trim().to_string();
    let key = iter.next().unwrap().to_string().trim().to_string();

    if let Some(entry) = new_map.orbits.get_mut(&key) {
      entry.push(value.clone());
    } else {
      new_map.orbits.insert(key.clone(), vec![value.clone()]);
    }

    new_map
  })
}

pub fn find_or_create_node(
  graph: &mut Graph<String, String, Undirected, u32>,
  planet: String,
) -> NodeIndex<u32> {
  find_node(graph, planet.clone()).unwrap_or_else(|| graph.add_node(planet))
}

pub fn find_node(
  graph: &Graph<String, String, Undirected, u32>,
  planet: String,
) -> Option<NodeIndex<u32>> {
  graph
    .node_indices()
    .find(|node_index| graph.node_weight(node_index.clone()) == Some(&planet))
}

#[aoc_generator(day6, part2)]
pub fn input_generator_part2(input: &str) -> Graph<String, String, Undirected, u32> {
  input.lines().fold(
    Graph::<String, String, Undirected>::new_undirected(),
    |graph, l| {
      let mut new_graph = graph.clone();
      let planet_names: Vec<String> = l.split(")").map(|s| s.trim().to_string()).collect();

      let node_one = match find_node(&new_graph, planet_names[0].clone()) {
        Some(node) => node,
        None => new_graph.add_node(planet_names[0].clone()),
      };

      let node_two = match find_node(&new_graph, planet_names[1].clone()) {
        Some(node) => node,
        None => new_graph.add_node(planet_names[1].clone()),
      };

      new_graph.extend_with_edges(&[(node_one, node_two)]);
      new_graph
    },
  )
}

#[aoc(day6, part1)]
pub fn solve_part1(input: &Map) -> i32 {
  input
    .orbits
    .keys()
    .fold(0, |acc, planet| acc + input.number_of_orbits(planet))
}

#[aoc(day6, part2)]
pub fn solve_part2(input: &Graph<String, String, Undirected, u32>) -> i32 {
  let santa = find_node(input, "SAN".into()).unwrap();
  let you = find_node(input, "YOU".into()).unwrap();
  let result = astar(input, you, |finish| finish == santa, |_| 1, |_| 0);
  result.unwrap().0 - 2
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  pub fn test_orbits() {
    let input = "COM)B
    B)C
    C)D
    D)E
    E)F
    B)G
    G)H
    D)I
    E)J
    J)K
    K)L";

    let map: Map = input_generator_part1(input);

    println!("{:?}", map);

    assert_eq!(7, map.number_of_orbits("L"))
  }
}
