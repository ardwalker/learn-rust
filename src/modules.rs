

mod submodules; // resolves to a rust file called submodules under ./modules

fn top_function() -> String {
    return "top".to_string();
}

mod my_outer_module {
    pub fn outer_function() -> String {
        return "outer".to_string();
    }

    pub mod my_inner_module {
        pub fn inner_function() -> String {
            return "inner".to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::modules::top_function;

    #[test]
    fn test_top_level_module() {
        assert_eq!(super::top_function(), "top");
        assert_eq!(top_function(), "top"); // needs use create::modules::
    }

    #[test]
    fn test_outer_module() {
        assert_eq!(super::my_outer_module::outer_function(), "outer");
    }

    #[test]
    fn test_inner_module() {
        assert_eq!(super::my_outer_module::my_inner_module::inner_function(), "inner");
    }

}