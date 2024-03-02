use http::{Request, Response};
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[tokio::test]
async fn test_1() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    let resp = reqwest::get("http://localhost:7878/sleep")
        .await?
        .text()
        .await?;
    println!("Response: {:#?}", resp);
    let duration = start.elapsed();
    println!("test_1 took {:?} seconds for 1 sleep request", duration);
    return Ok(());
}

#[tokio::test]
async fn test_2() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    let resp = reqwest::get("http://localhost:7878/sleep")
        .await?
        .text()
        .await?;
    println!("Response: {:#?}", resp);
    let duration = start.elapsed();
    println!("test_1 took {:?} seconds for 1 sleep request", duration);
    return Ok(());
}

#[tokio::test]
async fn test_3() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    let resp = reqwest::get("http://localhost:7878/sleep")
        .await?
        .text()
        .await?;
    println!("Response: {:#?}", resp);
    let duration = start.elapsed();
    println!("test_1 took {:?} seconds for 1 sleep request", duration);
    return Ok(());
}

#[tokio::test]
async fn test_4() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    let resp = reqwest::get("http://localhost:7878/sleep")
        .await?
        .text()
        .await?;
    println!("Response: {:#?}", resp);
    let duration = start.elapsed();
    println!("test_1 took {:?} seconds for 1 sleep request", duration);
    return Ok(());
}

#[tokio::test]
async fn test_5() -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();

    let resp = reqwest::get("http://localhost:7878/sleep")
        .await?
        .text()
        .await?;
    println!("Response: {:#?}", resp);
    let duration = start.elapsed();
    println!("test_1 took {:?} seconds for 1 sleep request", duration);
    return Ok(());
}
