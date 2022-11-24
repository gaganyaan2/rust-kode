extern crate yaml_rust;
use futures::{StreamExt, TryStreamExt};
use kube::api::{Api, ResourceExt, ListParams, PostParams, WatchEvent};
use kube::Client;
use k8s_openapi::api::core::v1::Namespace;
use std::fs;
use std::env;
use yaml_rust::YamlLoader;
use colored::Colorize;
use std::io::Write;
use std::process;

#[tokio::main]
pub async fn main() {
    let client = Client::try_default().await.unwrap();
    let namespace: Api<Namespace> = Api::all(client);
    let names: Vec<String>  = namespace.list(&ListParams::default())
    .await
    .unwrap()
    .items
    .iter()
    .map(|name| name.name().to_string())
    .collect();

    println!("namespaces :{:?}",names); // shows only ()
}