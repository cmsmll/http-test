#![allow(async_fn_in_trait)]

use derive_more::{Deref, Display};
use reqwest::{Response, StatusCode};
use serde_json::Value;

mod json_pretty;
pub use anyhow::*;
pub struct Resp(pub Option<Response>);

pub trait FromResp: Sized {
    async fn from_resp(resp: &mut Resp) -> anyhow::Result<Self>;
}

impl FromResp for Response {
    async fn from_resp(resp: &mut Resp) -> anyhow::Result<Self> {
        match resp.0.take() {
            Some(resp) => Ok(resp),
            None => Err(anyhow!("提取数据失败 提取数据失败 Response 已经被消耗")),
        }
    }
}

#[derive(Debug, Deref, Display)]
pub struct Json(pub Value);
impl Json {
    pub fn pretty(&self) -> String {
        let mut jp = json_pretty::Pretty::default();
        jp.format(&self.0.to_string())
    }
}
impl FromResp for Json {
    async fn from_resp(resp: &mut Resp) -> anyhow::Result<Self> {
        match resp.0.take() {
            Some(resp) => Ok(Self(resp.json().await?)),
            None => Err(anyhow!("提取数据失败 Response 已经被消耗")),
        }
    }
}

#[derive(Debug, Deref, Display)]
pub struct Text(pub String);
impl FromResp for Text {
    async fn from_resp(resp: &mut Resp) -> anyhow::Result<Self> {
        match resp.0.take() {
            Some(resp) => Ok(Self(resp.text().await?)),
            None => Err(anyhow!("提取数据失败 Response 已经被消耗")),
        }
    }
}

#[derive(Debug, Deref, Display)]
pub struct Status(pub StatusCode);
impl FromResp for Status {
    async fn from_resp(resp: &mut Resp) -> anyhow::Result<Self> {
        match &resp.0 {
            Some(resp) => Ok(Self(resp.status())),
            None => Err(anyhow!("提取数据失败 Response 已经被消耗")),
        }
    }
}
