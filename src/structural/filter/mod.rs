
#[derive(std::cmp::PartialEq, std::clone::Clone)]
pub struct Person {
    name: &'static str,
    gender: &'static str,
    marital_status: &'static str,
}

impl Person {
    pub fn new(name: &'static str, gender: &'static str, marital_status: &'static str) -> Self {
        Person { name, gender, marital_status }
    }

    pub fn get_name(&self) -> &'static str {
        self.name
    }

    pub fn get_gender(&self) -> &'static str {
        self.gender
    }

    pub fn get_marital_status(&self) -> &'static str {
        self.marital_status
    }
}

pub trait Criteria<'a> {
    fn meet_criteria(&self, persons: &'a Vec<Person>) -> Vec<Person>;

    fn meet_criteria_and(&self, _persons: Vec<Person>) -> Vec<Person> {
        Vec::new()
    }
}

pub struct CritariaMale;

impl<'a> Criteria<'a> for CritariaMale {
    fn meet_criteria(&self, persons: &'a Vec<Person>) -> Vec<Person> {
        let mut critaria: Vec<Person> = Vec::new();
        for person in persons.iter() {
            if let "Male" = person.get_gender() {
                critaria.push(person.to_owned());
            };
        }
        critaria
    }

    fn meet_criteria_and(&self, persons: Vec<Person>) -> Vec<Person> {
        let mut critaria: Vec<Person> = Vec::new();
        for person in persons.iter() {
            if let "Male" = person.get_gender() {
                critaria.push(person.clone());
            };
        }
        critaria
    }
}

pub struct CriteriaFemale;

impl<'a> Criteria<'a> for CriteriaFemale {
    fn meet_criteria(&self, persons: &'a Vec<Person>) -> Vec<Person> {
        let mut critaria: Vec<Person> = Vec::new();
        for person in persons.iter() {
            if let "Female" = person.get_gender() {
                critaria.push(person.clone());
            };
        }
        critaria
    }

    fn meet_criteria_and(&self, persons: Vec<Person>) -> Vec<Person> {
        let mut critaria: Vec<Person> = Vec::new();
        for person in persons.iter() {
            if let "Female" = person.get_gender() {
                critaria.push(person.clone());
            };
        }
        critaria
    }
}


pub struct CriteriaSingle;

impl<'a> Criteria<'a> for CriteriaSingle {
    fn meet_criteria(&self, persons: &'a Vec<Person>) -> Vec<Person> {
        let mut critaria: Vec<Person> = Vec::new();
        for person in persons.iter() {
            if let "Single" = person.get_marital_status() {
                critaria.push(person.clone());
            };
        }
        critaria
    }
}

pub struct AndCriteria<'a, T: Criteria<'a>, V: Criteria<'a>> {
    criteria: &'a T,
    other_criteria: &'a V,
}

impl<'a, T: Criteria<'a>, V: Criteria<'a>> AndCriteria<'a, T, V> {
    pub fn new(criteria: &'a T, other_criteria: &'a V) -> Self {
        AndCriteria { criteria, other_criteria }
    }
}

impl<'a, T: Criteria<'a>, V: Criteria<'a>> Criteria<'a> for AndCriteria<'a, T, V> {
    fn meet_criteria(&self, persons: &'a Vec<Person>) -> Vec<Person> {
        let first_criteria_persons = self.criteria.meet_criteria(persons);
        self.other_criteria.meet_criteria_and(first_criteria_persons)
    }
}


pub struct OrCriteria<'a, T: Criteria<'a>, V: Criteria<'a>> {
    criteria: &'a T,
    other_criteria: &'a V,
}

impl<'a, T: Criteria<'a>, V: Criteria<'a>> OrCriteria<'a, T, V> {
    pub fn new(criteria: &'a T, other_criteria: &'a V) -> Self {
        OrCriteria { criteria, other_criteria }
    }
}

impl<'a, T: Criteria<'a>, V: Criteria<'a>> Criteria<'a> for OrCriteria<'a, T, V> {
    fn meet_criteria(&self, persons: &'a Vec<Person>) -> Vec<Person> {
        let mut first_criteria_persons = self.criteria.meet_criteria(persons);
        let other_criteria_persons = self.other_criteria.meet_criteria(persons);
        for person in other_criteria_persons.into_iter() {
            if !first_criteria_persons.contains(&person) {
                first_criteria_persons.push(person)
            }
        }
        first_criteria_persons
    }
}



