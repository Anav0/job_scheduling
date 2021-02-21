#![allow(dead_code)]

pub struct JobSolver {
    pub machines: Vec<Vec<usize>>, //how many jobs of given type given machine can handle.
    pub processing_order: Vec<Vec<usize>>, //order in which given machine need to solve tasks.
}

impl JobSolver {
    fn new(machines: Vec<Vec<usize>>) -> Self {
        let number_of_machines = machines.len();
        JobSolver {
            machines,
            processing_order: vec![vec![]; number_of_machines],
        }
    }

    fn panic_if_invalid(&self, p: Vec<usize>, d: Vec<usize>, r: Vec<usize>, s: Vec<Vec<usize>>) {
        if p.len() != d.len() || p.len() != r.len() || d.len() != r.len() {
            panic!("'p', 'd', 'r' should be the same length");
        }
        for i in 0..s.len() {
            if s[i].len() != p.len() {
                panic!(format!("'p'needs to be a {0}x{0} matrix", p.len()));
            }
        }
        for i in 0..self.machines.len() {
            if self.machines[i].len() != p.len() {
                panic!(format!(
                    "All rows of machgines matrix need to be of length: '{0}'",
                    p.len()
                ));
            }
        }
        for i in 0..p.len() {
            if r[i] > d[i] {
                panic!("Availability of given task cannot be greater than its due date");
            }
        }
    }

    fn cmax(&self, p: &Vec<usize>, r: &Vec<usize>, s: &Vec<&Vec<usize>>) -> usize {
        let mut greatest_time = 0;
        for order in &self.processing_order {
            let mut total_time: usize = 0;
            let mut prev_job_index: Option<usize> = None;
            for job_index in order {
                total_time += p[*job_index];
                match prev_job_index {
                    Some(x) => total_time += s[*job_index][x],
                    None => total_time += r[*job_index],
                }
                prev_job_index = Some(*job_index);
            }
            if total_time > greatest_time {
                greatest_time = total_time;
            }
        }
        greatest_time
    }

    fn solve(&self, p: Vec<usize>, d: Vec<usize>, r: Vec<usize>, s: Vec<Vec<usize>>) {
        self.panic_if_invalid(p, d, r, s);
    }
}

fn main() {
    let p = vec![20, 30, 45]; //how much it take to finish given job. (in minutes)
    let d = vec![60 * 4, 60 * 4, 60 * 4]; //due date
    let r = vec![5, 10, 15]; //job would be available after x from start date.

    let mut s = vec![vec![0; p.len()]; p.len()]; //time delay neccessary between jobs.
    s[0][1] = 5;

    let solver = JobSolver::new(vec![vec![1, 1, 0], vec![2, 2, 1]]); //First machne can process one of job 1 and 2 but cannot process job 3

    solver.solve(p, d, r, s);
}
