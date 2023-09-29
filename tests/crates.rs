use reqwest::{blocking::Client, StatusCode, header::ACCEPT_CHARSET};
use serde_json::{json, Value};

pub mod common;

#[test]
fn test_create_crate() {
    let client = Client::new();
    
    // create rustacean that owns crate
    let rustacean = common::create_test_rustacean(&client);
    
    let response = client.post(format!("{}/crates", common::APP_HOST))
        .json(&json!({
            "rustacean_id": rustacean["id"],
            "code": "foo",
            "name": "Baz crate",
            "version": "0.1",
            "description": "The Crate Description"
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::CREATED);

    // check crate correctly created
    let a_crate: Value = response.json().unwrap();
    assert_eq!(a_crate, json!({
        "id": a_crate["id"],
        "code": "foo",
        "name": "Baz crate",
        "version": "0.1",
        "description": "The Crate Description",
        "rustacean_id": rustacean["id"],
        "created_at": a_crate["created_at"]
    }));

    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);
}

#[test]
fn test_view_crate() {
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client, &rustacean);

    let response = client.get(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // check crate correctly created
    let a_crate: Value = response.json().unwrap();
    assert_eq!(a_crate, json!({
        "id": a_crate["id"],
        "code": "foo",
        "name": "Baz crate",
        "version": "0.1",
        "description": "The Crate Description",
        "rustacean_id": rustacean["id"],
        "created_at": a_crate["created_at"]
    }));

    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);    

}

#[test]
fn test_update_crate() {
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client, &rustacean);

    let response = client.put(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .json(&json!({
            "code": "bla",
            "name": "Something else",
            "version": "0.3",
            "description": "The Crate Organization",
            "rustacean_id": rustacean["id"]
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // check crate correctly created
    let a_crate: Value = response.json().unwrap();
    assert_eq!(a_crate, json!({
        "id": a_crate["id"],
        "code": "bla",
        "name": "Something else",
        "version": "0.3",
        "description": "The Crate Organization",
        "rustacean_id": rustacean["id"],
        "created_at": a_crate["created_at"]
    }));

    // Test changing crate author
    let rustacean2 = common::create_test_rustacean(&client);
    let response = client.put(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .json(&json!({
            "code": "bla",
            "name": "Something else",
            "version": "0.3",
            "description": "The Crate Organization",
            "rustacean_id": rustacean2["id"]
        }))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let a_crate: Value = response.json().unwrap();
    assert_eq!(a_crate, json!({
        "id": a_crate["id"],
        "code": "bla",
        "name": "Something else",
        "version": "0.3",
        "description": "The Crate Organization",
        "rustacean_id": rustacean2["id"],
        "created_at": a_crate["created_at"]
    }));

    common::delete_test_crate(&client, a_crate);
    common::delete_test_rustacean(&client, rustacean);    

}

#[test]
fn test_delete_crate() {
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client, &rustacean);

    let response = client.delete(format!("{}/crates/{}", common::APP_HOST, a_crate["id"]))
        .send()
        .unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    common::delete_test_rustacean(&client, rustacean);    
}

#[test]
fn test_get_crates() {
    // Setup
    let client = Client::new();
    let rustacean = common::create_test_rustacean(&client);
    let a_crate = common::create_test_crate(&client, &rustacean);
    let b_crate = common::create_test_crate(&client, &rustacean);

    // Test
    let response = client.get(format!("{}/crates", common::APP_HOST)).send().unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json: Value = response.json().unwrap();
    assert!(json.as_array().unwrap().contains(&a_crate));
    assert!(json.as_array().unwrap().contains(&b_crate));

    // Cleanup
    common::delete_test_crate(&client, a_crate);
    common::delete_test_crate(&client, b_crate);
    common::delete_test_rustacean(&client, rustacean);
}
