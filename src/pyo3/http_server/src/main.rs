use pyo3::ffi::c_str;
use pyo3::prelude::*;

fn main() -> PyResult<()> {
    let py_app = c_str!(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/python/app.py"
    )));
    let from_python = Python::with_gil(|py| -> PyResult<Py<PyAny>> {
        let app: PyResult<Bound<'_, PyModule>> =
            PyModule::from_code(py, py_app, c_str!(""), c_str!(""));
        let _class_parameters: Py<PyAny> = app.clone().getattr("Parameters")?.into();
        let def_hello: Py<PyAny> = app?.getattr("hello")?.into();
        def_hello.call1(py, ("Rust",))
    });

    println!("py: {}", from_python?);
    Ok(())
}
