#![allow(dead_code)]

mod common;
use crate::common::setup;
use rocket::http::Status;

// Define the test function
#[test]
fn test_index() {

    // Get a lock on the `Client` instance using the `setup` function
    let client = setup().lock().unwrap();

    // Send a GET request to the root route
    let response = client.get("/").dispatch();

    // Assert that the response status code is OK (200)
    assert_eq!(response.status(), Status::Ok);

    // Assert that the response body contains the expected text
    assert!(response.into_string().unwrap().contains("Hello from rust and mongoDB"));
}