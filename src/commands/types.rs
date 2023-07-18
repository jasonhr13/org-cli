use std::fmt;

pub struct Instance {
    pub name: String,
    pub id: String,
    pub az: String,
    pub keyname: String,
    pub ip: String
}

impl fmt::Display for Instance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\tName: {}\n\tZone: {}\n\tID: {}\n\tIP: {}\n", self.name, self.az, self.id, self.ip)
    }
}

pub struct Instances(pub Vec<Instance>);
impl fmt::Display for Instances {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().fold(Ok(()), |result, instance| {
            result.and_then(|_| writeln!(f, "{}", instance))
        })
    }
}
