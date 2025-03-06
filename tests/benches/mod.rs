use bencher::format_duration;
use domino_lib::{
    classify_puzzle, generate_puzzle, solve_puzzle, validate_puzzle, ComplexityClass, Puzzle,
};
use std::{
    cell::RefCell,
    rc::Rc,
    time::{Duration, Instant},
};

mod bencher;

fn bench_test_suite() -> Vec<usize> {
    // todo!("Add more lengths to test suite");
    return vec![5];
}

fn mock_puzzle(n: usize, complexity: ComplexityClass) -> Puzzle {
    let l = if n % 2 == 0 {
        (n + 1) * (n + 2) / 2
    } else {
        (n + 1) * (n + 1) / 2
    };
    let max_hole = (l as f32 - (n as f32 / 2.0).floor()) as usize;
    let log_factor: f32 = match complexity.0 {
        1 => 1.0 / l as f32,
        2 => 4.0 / 7.0,
        3 => 6.0 / 7.0,
        _ => 0.0,
    };
    let max_index = (max_hole as f32 * log_factor.sqrt()).ceil() as usize;
    let puzzle = generate_puzzle(n, max_index, false);
    puzzle
}

// The number of tests to run to have better accuracy on time estimations to execute the tasks,
// Greater accuracy => More time required to run
const TEST_REPETITIONS: usize = 10;

#[test]
fn bench_generate() {
    bench_test_suite().into_iter().for_each(|n| {
        let mut durations: Vec<Duration> = vec![];
        let mut now: Instant;
        let mut duration: Duration;
        for _ in 0..TEST_REPETITIONS {
            now = Instant::now();
            let puzzle = generate_puzzle(n, 1, false);
            duration = now.elapsed();
            durations.push(duration);
            if n % 2 == 0 {
                assert_eq!(puzzle.len(), (n + 1) * (n + 2) / 2);
            } else {
                assert_eq!(puzzle.len(), (n + 1) * (n + 1) / 2);
            }

            now = Instant::now();
            let puzzle = generate_puzzle(n, 1, true);
            duration = now.elapsed();
            durations.push(duration);
            if n % 2 == 0 {
                assert_eq!(puzzle.len(), (n + 1) * (n + 2) / 2);
            } else {
                assert_eq!(puzzle.len(), (n + 1) * (n + 1) / 2);
            }
        }

        let average = durations.iter().sum::<Duration>() / durations.len() as u32;
        println!(
            "Average time for generation with n = {n}: {}",
            format_duration(average)
        );
        durations.clear();
    });
}

#[test]
fn bench_solve() {
    bench_test_suite().into_iter().for_each(|n| {
        let mut durations: Vec<Duration> = vec![];
        let mut now: Instant;
        let mut duration: Duration;
        for _ in 0..TEST_REPETITIONS {
            let puzzle = generate_puzzle(n, 1, false);
            now = Instant::now();
            duration = now.elapsed();
            durations.push(duration);
            let solution = solve_puzzle(&puzzle).unwrap();
            assert_eq!(solution.len(), puzzle.len());

            let puzzle = generate_puzzle(n, 1, true);
            now = Instant::now();
            let solution = solve_puzzle(&puzzle);
            duration = now.elapsed();
            durations.push(duration);
            if let Ok(solution) = solution {
                assert_eq!(solution.len(), puzzle.len());
            }
        }

        let average = durations.iter().sum::<Duration>() / durations.len() as u32;
        println!(
            "Average time for solution with n = {n}: {}",
            format_duration(average)
        );
        durations.clear();
    });
}

#[test]
fn bench_validate() {
    bench_test_suite().into_iter().for_each(|n| {
        let mut durations: Vec<Duration> = vec![];
        let mut now: Instant;
        let mut duration: Duration;
        for _ in 0..TEST_REPETITIONS {
            // For each length a puzzle with a single tile missing is always valid
            let puzzle = generate_puzzle(n, 1, false);
            let solution = solve_puzzle(&puzzle).unwrap();
            now = Instant::now();
            let result = validate_puzzle(&puzzle, &solution);
            duration = now.elapsed();
            durations.push(duration);
            assert!(result.is_ok());

            // For each length an empty puzzle should result in not valid
            let puzzle = vec![
                None;
                if n % 2 == 0 {
                    (n + 1) * (n + 2) / 2
                } else {
                    (n + 1) * (n + 1) / 2
                }
            ];
            let solution = solve_puzzle(&puzzle).unwrap();
            now = Instant::now();
            let result = validate_puzzle(&puzzle, &solution);
            duration = now.elapsed();
            durations.push(duration);
            assert!(result.is_err());
        }

        let average = durations.iter().sum::<Duration>() / durations.len() as u32;
        println!(
            "Average time for validation with n = {n}: {}",
            format_duration(average)
        );
        durations.clear();
    });
}

#[test]
fn bench_classify() {
    bench_test_suite().into_iter().for_each(|n| {
    (1..=3).into_iter().map(|c| ComplexityClass::new(c))
    .for_each(|expected_complexity| {
      let mut durations: Vec<Duration> = vec![];
      let mut now: Instant;
      let mut duration: Duration;
      for _ in 0..TEST_REPETITIONS {
        let puzzle = mock_puzzle(n, expected_complexity);
        now = Instant::now();
        let computed_complexity = classify_puzzle(&puzzle).expect("Failed to classify puzzle: {puzzle:?}");
        duration = now.elapsed();
        durations.push(duration);
        assert_eq!(computed_complexity, expected_complexity);
      }

      let average = durations.iter().sum::<Duration>() / durations.len() as u32;
      println!("Average time for ComplexityClass with n = {n} and c = {expected_complexity}: {}", format_duration(average));
    });

  });
}

#[test]
fn bench_all() {
    bench_test_suite().into_iter().for_each(|n| {
      let l = if n % 2 == 0 {(n + 1) * (n + 2) / 2} else {(n + 1) * (n + 1) / 2};
      let minimum_tiles = (n as f32/2.0).floor();
      let max_hole = l - minimum_tiles as usize;
      println!("n: {n} max_hole: {max_hole}\n\n");

      (1..=3).into_iter().map(|c| ComplexityClass::new(c)).for_each(|expected_complexity| {
        let log_factor = match expected_complexity.0 {
            1 => 1.0 / l as f32,
            2 => 4.0 / 7.0,
            3 => 6.0 / 7.0,
            _ => 0.0
        };
        let minimum_removals = (max_hole as f32 * log_factor.sqrt()).ceil() as usize;
        println!("expected_complexity: {expected_complexity} minimum_removals: {minimum_removals}");

        let durations: Rc<RefCell<Vec<Duration>>> = Rc::new(RefCell::new(vec![]));
        for _ in 0..TEST_REPETITIONS {
          let now: Instant = Instant::now();
          let puzzle = generate_puzzle(n, minimum_removals, false);
          solve_puzzle(&puzzle)
          .ok()
          .filter(|solution| validate_puzzle(&puzzle, solution).is_ok())
          .map_or_else(|| {
            durations.borrow_mut().push(now.elapsed());
          }, |_solution| {
            let computed_complexity = classify_puzzle(&puzzle).expect("Failed to classify puzzle: {puzzle:?}");
            let duration = now.elapsed();
            durations.borrow_mut().push(duration);
            assert_eq!(expected_complexity, computed_complexity);
          });
        }

        let computed_durations = durations.take();
        let average = computed_durations.iter().sum::<Duration>().checked_div(computed_durations.len() as u32);
        println!(
          "Average time for all the operations with n = {n} and c = {expected_complexity}: {}\n\n",
          average.map(format_duration).unwrap_or(format_duration(Duration::from_secs(0)))
        );
      });
    });
}
