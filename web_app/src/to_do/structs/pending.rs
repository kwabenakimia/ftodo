use super::super::enums::TaskStatus;
use super::base::Base;

pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::PENDING,
        };
        return Pending { super_struct: base };
    }
}

#[cfg(test)]
mod pending_tests {
    use super::Pending;
    use super::super::base::Base;
    use super::super::super::enums::TaskStatus;

    #[test]
    fn new() {
        let new_pending_struct = Pending::new("test_title");

        assert_eq!(new_pending_struct.super_struct.title, String::from("test_title"));
        assert_eq!(new_pending_struct.super_struct.status, TaskStatus::PENDING);
    }
}
