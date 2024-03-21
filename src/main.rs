use std::cmp::max;

use pyo3::prelude::*;

fn main() -> PyResult<()> {
    pyo3::prepare_freethreaded_python();

    let py_foo = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/diagram.py"));

    let from_python = Python::with_gil(|py| {
        PyModule::from_code(py, py_foo, "", "")?;

        let app: Py<PyAny> = PyModule::from_code(py, py_foo, "", "")?
            .getattr("draw_diagram")?
            .into();

        app.call0(py)

        // let fun: Py<PyAny> = PyModule::from_code(
        //     py,
        //     "def example(*args, **kwargs):
        //         if args != ():
        //             print('called with args', args)
        //         if kwargs != {}:
        //             print('called with kwargs', kwargs)
        //         if args == () and kwargs == {}:
        //             print('called with no arguments')",
        //     "",
        //     "",
        // )?
        // .getattr("example")?
        // .into();

        // // call object without any arguments
        // fun.call0(py)?;

        // // call object with PyTuple
        // let args = PyTuple::new(py, &[arg1, arg2, arg3]);
        // fun.call1(py, args)?;

        // // pass arguments as rust tuple
        // let args = (arg1, arg2, arg3);
        // fun.call1(py, args)?;
    });

    println!("py: {}", from_python?);
    Ok(())
}

fn max_subarray(nums: Vec<i32>) -> i32 {
    let mut best_sum = i32::MIN;
    let mut current_sum = 0;

    for x in nums {
        current_sum = std::cmp::max(x, current_sum + x);
        best_sum = max(best_sum, current_sum)
    }

    best_sum
}

#[cfg(test)]
mod array_tests {
    use super::max_subarray;

    #[test]
    fn test_max_subarray() {
        assert_eq!(max_subarray(vec![-1]), -1);
        assert_eq!(max_subarray(vec![-1, 0, -2]), 0);
    }
}
