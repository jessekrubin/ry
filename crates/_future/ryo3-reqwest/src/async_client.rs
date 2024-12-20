use pyo3::prelude::*;

use crate::pyo3_bytes::Pyo3Bytes;
use bytes::Bytes;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use reqwest::StatusCode;
use std::borrow::Borrow;

#[pyclass]
#[pyo3(name = "AsyncClient")]
#[derive(Debug, Clone)]
pub struct RyAsyncClient(reqwest::Client);
#[pyclass]
#[pyo3(name = "AsyncResponse")]
#[derive(Debug)]
pub struct RyAsyncResponse {
    // Store the response in an Option so we can take ownership later.
    status_code: StatusCode,
    headers: reqwest::header::HeaderMap,
    // cookies: reqwest::cookie::CookieJar,
    // version: Option<reqwest::Version>,
    url: reqwest::Url,

    body: Option<Bytes>,

    res: Option<reqwest::Response>,
}
impl RyAsyncResponse {
    async fn read_body_async(&mut self) -> Result<(), PyErr> {
        if self.body.is_none() {
            let res = self
                .res
                .take()
                .ok_or_else(|| PyValueError::new_err("Response already consumed"))?;
            let b = res
                .bytes()
                .await
                .map_err(|e| PyValueError::new_err(format!("{e}")))?;
            self.body = Some(b);
        }
        Ok(())
    }
    // async fn read_body(&mut self) ->  Result<(), String>
    // {
    //     match self.body.as_ref() {
    //         Some(b) => Ok(()),
    //         None => {
    //             let res = self
    //                 .res
    //                 .take()
    //                 .ok_or_else(|| PyValueError::new_err("Response already consumed"))?;
    //
    //
    //             let b = res
    //                 .bytes()
    //                 .await;
    //             // .map_err(|e| format!("{e}"))?;
    //
    //             match b {
    //                 Ok(b) => {
    //                     self.body = Some(b);
    //                     Ok(())
    //                 },
    //                 Err(e) => {
    //                     Err(format!("{e}"))
    //                 }
    //             }
    //
    //             // println!("b: {:?}", b);
    //             // self.body = Some(b);
    //             // Ok(())
    //             // Ok(&*b)
    //         }
    //     }
    // }
}

#[pymethods]
impl RyAsyncResponse {
    #[getter]
    fn status_code(&self) -> PyResult<u16> {
        Ok(self.status_code.as_u16())
    }
    // async fn bytes<'py>(mut slf: PyRefMut<'py, Self>) -> PyResult<Bound<'py, PyAny>> {

    // #[pyo3(name = "bytes")]
    // fn bytes_coroutine(&mut self, py: Python) -> PyResult<Py<PyAny>> {
    //     let mut this = self.clone(); // if needed, ensure clonable or handle differently
    //     pyo3_async_runtimes::tokio::future_into_py(py, async move {
    //         this.read_body().await.map_err(|e| PyValueError::new_err(e))?;
    //         let b = this.body.as_ref().unwrap();
    //         Python::with_gil(|py| Ok(PyBytes::new(py, &b[..]).into_py(py)))
    //     })
    // }

    fn bytes<'py>(&'py mut self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
        let get_result = self
            .res
            // .lock()
            // .unwrap()
            .take()
            .ok_or(PyValueError::new_err("Result has already been disposed."))?;
        let r = pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let bytes = get_result
                .bytes()
                .await
                .map_err(|e| PyValueError::new_err(format!("{e}")))?;
            println!("bytes: {:?}", bytes);
            Ok(Pyo3Bytes::new(bytes))
            // let pybytes = PyBytes::new(py, &bytes);
            // Ok(pybytes.into_any())

            // Ok(PyBytes::new(py, &bytes).into())
        });
        r
    }
    // #[pyo3(name = "bytes")]
    // fn bytes_py(&mut self, py: Python<'_>) -> PyResult <Bound<'_, PyAny>>
    // {
    //     Return a coroutine that will await reading the body if needed, then return the bytes.
    // let this = self as *mut RyAsyncResponse; // Raw pointer for closure capture
    // let fut = async move {
    //     Safety: We know `this`/**/ lives until future completes because Python awaits immediately,
    //     but it's safer to clone necessary data up front in more complex cases.
    // let this = unsafe { &mut *this };
    // this.read_body_async().await?;
    // let b = this.body.as_ref().unwrap();
    // Python::with_gil(|py| Ok(PyBytes::new(py, b).into_py(py)))
    // };
    // pyo3_async_runtimes::tokio::future_into_py(py, fut)
    // }
    // async fn bytes(
    //     mut slf: PyRefMut<'_, Self>,
    // ) -> PyResult<
    //     // Bound< PyAny>
    //     ()
    // > {
    //     let future = slf.read_body();
    //     let rt = pyo3_async_runtimes::tokio::get_runtime();
    //     let a:  Result<(), String> = rt.block_on(
    //         async {
    //             future.await?;
    //             Ok(())
    //         }
    //     ).map_err(|e| PyValueError::new_err(format!("{e}")))?;
    //     // let a = pyo3_async_runtimes::tokio::future_into_py(slf.py(),
    //     //     async move {
    //     //         future.await?;
    //     //         Ok(())
    //     //     }
    //     // );
    //
    //
    //     // let b = slf.body.as_ref().unwrap();
    //     // Ok(PyBytes::new(slf.py(), &b))
    //
    //     let string = "hello".to_string();
    //     println!("string: {:?}", string);
    //     // let pystring = PyString::new(slf.py(), &string);
    //     // let pyany = pystring.into_any();
    //     Ok(())
    //
    //
    //     // Ok(string.into(slf.py()))
    //
    //     // match slf.body.as_ref() {
    //     //     Some(b) => Ok(b.to_vec()),
    //     //     None => {
    //     //         // Take ownership of the response, leaving None in place.
    //     //         let res = slf
    //     //             .res
    //     //             .take()
    //     //             .ok_or_else(|| PyValueError::new_err("Response already consumed"))?;
    //     //
    //     //         // Now we have full ownership of res, so we can call text() without error.
    //     //         let b = res
    //     //             .bytes()
    //     //             .map_err(|e| PyValueError::new_err(format!("{e}")))?;
    //     //         // return the b
    //     //         Ok(b.to_vec())
    //     //     }
    //     // }
    // }
    //
    // fn text(mut slf: PyRefMut<'_, Self>) -> PyResult<String> {
    //     slf.read_body().await?;
    //     let b = slf.body.as_ref().unwrap();
    //
    //     let s = String::from_utf8_lossy(b);
    //     Ok(s.to_string())
    // }

    fn __str__(&self) -> String {
        format!("Response: {}", self.status_code)
    }

    fn __repr__(&self) -> String {
        format!("Response: {}", self.status_code)
    }

    // ) -> PyResult<Bound<'py, PyAny>> {

    // async fn json<'py>(mut slf: PyRefMut<'py, Self>) -> PyResult<Bound<'py, PyAny>> {
    //     slf.read_body().await?;
    //
    //     let parse_builder = jiter::PythonParse {
    //         allow_inf_nan: true,
    //         cache_mode: ::jiter::StringCacheMode::All,
    //         partial_mode: ::jiter::PartialMode::Off,
    //         catch_duplicate_keys: false,
    //         float_mode: ::jiter::FloatMode::Float,
    //         // cache_mode = StringCacheMode::All,
    //         // partial_mode = PartialMode::Off,
    //         // catch_duplicate_keys = false,
    //         // float_mode = FloatMode::Float
    //     };
    //     let b = slf.body.as_ref().unwrap();
    //     parse_builder
    //         .python_parse(slf.py(), b)
    //         .map_err(|e| jiter::map_json_error(b, &e))
    // }
}

#[pymethods]
impl RyAsyncClient {
    #[new]
    fn new() -> Self {
        Self(reqwest::Client::new())
    }

    // self.request(Method::GET, url)
    fn get<'py>(&'py mut self, py: Python<'py>, url: String) -> PyResult<Bound<'py, PyAny>> {
        // async fn get(&self, py: Python<'_>, url: String) -> PyResult<Bound<PyAny>> {
        // fn get(&self, url: String) -> PyResult<RyAsyncResponse> {

        // let rt = pyo3_async_runtimes::tokio::get_runtime();
        let response_future = self.0.get(&url).send();

        // .await
        // .map_err(|e| PyValueError::new_err(format!("{e}")))?;
        let a = pyo3_async_runtimes::tokio::future_into_py(py, async move {
            let response = response_future
                .await
                .map_err(|e| PyValueError::new_err(format!("{e}")))?;
            let r = RyAsyncResponse {
                status_code: response.status(),
                headers: response.headers().clone(),
                url: response.url().clone(),
                body: None,
                res: Some(response),
            };
            Ok(r)
        });

        a
        // rt.spawn(async {
        //     let response = self
        //         .0
        //         .get(&url)
        //         .send()
        //         .await
        //         .map_err(|e| PyValueError::new_err(format!("{e}")))?;
        //     Ok(RyAsyncResponse {
        //         status_code: response.status(),
        //         headers: response.headers().clone(),
        //         url: response.url().clone(),
        //         body: None,
        //         res: Some(response),
        //     })
        // })
        // let response = self
        //     .0
        //     .get(&url)
        //     .send()
        //     .await
        //     .map_err(|e| PyValueError::new_err(format!("{e}")))?;
        //
        // let url = response.url().clone();
        // let headers = response.headers().clone();
        // let status_code = response.status();
        //
        // Ok(RyAsyncResponse {
        //     status_code,
        //     headers,
        //     url,
        //     body: None,
        //
        //     res: Some(response),
        // })
    }
}

// #[pyclass]
// #[pyo3(name = "Client")]
// #[derive(Debug, Clone)]
// pub struct RysyncClient(reqwest::blocking::Client);

// #[pyclass]
// #[pyo3(name = "AsyncResponse")]
// #[derive(Debug)]
// pub struct RyAsyncResponse {
//     res: Arc<reqwest::Response>,
// }

// #[pymethods]
// impl RyAsyncResponse {
//     #[getter]
//     fn status_code(&self) -> u16 {
//         self.res.status().as_u16()
//     }

//     // async fn text<'py>(&mut self, py: Py<Self>) -> PyResult<Bound<'py, PyAny>> {
//     async fn text<'py>(&mut self, py: Py<Self>) -> PyResult<String> {
//         // let res = self.res.clone();
//         // let a = res.as_ref().text().await;
//         // let b = a.unwrap();

//         let string = "hello".to_string();
//         Ok(string)

//         // let rt = pyo3_async_runtimes::tokio::get_runtime();
//         // let res = self.res.clone();
//         // let r = rt.block_on(async move {
//         //     let a = res
//         //         .text()
//         //         .await
//         //         .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
//         //     Ok(a.into_py_any(py))
//         // });

//         // let a = self
//         //     .0
//         //     .text()
//         //     .await
//         //     .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
//         // Ok(a)

//         // let py = py.clone_ref(py);
//         // let pystring =PyString::new(py.clone_ref(py), &a.unwrap());
//         // let an = pystring.into_py_any(py);
//         // an

//         // pyo3_async_runtimes::tokio::future_into_py(py, async {
//         //     let a = self
//         //         .0
//         //         .text()
//         //         .await
//         //         .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
//         //     Ok(a)

//         // tokio::time::sleep(Duration::from_secs(secs)).await;
//         // Ok(())
//         // })
//     }
// }

// #[pymethods]
// impl RyAsyncClient {
//     #[new]
//     fn new() -> Self {
//         Self(reqwest::Client::new())
//     }

//     // fn __aenter__(&self) -> PyResult<RyAsyncClient> {
//     //     Ok(self.clone())
//     // }
//     fn __enter__<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyAny>> {
//         // fn __enter__<'py>(&self, py: Python<'py>) -> PyResult<Self> {
//         // Ok(self)
//         let rval = self.clone().into_bound_py_any(py);
//         rval

//         // Ok(rval.into_bound_py_any(py))
//         // let returnval = self.clone().into_bound_py_any(py);
//         // returnval
//         // Ok(returnval)
//     }

//     #[pyo3(signature = (* args))]
//     pub fn __exit__(&self, args: &Bound<'_, PyAny>) -> PyResult<()> {
//         // let tile = parse_tile_arg(args)?;
//         // Ok(self.tformatter.fmt(&tile.xyz))
//         Ok(())
//     }

//     async fn get(&self, url: String) -> PyResult<RyAsyncResponse> {
//         let rt = pyo3_async_runtimes::tokio::get_runtime();
//         let r =
//             rt.block_on(async {
//                 let response =
//                     self.0.get(&url).send().await.map_err(|e| {
//                         PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}"))
//                     })?;
//                 Ok(RyAsyncResponse {
//                     res: Arc::new(response),
//                 })
//             });
//         r

//         // pyo3_async_runtimes::tokio::future_into_py(py, async move {
//         // let response = self
//         //     .0
//         //     .get(url)
//         //     .send()
//         //     .await
//         //     .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
//         // Ok(RyAsyncResponse(response))
//         // pyo3_async_runtimes::tokio::future_into_py(py, async move {
//         //     let response = self
//         //         .0
//         //         .get(url)
//         //         .send()
//         //         .await
//         //         .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
//         //     Ok(RyAsyncResponse(response))
//         // })
//     }

//     // fn get<'py>(&'py self, py: Python<'py>, url: &str) -> PyResult<Bound<'py, PyAny>> {
//     //     // fn get(&self, py: Python, url: &str) -> PyResult<Bound<PyAny>> {
//     //     let urlcopy = url.to_string();
//     //
//     //     let c = self.clone();
//     //     pyo3_async_runtimes::tokio::future_into_py(py, async move {
//     //         let response =
//     //             c.0.get(urlcopy)
//     //                 .send()
//     //                 .await
//     //                 .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
//     //         let pyres = RyAsyncResponse(response);
//     //         // let restext = response
//     //         //     .text()
//     //         //     .await
//     //         //     .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
//     //
//     //         // print the response
//     //         // println!("{:?}", restext);
//     //
//     //         // let response2 = self
//     //         //     .0
//     //         //     .get(urlcopy)
//     //         //     .send()
//     //         //     .await
//     //         //     .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
//     //
//     //         // let pyresponse = RyAsyncResponse(response);
//     //
//     //         let t = pyres.into_py_any(py)?;
//     //         // .into_bound_py_any(py)?;
//     //         Ok(())
//     //     })
//     //     // let response = self.0.get(url).send()?;
//     //     // Ok(response)
//     // }

//     // fn __aenter__(&self, py: Python) -> PyResult<Bound<PyAny>> {
//     // let returnval = self.clone().into_bound_py_any(py);
//     // returnval
//     // Ok(returnval)
//     // pyo3_async_runtimes::tokio::future_into_py(py, async move {
//     //     let returnval = self.clone().into_bound_py_any(py);
//     //     returnval
//     // let returnval = self.clone().into_bound_py_any(py);
//     // Ok(returnval)
//     // })
//     // }

//     // fn __aexit__(&self) -> PyResult<()> {
//     //     Ok(())
//     // }
// }

// #[pyclass]
// #[pyo3(name = "Response")]
// #[derive(Debug)]
// pub struct RyResponse {
//     res: reqwest::blocking::Response,
// }

// impl RyResponse {

//     fn get_response(&self) -> reqwest::blocking::Response {
//         let a =  *self;
//         // self.to_owned();
//         a.res
//     }
// }

// #[pymethods]
// impl RyResponse {
//     #[getter]
//     fn status_code(&self) -> u16 {
//         self.res.status().as_u16()
//     }

//     fn text(&self) -> PyResult<String> {

//         let r = self.get_response();

//         // let a = {
//         //     let a = self.res.text();
//         //     a
//         // };
//         // let thingy = a.unwrap();
//         // .text().map_err(
//         // |e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")),
//         // )?;

//         // .res
//         // .text()
//         // .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
//         Ok(
//             "hello".to_string()
//         )
//     }
// }

// #[pymethods]
// impl RysyncClient {
//     #[new]
//     fn new() -> Self {
//         Self(reqwest::blocking::Client::new())
//     }

//     fn get(&self, url: String) -> PyResult<reqwest::blocking::Response> {
//         let response = self
//             .0
//             .get(&url)
//             .send()
//             .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
//         Ok(response)
//     }
// }

// // #[pyclass]
// // #[pyo3(name = "Client")]
// // #[derive(Debug, Clone)]
// // pub struct RyClient(reqwest::blocking::Client);

// pub fn pymod_add(m: &Bound<'_, PyModule>) -> PyResult<()> {
//     m.add_class::<RyAsyncClient>()?;
//     // m.add_class::<RyClient>()?;
//     // m.add_function(wrap_pyfunction!(self::which, m)?)?;
//     Ok(())
// }
// #[pyclass]
// #[pyo3(name = "Response")]
// #[derive(Debug)]
// pub struct RyResponse {
//     res: reqwest::blocking::Response,
// }
//
// #[pymethods]
// impl RyResponse {
//     #[getter]
//     fn status_code(&self) -> u16 {
//         self.res.status().as_u16()
//     }
//     fn text(slf: PyRefMut<'_, Self>) -> PyResult<String> {
//         let a = slf.res.text().map_err(|e| PyValueError::new_err(format!("{e}")));
//
//         Ok(
//             "hello".to_string()
//         )
//
//     }
//
// }
