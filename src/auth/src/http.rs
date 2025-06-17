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
use crate::errors::{self, CredentialsError, is_retryable};
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
}

pub(crate) struct Builder {
    inner: reqwest::Client,
    endpoint: String,
    retry_policy: Option<Arc<dyn RetryPolicy>>,
    backoff_policy: Option<Arc<dyn BackoffPolicy>>,
    retry_throttler: SharedRetryThrottler,
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

    pub(crate) fn build(self) -> ReqwestClient {
        ReqwestClient {
            inner: self.inner,
            endpoint: self.endpoint,
            retry_policy: self.retry_policy,
            backoff_policy: self.backoff_policy,
            retry_throttler: self.retry_throttler,
        }
    }
}