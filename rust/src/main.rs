struct Env<'a> {
    parent: Option<&'a Env<'a>>,
}

impl<'a> Env<'a> {
    fn new(parent: Option<&'a Env<'a>>) -> Env<'a> {
        Env { parent }
    }
    fn depth(&self) -> usize {
        match self.parent {
            Some(parent) => parent.depth(),
            None => 1,
        }
    }
}

fn rec_env(depth: usize, env: &Env, case: usize) {
    if depth == 0 {
        return;
    }
    println!("rec_env: depth = {}, env.depth() = {}", depth, env.depth());
    if case == 1 {
        rec_env(depth - 1, &Env::new(Some(env)), case);
    } else {
        rec_env(depth - 2, &Env::new(Some(env)), case);
    }
}

fn main() {
    rec_env(3, &Env::new(None), 0);
    rec_env(3, &Env::new(None), 1);
}
