pub trait Job<'chain> {
    fn next_job(&mut self, next_job: &'chain dyn Job<'chain>) -> &mut dyn Job<'chain>;
    fn process(&self, name: &'chain str);
    fn get_name(&self) -> Option<&'chain str>;
    fn get_title(&self) -> Option<&'chain str>;
}

pub struct Job1<'chain> {
    next_job: Option<&'chain dyn Job<'chain>>,
    name: &'chain str,
    title: &'chain str,
}

impl<'chain> Job1<'chain> {
    pub fn new(name: &'chain str, title: &'chain str) -> Job1<'chain> {
        Job1 {
            next_job: None,
            name,
            title,
        }
    }
}

impl<'chain> Job<'chain> for Job1<'chain> {
    fn next_job(&mut self, next_job: &'chain dyn Job<'chain>) -> &mut dyn Job<'chain> {
        self.next_job = Some(next_job);
        self
    }

    fn process(&self, title: &'chain str) {
        println!(
            "{} processing job : {}",
            self.get_name().unwrap(),
            self.get_title().unwrap()
        );

        if let Some(v) = &self.next_job {
            v.process(title);
        }
    }

    fn get_name(&self) -> Option<&'chain str> {
        Some(self.name)
    }

    fn get_title(&self) -> Option<&'chain str> {
        Some(self.title)
    }
}

pub struct Engine;

impl<'chain> Engine {
    pub fn run<T: Job<'chain>>(job: &T) {
        job.process(job.get_title().unwrap());
    }
}
