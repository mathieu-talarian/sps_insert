use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};
use std::f32::consts::PI;

const SPS_ID: &str = "q";
const TOKEN: &str = "ya29.";

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ValueRange {
    range: String,
    major_dimension: String,
    values: Vec<Vec<Value>>,
}

struct V(f32);
impl From<V> for Number {
    fn from(v: V) -> Self {
        Number::from_f64(v.0 as f64).unwrap()
    }
}
impl From<V> for Value {
    fn from(v: V) -> Self {
        Value::Number(v.into())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct M(Map<String, Value>);

pub async fn write_sps() -> Result<(), std::io::Error> {
    let mut body: Vec<Vec<Value>> = Vec::new();

    let mut map = Map::new();
    map.insert("a".into(), "b".into());
    map.insert("c".into(), "d".into());
    let m: M = M(map);

    body.push(vec!["1".into()]);
    body.push(vec!["2".into()]);
    body.push(vec!["3".into()]);
    body.push(vec![12.into()]);
    body.push(vec![V(15.99).into()]);
    body.push(vec![V(1f32 / 3f32).into()]);
    body.push(vec![V(PI).into()]);
    body.push(vec![Value::Null]);
    body.push(vec![true.into()]);

    let output = ValueRange {
        range: "api!A2:A".into(),
        major_dimension: "ROWS".into(),
        values: body,
    };
    let client = reqwest::Client::new();
    let res = client
        .post(format!(
            "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}:append?valueInputOption=USER_ENTERED",
            SPS_ID, "api!A2:A"
        ))
        .bearer_auth(TOKEN)
        .json(&output)
        .send()
        .await
        .unwrap();
    println!("{:?}", res);
    let json = res.json::<Value>().await.unwrap();

    println!("{:?}", json);

    Ok(())
}
