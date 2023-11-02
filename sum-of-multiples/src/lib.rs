use std::collections::HashSet;

pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
  let mut all_multiples: HashSet<u32> = HashSet::new();
  for &factor in factors {
      if factor == 0 {
          continue;
      }
      let multiples = calculate_multiples(limit, factor);
      all_multiples.extend(multiples);
  }
  all_multiples.iter().sum()
}


fn calculate_multiples(limit: u32, factor: u32) -> Vec<u32> {
  let mut multiples: Vec<u32> = Vec::new();
  let mut count: u32 = factor;
  while count < limit {
      multiples.push(count);
      count += factor;
  }
  multiples
}
