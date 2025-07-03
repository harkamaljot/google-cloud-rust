# Copyright 2025 Google LLC
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.

variable "project" {}

data "google_project" "project" {
}

# Service account to be impersonated by the workload identity.
resource "google_service_account" "workload_identity_sa" {
  project      = var.project
  account_id   = "test-workload-identity-sa"
  display_name = "Test Service Account for Workload Identity"
}

# Workload Identity Pool
resource "google_iam_workload_identity_pool" "pool" {
  project                   = var.project
  workload_identity_pool_id = "test-byoid-pool"
  display_name              = "Test BYOID Pool"
  description               = "Test pool for BYOID integration tests"
}

# Workload Identity Pool Provider
resource "google_iam_workload_identity_pool_provider" "oidc_provider" {
  project                            = var.project
  workload_identity_pool_id          = google_iam_workload_identity_pool.pool.workload_identity_pool_id
  workload_identity_pool_provider_id = "test-oidc-provider"
  display_name                       = "Test OIDC Provider"
  attribute_mapping = {
    "google.subject" = "assertion.sub"
    "attribute.actor"  = "assertion.actor"
    "attribute.aud"    = "assertion.aud"
  }
  oidc {
    issuer_uri = "https://oidc.integration-tests.google-cloud-rust.com" # A dummy issuer URI
    allowed_audiences = [
      "//iam.googleapis.com/${google_iam_workload_identity_pool.pool.name}",
    ]
  }
}

# Grant the workload identity user role to allow impersonation.
# This allows any identity from the pool to impersonate the service account.
resource "google_service_account_iam_member" "impersonate" {
  service_account_id = google_service_account.workload_identity_sa.name
  role               = "roles/iam.workloadIdentityUser"
  member             = "principalSet://iam.googleapis.com/${google_iam_workload_identity_pool.pool.name}/*"
}

# A secret for testing.
resource "google_secret_manager_secret" "test_secret" {
  project   = var.project
  secret_id = "test-byoid-secret"
  replication {
    auto {}
  }
}

resource "google_secret_manager_secret_version" "test_secret_version" {
  secret      = google_secret_manager_secret.test_secret.id
  secret_data = "workload_identity"
}

# Grant the service account access to the secret.
resource "google_secret_manager_secret_iam_member" "secret_accessor" {
  project   = var.project
  secret_id = google_secret_manager_secret.test_secret.id
  role      = "roles/secretmanager.secretAccessor"
  member    = "serviceAccount:${google_service_account.workload_identity_sa.email}"
}

output "service_account_email" {
  value = google_service_account.workload_identity_sa.email
}

output "audience" {
  value = google_iam_workload_identity_pool_provider.oidc_provider.oidc[0].allowed_audiences[0]
}
