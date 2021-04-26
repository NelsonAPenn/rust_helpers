use std::collections::HashMap;

pub struct Combinator
{
    nk_lookup_table: HashMap< (usize, usize), Vec<Vec<usize>> >,
    all_lookup_table: HashMap< usize, Vec<Vec<usize>> > 
}
impl Combinator
{
    pub fn new() -> Combinator
    {
        Combinator
        {
            nk_lookup_table: HashMap::< (usize, usize), Vec<Vec<usize>> >::new(),
            all_lookup_table: HashMap::< usize, Vec<Vec<usize>> >::new()
        }
    }
    pub fn all_combinations(&mut self, n: usize) -> Vec::<Vec<usize>>
    {
        if let Some(combs) = self.all_lookup_table.get(&n)
        {
            return combs.clone();
        }

        let base: usize = 2;

        let mut out_combs = Vec::<Vec<usize>>::with_capacity(base.pow( n as u32 ));
        for k in 0..=n
        {
            for comb in self.combinations(n, k).into_iter()
            {
                out_combs.push(comb);
            }
        }
        self.all_lookup_table.insert(n, out_combs.clone());
        return out_combs;
    }

    pub fn combinations(&mut self, n: usize, k: usize) -> Vec<Vec<usize>>
    {
        if k > n
        {
            return vec![];
        }
        if let Some(combs) = self.nk_lookup_table.get( &(n, k) )
        {
            return combs.clone();
        }
        if k == 0
        {
            // 1 empty combination
            self.nk_lookup_table.insert( (n, k), vec![ vec![] ] );
            return vec![ vec![] ];
        }
        if k == n
        {
            let out_combs: Vec<Vec<usize>> = vec![ (0..n).collect() ];
            self.nk_lookup_table.insert( (n, k), out_combs.clone());
            return out_combs;
        }

        let mut out_combs = self.combinations(n - 1, k).clone(); // all combinations from the right

        for mut comb in self.combinations(n - 1, k - 1).into_iter()
        {
            comb.push(n - 1); // insert last node 
            out_combs.push(comb);
        }

        self.nk_lookup_table.insert( (n, k), out_combs.clone());
        out_combs
    }
}

#[cfg(test)]
mod tests
{
    use super::*;
    #[test]
    fn test_combinations()
    {
        let mut combinator = Combinator::new();
        let combinations = combinator.combinations(3, 1);
        assert_eq!(combinations, vec![vec![0], vec![1], vec![2]]);

        let combinations = combinator.combinations(3, 2);
        assert_eq!(combinations, vec![
            vec![0, 1],
            vec![0, 2],
            vec![1, 2]
        ]);

        let combinations = combinator.all_combinations(3);
        assert_eq!(combinations, vec![
            vec![],
            vec![0],
            vec![1],
            vec![2],
            vec![0, 1],
            vec![0, 2],
            vec![1, 2],
            vec![0, 1, 2]
        ]);



    }
}