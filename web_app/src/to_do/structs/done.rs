use super::super::enums::TaskStatus;
use super::base::Base;

pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(input_title: &str) -> Self {
        let base = Base {
            title: input_title.to_string(),
            status: TaskStatus::DONE,
        };
        return Done { super_struct: base };
    }
}

mod done_tests {
    use super::Done;
    use super::super::base::Base;
    use super::super::super::enums::TaskStatus;

    #[test]
    fn new() {
        let new_done_struct = Done::new("test_title");

        assert_eq!(new_done_struct.super_struct.title, String::from("test_title"));
        assert_eq!(new_done_struct.super_struct.status, TaskStatus::DONE);
    }
}
