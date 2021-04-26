extern crate rand;
use rand::distributions::Distribution;

pub mod combinator;

/// Randomly select an item from `vector`. The item is removed from `vector`.
/// Returns `Some(the_selected_item)` if present or `None` if the vector is empty.
/// ## Examples
/// ```rust
/// use rust_helpers::sample_and_pop;
/// 
/// let mut x = Vec::<u32>::new();
/// let mut y: Vec<u32> = vec![3];
/// 
/// let mut rng = rand::thread_rng();
/// assert_eq!(sample_and_pop(&mut rng, &mut x), None);
/// assert_eq!(sample_and_pop(&mut rng, &mut y), Some(3));
/// assert_eq!(sample_and_pop(&mut rng, &mut y), None);
/// ```
pub fn sample_and_pop<T>(rng: &mut rand::rngs::ThreadRng, vector: &mut Vec<T>) -> Option<T>
{
    if vector.is_empty()
    {
        return None
    }
    Some(vector.swap_remove(
        rand::distributions::Uniform::from(0..vector.len()).sample(rng)
    ))
}


pub fn sample_and_consume<T>(rng: &mut rand::rngs::ThreadRng, mut vector: Vec<T>)-> Option<T>
{
    if vector.is_empty()
    {
        return None
    }
    Some(vector.swap_remove(
        rand::distributions::Uniform::from(0..vector.len()).sample(rng)
    ))
}

pub fn select<T>(source_vector: &Vec<T>, indices: &Vec<usize>) -> Vec<T>
    where T: Clone
{
    let mut out = Vec::<T>::with_capacity(indices.len());

    for index in indices.iter()
    {
        out.push(source_vector[*index].clone());
    }

    out
}

pub fn factorial_upper_bound(n: usize) -> usize 
{
    let n = n as f64;
    use std::f64::consts::E as e;
    use std::f64::consts::PI as pi;

    // Ramanujan order 1 / n^5 approximation
    let approx = (2.0 * pi * n).sqrt() * (n / e).powf(n) * e.powf( (1.0 / (12.0 * n)) * (1.0 - 1.0 / (30.0 * n * n) ) );

    return approx.ceil() as usize
}

pub fn generate_permutations(n: usize) -> Vec<Vec<usize>>
{
    let mut output = Vec::<Vec<usize>>::with_capacity(factorial_upper_bound(n));
    let mut identity = (0..n).collect();
    inner_generate_permutations(n, &mut identity, &mut output);

    output
}

fn inner_generate_permutations(k: usize, a: &mut Vec<usize>, output: &mut Vec<Vec<usize>>)
{
    if k == 1
    {
        output.push(a.clone());
        return;
    }

    inner_generate_permutations(k - 1, a, output);

    for i in 0..(k - 1)
    {
        if k % 2 == 0
        {
            a.swap(i, k - 1);
        }
        else
        {
            a.swap(0, k - 1);
        }

        inner_generate_permutations(k - 1, a, output);
    }

}

pub fn overwrite_file(name: String) -> std::io::BufWriter<std::fs::File>
{
    let opt = std::fs::OpenOptions::new()
    .write(true)
    .truncate(true)
    .create(true)
    .open(name)
    .unwrap();

    std::io::BufWriter::new(opt)
}

pub fn lattice(dimension: usize, depth: u32) -> Vec<Vec<u32>>
{
    let mut out = Vec::<Vec<u32>>::with_capacity((depth as usize).pow(dimension as u32));
    inner_lattice(dimension, depth, Vec::<u32>::with_capacity(dimension), &mut out);
    out
}

fn inner_lattice(dimension: usize, depth: u32, working_vector: Vec<u32>, out_set: &mut Vec<Vec<u32>>)
{
    if working_vector.len() == dimension
    {
        out_set.push(working_vector);
        return;
    }

    for i in 0..depth
    {
        let mut child_vector = working_vector.clone();
        child_vector.push(i);
        inner_lattice(dimension, depth, child_vector, out_set);
    }
}

pub struct ListExplorer<T>
    where T: Clone
{
    num_options: usize,
    options_list: Vec<T>,
    current_list: Vec<usize>,
    step: usize
}

impl<T> ListExplorer<T>
    where T: Clone
{
    pub fn new(options_list: Vec<T>, current_list: Vec<usize>, step: Option<usize>) -> ListExplorer<T>
    {
        let step = step.unwrap_or(1);
        assert!(step > 0, "A step of zero is just dumb!");
        ListExplorer
        {
            num_options: options_list.len(),
            options_list,
            current_list,
            step
        }
    }

    fn increment(&mut self, index: usize, amount: usize) -> usize
    {
        if index == self.current_list.len()
        {
            self.current_list.push(amount - 1);
        }
        else
        {
            self.current_list[index] += amount;
        }

        let multiple = self.current_list[index] / self.num_options;
        let modulus = self.current_list[index] - self.num_options * multiple;

        self.current_list[index] = modulus;

        multiple
    }

}

impl<T> Iterator for ListExplorer<T>
    where T: Clone
{
    type Item = Vec<T>;
    fn next(&mut self) -> Option<Self::Item>
    {
        let out = select(&self.options_list, &self.current_list);

        let mut carry = self.increment(0, self.step);

        let mut index_to_check = 0;

        while
            index_to_check < self.current_list.len() &&
            carry != 0
        {
            carry = self.increment(index_to_check + 1, carry);
            index_to_check += 1;
        }
       
        Some(out)
    }
}

#[macro_export]
macro_rules! intersperse_while {
    ( while $condition:expr => { $($while_stmt:stmt;)* },  intersperse => { $($intersperse_stmt:stmt;)*  } ) => {
        {
            let mut first = true;
            while $condition {
                if first
                {
                    first = false;
                }
                else
                {
                    $($intersperse_stmt)*
                }
                $($while_stmt)*
            }
        }
    };
}

#[macro_export]
macro_rules! intersperse_for {
    ( for $pattern:pat in $iterator:expr => { $($for_stmt:stmt;)* },  intersperse => { $($intersperse_stmt:stmt;)*  } ) => {
        {
            let mut first = true;
            for $pattern in $iterator {
                if first
                {
                    first = false;
                }
                else
                {
                    $($intersperse_stmt)*
                }
                $($for_stmt)*
            }
        }
    };
}

#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_list_explorer()
    {
        let mut count = 0;
        for i in ListExplorer::new( (0..16).collect::<Vec<u32>>(), vec![1], Some(3))
        {
            if count == 32
            {
                break;
            }
            println!("{:?}", i);
            count += 1;
            
        }
    }
    #[test]
    fn test_intersperse_while() {

        let mut i = 0;
        intersperse_while! (
            while i < 10 => {
                println!("{}", i);
                i += 1;
            },
            intersperse => {
                println!("moo");
            }
        )

    }

    #[test]
    fn test_intersperse_for() {


        intersperse_for! (
            for i in 0..10 => {
                print!("{}", i);


            },
            intersperse => {
                print!(",");
            }
        )

    }

}
