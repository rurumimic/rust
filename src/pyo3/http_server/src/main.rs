use pyo3::ffi::c_str;
use pyo3::prelude::*;

fn main() -> PyResult<()> {
    let py_app = c_str!(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/python/app.py"
    )));

    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let app: PyResult<Bound<'_, PyModule>> =
            PyModule::from_code(py, py_app, c_str!("app.py"), c_str!("parameters"));

        let class_parameters = app?.getattr("Parameters")?;

        let args = ("Rust", 3);
        let instance_parameters = class_parameters.call1(args)?;

        let name = instance_parameters.getattr("name")?;
        let count = instance_parameters.getattr("count")?;

        println!("name: {}", name);
        println!("count: {}", count);

        Ok(instance_parameters.into())
    });

    println!("on rust: {}", from_python.as_ref().unwrap());
    let instance_parameters = from_python.as_ref().unwrap();

    let _ = Python::with_gil(|py| -> PyResult<()> {
        let result = instance_parameters
            .call_method0(py, "__str__")?
            .extract::<String>(py)?;

        println!("on python: {}", result);

        Ok(())
    });

    Ok(())
}
