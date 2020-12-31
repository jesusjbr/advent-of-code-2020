use petgraph::graphmap::DiGraphMap;
use petgraph::visit::Walker;

use itertools::izip;
use std::iter;

fn get_edges_with_weights(rule: &str) -> impl Iterator<Item = (&str, &str, u32)> {
    //Parse a line of the input, and gives back an iterator of tuples of nodes(bags) including weights.
    //Dont give it input of rules that contains the text "contain no".
    let mut iter_parts = rule.split(" bags contain ");
    let (parent, rest) = (iter_parts.next().unwrap(), iter_parts.next().unwrap());
    let parent_iter = iter::repeat(parent);
    let rest_iter = rest
        .split(char::is_numeric)
        .skip(1)
        .map(|s| s.trim().rsplitn(2, ' ').skip(1).next().unwrap());
    let weights_iter = rest
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap());

    let edges_iterator = izip!(parent_iter, rest_iter, weights_iter);
    edges_iterator
}

fn build_graph(input: &str) -> DiGraphMap<&str, u32> {
    //Return the number of bag colors that can contain one shiny gold.
    // Create a new directed GraphMap.
    let edges = input
        .lines()
        .filter(|rule| !rule.contains(" contain no "))
        .flat_map(|rule| get_edges_with_weights(rule));
    DiGraphMap::<_, u32>::from_edges(edges)
}

fn bags_can_contain(gr: DiGraphMap<&str, u32>) -> usize {
    //Return the number of bag colors that can contain one shiny gold.
    let count = petgraph::visit::Dfs::new(&gr, "shiny gold")
        .iter(petgraph::visit::Reversed(&gr))
        .skip(1) // first is our starting node shiny gold
        .count();
    count
}

fn individual_bags_required(graph: &DiGraphMap<&str, u32>, start: &str, last_call: bool) -> u32 {
    //Given a graph and a starting node calculate the number of individual bags required.
    //Recursive solution, last_call parameter is used to decide if is needed to substract one in the main call.
    let mut count = 1;
    for e in graph.edges(start) {
        //e.weight == e.2
        //e.target_node == e.1
        count += e.2 * individual_bags_required(graph, e.1, false);
    }
    if !last_call {
        count
    } else {
        count - 1
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn example_part1() {
        let input = include_str!("../inputs/day7_example1.txt");
        let graph = build_graph(input);
        assert_eq!(bags_can_contain(graph), 4);
    }

    #[test]
    fn part1() {
        let input = include_str!("../inputs/day7.txt");
        let graph = build_graph(input);
        assert_eq!(bags_can_contain(graph), 151);
    }

    #[test]
    fn example1_part2() {
        let input = include_str!("../inputs/day7_example1.txt");
        let graph = build_graph(input);
        assert_eq!(individual_bags_required(&graph, "shiny gold", true), 32);
    }

    #[test]
    fn example2_part2() {
        let input = include_str!("../inputs/day7_example2.txt");
        let graph = build_graph(input);
        assert_eq!(individual_bags_required(&graph, "shiny gold", true), 126);
    }

    #[test]
    fn part2() {
        let input = include_str!("../inputs/day7.txt");
        let graph = build_graph(input);
        assert_eq!(individual_bags_required(&graph, "shiny gold", true), 41559);
    }
}
