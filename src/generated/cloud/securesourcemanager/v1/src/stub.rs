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

//! Traits to mock the clients in this library.
//!
//! Application developers may need to mock the clients in this library to test
//! how their application works with different (and sometimes hard to trigger)
//! client and service behavior. Such test can define mocks implementing the
//! trait(s) defined in this module, initialize the client with an instance of
//! this mock in their tests, and verify their application responds as expected.

#![allow(rustdoc::broken_intra_doc_links)]

pub(crate) mod dynamic;

/// Defines the trait used to implement [super::client::SecureSourceManager].
///
/// Application developers may need to implement this trait to mock
/// `client::SecureSourceManager`.  In other use-cases, application developers only
/// use `client::SecureSourceManager` and need not be concerned with this trait or
/// its implementations.
///
/// Services gain new RPCs routinely. Consequently, this trait gains new methods
/// too. To avoid breaking applications the trait provides a default
/// implementation of each method. Most of these implementations just return an
/// error.
pub trait SecureSourceManager: std::fmt::Debug + Send + Sync {
    /// Implements [super::client::SecureSourceManager::list_instances].
    fn list_instances(
        &self,
        _req: crate::model::ListInstancesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListInstancesResponse>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::get_instance].
    fn get_instance(
        &self,
        _req: crate::model::GetInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Instance>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::create_instance].
    fn create_instance(
        &self,
        _req: crate::model::CreateInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::delete_instance].
    fn delete_instance(
        &self,
        _req: crate::model::DeleteInstanceRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::list_repositories].
    fn list_repositories(
        &self,
        _req: crate::model::ListRepositoriesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListRepositoriesResponse>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::get_repository].
    fn get_repository(
        &self,
        _req: crate::model::GetRepositoryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Repository>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::create_repository].
    fn create_repository(
        &self,
        _req: crate::model::CreateRepositoryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::update_repository].
    fn update_repository(
        &self,
        _req: crate::model::UpdateRepositoryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::delete_repository].
    fn delete_repository(
        &self,
        _req: crate::model::DeleteRepositoryRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::list_hooks].
    fn list_hooks(
        &self,
        _req: crate::model::ListHooksRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListHooksResponse>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::get_hook].
    fn get_hook(
        &self,
        _req: crate::model::GetHookRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Hook>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::create_hook].
    fn create_hook(
        &self,
        _req: crate::model::CreateHookRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::update_hook].
    fn update_hook(
        &self,
        _req: crate::model::UpdateHookRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::delete_hook].
    fn delete_hook(
        &self,
        _req: crate::model::DeleteHookRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::get_iam_policy_repo].
    fn get_iam_policy_repo(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<iam_v1::model::Policy>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::set_iam_policy_repo].
    fn set_iam_policy_repo(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<iam_v1::model::Policy>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::test_iam_permissions_repo].
    fn test_iam_permissions_repo(
        &self,
        _req: iam_v1::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::create_branch_rule].
    fn create_branch_rule(
        &self,
        _req: crate::model::CreateBranchRuleRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::list_branch_rules].
    fn list_branch_rules(
        &self,
        _req: crate::model::ListBranchRulesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListBranchRulesResponse>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::get_branch_rule].
    fn get_branch_rule(
        &self,
        _req: crate::model::GetBranchRuleRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::BranchRule>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::update_branch_rule].
    fn update_branch_rule(
        &self,
        _req: crate::model::UpdateBranchRuleRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::delete_branch_rule].
    fn delete_branch_rule(
        &self,
        _req: crate::model::DeleteBranchRuleRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::create_pull_request].
    fn create_pull_request(
        &self,
        _req: crate::model::CreatePullRequestRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::get_pull_request].
    fn get_pull_request(
        &self,
        _req: crate::model::GetPullRequestRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::PullRequest>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::list_pull_requests].
    fn list_pull_requests(
        &self,
        _req: crate::model::ListPullRequestsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListPullRequestsResponse>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::update_pull_request].
    fn update_pull_request(
        &self,
        _req: crate::model::UpdatePullRequestRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::merge_pull_request].
    fn merge_pull_request(
        &self,
        _req: crate::model::MergePullRequestRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::open_pull_request].
    fn open_pull_request(
        &self,
        _req: crate::model::OpenPullRequestRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::close_pull_request].
    fn close_pull_request(
        &self,
        _req: crate::model::ClosePullRequestRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::list_pull_request_file_diffs].
    fn list_pull_request_file_diffs(
        &self,
        _req: crate::model::ListPullRequestFileDiffsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<
            gax::response::Response<crate::model::ListPullRequestFileDiffsResponse>,
        >,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::fetch_tree].
    fn fetch_tree(
        &self,
        _req: crate::model::FetchTreeRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::FetchTreeResponse>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::fetch_blob].
    fn fetch_blob(
        &self,
        _req: crate::model::FetchBlobRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::FetchBlobResponse>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::create_issue].
    fn create_issue(
        &self,
        _req: crate::model::CreateIssueRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::get_issue].
    fn get_issue(
        &self,
        _req: crate::model::GetIssueRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::Issue>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::list_issues].
    fn list_issues(
        &self,
        _req: crate::model::ListIssuesRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListIssuesResponse>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::update_issue].
    fn update_issue(
        &self,
        _req: crate::model::UpdateIssueRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::delete_issue].
    fn delete_issue(
        &self,
        _req: crate::model::DeleteIssueRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::open_issue].
    fn open_issue(
        &self,
        _req: crate::model::OpenIssueRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::close_issue].
    fn close_issue(
        &self,
        _req: crate::model::CloseIssueRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::get_pull_request_comment].
    fn get_pull_request_comment(
        &self,
        _req: crate::model::GetPullRequestCommentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::PullRequestComment>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::list_pull_request_comments].
    fn list_pull_request_comments(
        &self,
        _req: crate::model::ListPullRequestCommentsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<
            gax::response::Response<crate::model::ListPullRequestCommentsResponse>,
        >,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::create_pull_request_comment].
    fn create_pull_request_comment(
        &self,
        _req: crate::model::CreatePullRequestCommentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::update_pull_request_comment].
    fn update_pull_request_comment(
        &self,
        _req: crate::model::UpdatePullRequestCommentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::delete_pull_request_comment].
    fn delete_pull_request_comment(
        &self,
        _req: crate::model::DeletePullRequestCommentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::batch_create_pull_request_comments].
    fn batch_create_pull_request_comments(
        &self,
        _req: crate::model::BatchCreatePullRequestCommentsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::resolve_pull_request_comments].
    fn resolve_pull_request_comments(
        &self,
        _req: crate::model::ResolvePullRequestCommentsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::unresolve_pull_request_comments].
    fn unresolve_pull_request_comments(
        &self,
        _req: crate::model::UnresolvePullRequestCommentsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::create_issue_comment].
    fn create_issue_comment(
        &self,
        _req: crate::model::CreateIssueCommentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::get_issue_comment].
    fn get_issue_comment(
        &self,
        _req: crate::model::GetIssueCommentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::IssueComment>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::list_issue_comments].
    fn list_issue_comments(
        &self,
        _req: crate::model::ListIssueCommentsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<crate::model::ListIssueCommentsResponse>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::update_issue_comment].
    fn update_issue_comment(
        &self,
        _req: crate::model::UpdateIssueCommentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::delete_issue_comment].
    fn delete_issue_comment(
        &self,
        _req: crate::model::DeleteIssueCommentRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::list_locations].
    fn list_locations(
        &self,
        _req: location::model::ListLocationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<location::model::ListLocationsResponse>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::get_location].
    fn get_location(
        &self,
        _req: location::model::GetLocationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<location::model::Location>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::set_iam_policy].
    fn set_iam_policy(
        &self,
        _req: iam_v1::model::SetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<iam_v1::model::Policy>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::get_iam_policy].
    fn get_iam_policy(
        &self,
        _req: iam_v1::model::GetIamPolicyRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<iam_v1::model::Policy>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::test_iam_permissions].
    fn test_iam_permissions(
        &self,
        _req: iam_v1::model::TestIamPermissionsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<iam_v1::model::TestIamPermissionsResponse>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::list_operations].
    fn list_operations(
        &self,
        _req: longrunning::model::ListOperationsRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::ListOperationsResponse>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::get_operation].
    fn get_operation(
        &self,
        _req: longrunning::model::GetOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<
        Output = crate::Result<gax::response::Response<longrunning::model::Operation>>,
    > + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::delete_operation].
    fn delete_operation(
        &self,
        _req: longrunning::model::DeleteOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Implements [super::client::SecureSourceManager::cancel_operation].
    fn cancel_operation(
        &self,
        _req: longrunning::model::CancelOperationRequest,
        _options: gax::options::RequestOptions,
    ) -> impl std::future::Future<Output = crate::Result<gax::response::Response<()>>> + Send {
        gaxi::unimplemented::unimplemented_stub()
    }

    /// Returns the polling error policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_error_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_error_policy::PollingErrorPolicy> {
        std::sync::Arc::new(gax::polling_error_policy::Aip194Strict)
    }

    /// Returns the polling backoff policy.
    ///
    /// When mocking, this method is typically irrelevant. Do not try to verify
    /// it is called by your mocks.
    fn get_polling_backoff_policy(
        &self,
        _options: &gax::options::RequestOptions,
    ) -> std::sync::Arc<dyn gax::polling_backoff_policy::PollingBackoffPolicy> {
        std::sync::Arc::new(gax::exponential_backoff::ExponentialBackoff::default())
    }
}
