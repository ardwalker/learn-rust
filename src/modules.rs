

mod submodules; // resolves to a rust file called submodules under ./modules

fn _top_function() -> String {
    return "top".to_string();
}

mod my_outer_module {
    pub fn _outer_function() -> String {
        return "outer".to_string();
    }

    pub mod my_inner_module {
        pub fn _inner_function() -> String {
            return "inner".to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::modules::_top_function;

    #[test]
    fn test_top_level_module() {
        assert_eq!(super::_top_function(), "top");
        assert_eq!(_top_function(), "top"); // needs use create::modules::
    }

    #[test]
    fn test_outer_module() {
        assert_eq!(super::my_outer_module::_outer_function(), "outer");
    }

    #[test]
    fn test_inner_module() {
        assert_eq!(super::my_outer_module::my_inner_module::_inner_function(), "inner");
    }

}