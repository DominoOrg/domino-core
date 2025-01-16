#[cfg(test)]
mod unit_tests {
    use crate::types::graph_types::graph::Graph;
    use crate::functionalities::graph_common::find_eulerian_cycle;

    #[test]
    fn find_eulerian_cycle_no_random_works() {
        for n in 3..=12 {
            let graph = Graph::regular(n);
            let eulerian_cycle = find_eulerian_cycle(&graph, false);
            if n % 2 == 0 {
                assert_eq!(eulerian_cycle.len() - 1, (n + 1) * (n + 2) / 2);                
            } else {
                assert_eq!(eulerian_cycle.len() - 1, (n + 1) * (n + 1) / 2);                
            }
        }
    }

    #[test]
    fn find_eulerian_cycle_random_works() {
        for n in 3..=12 {
            let graph = Graph::regular(n);
            let eulerian_cycle = find_eulerian_cycle(&graph, true);
            if n % 2 == 0 {
                assert_eq!(eulerian_cycle.len() - 1, (n + 1) * (n + 2) / 2);                
            } else {
                assert_eq!(eulerian_cycle.len() - 1, (n + 1) * (n + 1) / 2);                
            }
        }
    }
}