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
//
// Code generated by sidekick. DO NOT EDIT.
#![allow(rustdoc::redundant_explicit_links)]
#![allow(rustdoc::broken_intra_doc_links)]

use crate::Result;
use std::sync::Arc;

/// Implements a client for the Google Cloud Memorystore for Redis API.
///
/// # Service Description
///
/// Configures and manages Cloud Memorystore for Redis instances
///
/// Google Cloud Memorystore for Redis v1
///
/// The `redis.googleapis.com` service implements the Google Cloud Memorystore
/// for Redis API and defines the following resource model for managing Redis
/// instances:
///
/// * The service works with a collection of cloud projects, named: `/projects/*`
/// * Each project has a collection of available locations, named: `/locations/*`
/// * Each location has a collection of Redis instances, named: `/instances/*`
/// * As such, Redis instances are resources of the form:
///   `/projects/{project_id}/locations/{location_id}/instances/{instance_id}`
///
/// Note that location_id must be referring to a GCP `region`; for example:
///
/// * `projects/redpepper-1290/locations/us-central1/instances/my-redis`
///
/// # Configuration
///
/// `CloudRedis` has various configuration parameters, the defaults should
/// work with most applications.
///
/// # Pooling and Cloning
///
/// `CloudRedis` holds a connection pool internally, it is advised to
/// create one and the reuse it.  You do not need to wrap `CloudRedis` in
/// an [Rc](std::rc::Rc) or [Arc] to reuse it, because it already uses an `Arc`
/// internally.
#[derive(Clone, Debug)]
pub struct CloudRedis {
    inner: Arc<dyn super::stubs::dynamic::CloudRedis>,
}

impl CloudRedis {
    /// Creates a new client with the default configuration.
    pub async fn new() -> Result<Self> {
        Self::new_with_config(gax::options::ClientConfig::default()).await
    }

    /// Creates a new client with the specified configuration.
    pub async fn new_with_config(conf: gax::options::ClientConfig) -> Result<Self> {
        let inner = Self::build_inner(conf).await?;
        Ok(Self { inner })
    }

    /// Creates a new client from the provided stub.
    ///
    /// The most common case for calling this function is when mocking the
    /// client.
    pub fn from_stub<T>(stub: T) -> Self
    where
        T: super::stubs::CloudRedis + 'static,
    {
        Self {
            inner: Arc::new(stub),
        }
    }

    async fn build_inner(
        conf: gax::options::ClientConfig,
    ) -> Result<Arc<dyn super::stubs::dynamic::CloudRedis>> {
        if conf.tracing_enabled() {
            return Ok(Arc::new(Self::build_with_tracing(conf).await?));
        }
        Ok(Arc::new(Self::build_transport(conf).await?))
    }

    async fn build_transport(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stubs::CloudRedis> {
        super::transport::CloudRedis::new(conf).await
    }

    async fn build_with_tracing(
        conf: gax::options::ClientConfig,
    ) -> Result<impl super::stubs::CloudRedis> {
        Self::build_transport(conf)
            .await
            .map(super::tracing::CloudRedis::new)
    }

    /// Lists all Redis instances owned by a project in either the specified
    /// location (region) or all locations.
    ///
    /// The location should have the following format:
    ///
    /// * `projects/{project_id}/locations/{location_id}`
    ///
    /// If `location_id` is specified as `-` (wildcard), then all regions
    /// available to the project are queried, and the results are aggregated.
    pub fn list_instances(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::cloud_redis::ListInstances {
        super::builders::cloud_redis::ListInstances::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Gets the details of a specific Redis instance.
    pub fn get_instance(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::cloud_redis::GetInstance {
        super::builders::cloud_redis::GetInstance::new(self.inner.clone()).set_name(name.into())
    }

    /// Gets the AUTH string for a Redis instance. If AUTH is not enabled for the
    /// instance the response will be empty. This information is not included in
    /// the details returned to GetInstance.
    pub fn get_instance_auth_string(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::cloud_redis::GetInstanceAuthString {
        super::builders::cloud_redis::GetInstanceAuthString::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Creates a Redis instance based on the specified tier and memory size.
    ///
    /// By default, the instance is accessible from the project's
    /// [default network](https://cloud.google.com/vpc/docs/vpc).
    ///
    /// The creation is executed asynchronously and callers may check the returned
    /// operation to track its progress. Once the operation is completed the Redis
    /// instance will be fully functional. Completed longrunning.Operation will
    /// contain the new instance object in the response field.
    ///
    /// The returned operation is automatically deleted after a few hours, so there
    /// is no need to call DeleteOperation.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn create_instance(
        &self,
        parent: impl Into<std::string::String>,
    ) -> super::builders::cloud_redis::CreateInstance {
        super::builders::cloud_redis::CreateInstance::new(self.inner.clone())
            .set_parent(parent.into())
    }

    /// Updates the metadata and configuration of a specific Redis instance.
    ///
    /// Completed longrunning.Operation will contain the new instance object
    /// in the response field. The returned operation is automatically deleted
    /// after a few hours, so there is no need to call DeleteOperation.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn update_instance(
        &self,
        instance: impl Into<crate::model::Instance>,
    ) -> super::builders::cloud_redis::UpdateInstance {
        super::builders::cloud_redis::UpdateInstance::new(self.inner.clone())
            .set_instance(instance.into())
    }

    /// Upgrades Redis instance to the newer Redis version specified in the
    /// request.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn upgrade_instance(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::cloud_redis::UpgradeInstance {
        super::builders::cloud_redis::UpgradeInstance::new(self.inner.clone()).set_name(name.into())
    }

    /// Import a Redis RDB snapshot file from Cloud Storage into a Redis instance.
    ///
    /// Redis may stop serving during this operation. Instance state will be
    /// IMPORTING for entire operation. When complete, the instance will contain
    /// only data from the imported file.
    ///
    /// The returned operation is automatically deleted after a few hours, so
    /// there is no need to call DeleteOperation.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn import_instance(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::cloud_redis::ImportInstance {
        super::builders::cloud_redis::ImportInstance::new(self.inner.clone()).set_name(name.into())
    }

    /// Export Redis instance data into a Redis RDB format file in Cloud Storage.
    ///
    /// Redis will continue serving during this operation.
    ///
    /// The returned operation is automatically deleted after a few hours, so
    /// there is no need to call DeleteOperation.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn export_instance(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::cloud_redis::ExportInstance {
        super::builders::cloud_redis::ExportInstance::new(self.inner.clone()).set_name(name.into())
    }

    /// Initiates a failover of the primary node to current replica node for a
    /// specific STANDARD tier Cloud Memorystore for Redis instance.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn failover_instance(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::cloud_redis::FailoverInstance {
        super::builders::cloud_redis::FailoverInstance::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Deletes a specific Redis instance.  Instance stops serving and data is
    /// deleted.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn delete_instance(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::cloud_redis::DeleteInstance {
        super::builders::cloud_redis::DeleteInstance::new(self.inner.clone()).set_name(name.into())
    }

    /// Reschedule maintenance for a given instance in a given project and
    /// location.
    ///
    /// # Long running operations
    ///
    /// This method is used to start, and/or poll a [long-running Operation].
    /// The [Working with long-running operations] chapter in the [user guide]
    /// covers these operations in detail.
    ///
    /// [long-running operation]: https://google.aip.dev/151
    /// [user guide]: https://googleapis.github.io/google-cloud-rust/
    /// [working with long-running operations]: https://googleapis.github.io/google-cloud-rust/working_with_long_running_operations.html
    pub fn reschedule_maintenance(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::cloud_redis::RescheduleMaintenance {
        super::builders::cloud_redis::RescheduleMaintenance::new(self.inner.clone())
            .set_name(name.into())
    }

    /// Lists information about the supported locations for this service.
    pub fn list_locations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::cloud_redis::ListLocations {
        super::builders::cloud_redis::ListLocations::new(self.inner.clone()).set_name(name.into())
    }

    /// Gets information about a location.
    pub fn get_location(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::cloud_redis::GetLocation {
        super::builders::cloud_redis::GetLocation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn list_operations(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::cloud_redis::ListOperations {
        super::builders::cloud_redis::ListOperations::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn get_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::cloud_redis::GetOperation {
        super::builders::cloud_redis::GetOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn delete_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::cloud_redis::DeleteOperation {
        super::builders::cloud_redis::DeleteOperation::new(self.inner.clone()).set_name(name.into())
    }

    /// Provides the [Operations][google.longrunning.Operations] service functionality in this service.
    ///
    /// [google.longrunning.Operations]: longrunning::client::Operations
    pub fn cancel_operation(
        &self,
        name: impl Into<std::string::String>,
    ) -> super::builders::cloud_redis::CancelOperation {
        super::builders::cloud_redis::CancelOperation::new(self.inner.clone()).set_name(name.into())
    }
}
