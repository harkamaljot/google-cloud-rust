// Copyright 2025 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::Result as CredentialResult;
use crate::errors::{self, CredentialsError};
use gax::Result;
use gax::backoff_policy::BackoffPolicy;
use gax::error::Error as GaxError;
use gax::exponential_backoff::ExponentialBackoff;
use gax::response::{Parts, Response};
use gax::retry_policy::RetryPolicy;
use gax::retry_throttler::AdaptiveThrottler;
use gax::retry_throttler::SharedRetryThrottler;
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug)]
pub(crate) struct ReqwestClient {
    inner: reqwest::Client,
    endpoint: String,
    retry_policy: Option<Arc<dyn RetryPolicy>>,
    backoff_policy: Option<Arc<dyn BackoffPolicy>>,
    retry_throttler: SharedRetryThrottler,
    error_message: String,
}

pub(crate) struct Builder {
    inner: reqwest::Client,
    endpoint: String,
    retry_policy: Option<Arc<dyn RetryPolicy>>,
    backoff_policy: Option<Arc<dyn BackoffPolicy>>,
    retry_throttler: SharedRetryThrottler,
    error_message: String,
}

impl Builder {
    pub(crate) fn new(endpoint: String) -> Self {
        let inner = reqwest::Client::new();
        Self {
            inner,
            endpoint,
            retry_policy: None,
            backoff_policy: None,
            retry_throttler: Arc::new(Mutex::new(AdaptiveThrottler::default())),
            error_message: None,
        }
    }

    pub(crate) fn with_retry_policy(mut self, retry_policy: Arc<dyn RetryPolicy>) -> Self {
        self.retry_policy = Some(retry_policy);
        self
    }

    pub(crate) fn with_backoff_policy(mut self, backoff_policy: Arc<dyn BackoffPolicy>) -> Self {
        self.backoff_policy = Some(backoff_policy);
        self
    }

    pub(crate) fn with_retry_throttler(mut self, retry_throttler: SharedRetryThrottler) -> Self {
        self.retry_throttler = retry_throttler;
        self
    }

    pub(crate) fn with_error_message(mut self, error_message: String) -> Self {
        self.error_message = error_message;
        self
    }

    pub(crate) fn build(self) -> ReqwestClient {
        ReqwestClient {
            inner: self.inner,
            endpoint: self.endpoint,
            retry_policy: self.retry_policy,
            backoff_policy: self.backoff_policy,
            retry_throttler: self.retry_throttler,
            error_message: self.error_message,
        }
    }
}

impl ReqwestClient {
    pub fn prepare_request(
        &self,
        method: reqwest::Method,
        path: String,
    ) -> reqwest::RequestBuilder {
        self.inner
            .request(method, format!("{}{path}", &self.endpoint))
    }

    pub async fn execute<I: serde::ser::Serialize, O: serde::de::DeserializeOwned + Default>(
        &self,
        mut builder: reqwest::RequestBuilder,
        body: Option<I>,
    ) -> CredentialResult<O> {
        if let Some(body) = body {
            builder = builder.json(&body);
        }

        match self.retry_policy.clone() {
            None => self.request_attempt::<O>(builder, None).await,
            Some(policy) => self.retry_loop::<O>(builder, policy).await,
        }
    }

    async fn retry_loop<O: serde::de::DeserializeOwned + Default>(
        &self,
        builder: reqwest::RequestBuilder,
        retry_policy: Arc<dyn RetryPolicy>,
    ) -> CredentialResult<O> {
        let throttler = self.retry_throttler.clone();
        let backoff = self.get_backoff_policy();
        let this = self.clone();
        let inner = async move |d| {
            let builder = builder
                .try_clone()
                .expect("client libraries only create builders where `try_clone()` succeeds");
            this.request_attempt(builder, d).await
        };
        let sleep = async |d| tokio::time::sleep(d).await;
        gax::retry_loop_internal::retry_loop(inner, sleep, true, throttler, retry_policy, backoff)
            .await
    }

    async fn request_attempt<O: serde::de::DeserializeOwned + Default>(
        &self,
        mut builder: reqwest::RequestBuilder,
        remaining_time: Option<std::time::Duration>,
    ) -> CredentialResult<O> {
        if let Some(remaining_time) = remaining_time {
            builder = builder.timeout(remaining_time);
        }

        let response = builder.send().await.map_err(|e| crate::errors::from_http_error(e, &self.error_message))?;

        if !response.status().is_success() {
            let err = crate::errors::from_http_response(response, &self.error_message).await;
            return Err(err);
        }
        response.json::<O>().await.map_err(|e| {
            // Decoding errors are not transient. Typically they indicate a badly
            // configured MDS endpoint, or DNS redirecting the request to a random
            // server, e.g., ISPs that redirect unknown services to HTTP.
            CredentialsError::from_source(!e.is_decode(), e)
        })
    }

    pub(crate) fn get_backoff_policy(&self) -> Arc<dyn BackoffPolicy> {
        self.backoff_policy
            .clone()
            .unwrap_or_else(|| Arc::new(ExponentialBackoff::default()))
    }
}

#[derive(serde::Serialize, Default)]
pub(crate) struct NoBody {}
