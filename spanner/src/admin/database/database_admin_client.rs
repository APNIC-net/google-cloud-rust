use google_cloud_gax::conn::Channel;
use google_cloud_gax::create_request;
use google_cloud_gax::grpc::{Response, Status};
use google_cloud_gax::retry::{invoke, RetrySetting};
use google_cloud_googleapis::iam::v1::{
    GetIamPolicyRequest, Policy, SetIamPolicyRequest, TestIamPermissionsRequest, TestIamPermissionsResponse,
};
use google_cloud_googleapis::longrunning::Operation as InternalOperation;
use google_cloud_googleapis::spanner::admin::database::v1::database_admin_client::DatabaseAdminClient as InternalDatabaseAdminClient;
use google_cloud_googleapis::spanner::admin::database::v1::{
    Backup, CreateBackupRequest, CreateDatabaseRequest, Database, DeleteBackupRequest, DropDatabaseRequest,
    GetBackupRequest, GetDatabaseDdlRequest, GetDatabaseDdlResponse, GetDatabaseRequest, ListBackupOperationsRequest,
    ListBackupsRequest, ListDatabaseOperationsRequest, ListDatabasesRequest, RestoreDatabaseRequest,
    UpdateBackupRequest, UpdateDatabaseDdlRequest,
};
use google_cloud_longrunning::autogen::operations_client::OperationsClient;
use google_cloud_longrunning::longrunning::Operation;

use crate::admin::default_retry_setting;

#[derive(Clone)]
pub struct DatabaseAdminClient {
    inner: InternalDatabaseAdminClient<Channel>,
    lro_client: OperationsClient,
}

impl DatabaseAdminClient {
    pub fn new(channel: Channel, lro_client: OperationsClient) -> Self {
        Self {
            inner: InternalDatabaseAdminClient::new(channel).max_decoding_message_size(i32::MAX as usize),
            lro_client,
        }
    }

    /// list_databases lists Cloud Spanner databases.
    #[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
    pub async fn list_databases(
        &self,
        mut req: ListDatabasesRequest,
        retry: Option<RetrySetting>,
    ) -> Result<Vec<Database>, Status> {
        let retry = Some(retry.unwrap_or_else(default_retry_setting));
        let parent = &req.parent;
        let mut all_databases = vec![];
        //eager loading
        loop {
            let action = || async {
                let request = create_request(format!("parent={parent}"), req.clone());
                self.inner.clone().list_databases(request).await.map(|d| d.into_inner())
            };
            let response = invoke(retry.clone(), action).await?;
            all_databases.extend(response.databases.into_iter());
            if response.next_page_token.is_empty() {
                return Ok(all_databases);
            }
            req.page_token = response.next_page_token;
        }
    }

    /// create_database creates a new Cloud Spanner database and starts to prepare it for serving.
    /// The returned [long-running operation][google.longrunning.Operation] will
    /// have a name of the format <database_name>/operations/<operation_id> and
    /// can be used to track preparation of the database. The metadata field type is CreateDatabaseMetadata.
    /// The response field type is Database, if successful.
    #[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
    pub async fn create_database(
        &self,
        req: CreateDatabaseRequest,
        retry: Option<RetrySetting>,
    ) -> Result<Operation<Database>, Status> {
        let retry = Some(retry.unwrap_or_else(default_retry_setting));
        let parent = &req.parent;
        let action = || async {
            let request = create_request(format!("parent={parent}"), req.clone());
            self.inner.clone().create_database(request).await
        };
        invoke(retry, action)
            .await
            .map(|d| Operation::new(self.lro_client.clone(), d.into_inner()))
    }

    /// get_database gets the state of a Cloud Spanner database.
    #[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
    pub async fn get_database(
        &self,
        req: GetDatabaseRequest,
        retry: Option<RetrySetting>,
    ) -> Result<Response<Database>, Status> {
        let retry = Some(retry.unwrap_or_else(default_retry_setting));
        let name = &req.name;
        let action = || async {
            let request = create_request(format!("name={name}"), req.clone());
            self.inner.clone().get_database(request).await
        };
        invoke(retry, action).await
    }

    /// update_database_ddl updates the schema of a Cloud Spanner database by
    /// creating/altering/dropping tables, columns, indexes, etc. The returned
    /// [long-running operation][google.longrunning.Operation] will have a name of
    /// the format <database_name>/operations/<operation_id> and can be used to
    /// track execution of the schema change(s). The
    /// metadata field type is
    /// UpdateDatabaseDdlMetadata.
    /// The operation has no response.
    #[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
    pub async fn update_database_ddl(
        &self,
        req: UpdateDatabaseDdlRequest,
        retry: Option<RetrySetting>,
    ) -> Result<Operation<()>, Status> {
        let retry = Some(retry.unwrap_or_else(default_retry_setting));
        let database = &req.database;
        let action = || async {
            let request = create_request(format!("database={database}"), req.clone());
            self.inner.clone().update_database_ddl(request).await
        };
        invoke(retry, action)
            .await
            .map(|d| Operation::new(self.lro_client.clone(), d.into_inner()))
    }

    /// drop_database drops (aka deletes) a Cloud Spanner database.
    /// Completed backups for the database will be retained according to their
    /// expire_time.
    #[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
    pub async fn drop_database(
        &self,
        req: DropDatabaseRequest,
        retry: Option<RetrySetting>,
    ) -> Result<Response<()>, Status> {
        let retry = Some(retry.unwrap_or_else(default_retry_setting));
        let database = &req.database;
        let action = || async {
            let request = create_request(format!("database={database}"), req.clone());
            self.inner.clone().drop_database(request).await
        };
        invoke(retry, action).await
    }

    /// get_database_ddl returns the schema of a Cloud Spanner database as a list of formatted
    /// DDL statements. This method does not show pending schema updates, those may
    /// be queried using the Operations API.
    #[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
    pub async fn get_database_ddl(
        &self,
        req: GetDatabaseDdlRequest,
        retry: Option<RetrySetting>,
    ) -> Result<Response<GetDatabaseDdlResponse>, Status> {
        let retry = Some(retry.unwrap_or_else(default_retry_setting));
        let database = &req.database;
        let action = || async {
            let request = create_request(format!("database={database}"), req.clone());
            self.inner.clone().get_database_ddl(request).await
        };
        invoke(retry, action).await
    }

    /// set_iam_policy sets the access control policy on a database or backup resource.
    /// Replaces any existing policy.
    ///
    /// Authorization requires spanner.databases.setIamPolicy
    /// permission on resource.
    /// For backups, authorization requires spanner.backups.setIamPolicy
    /// permission on resource.
    #[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
    pub async fn set_iam_policy(
        &self,
        req: SetIamPolicyRequest,
        retry: Option<RetrySetting>,
    ) -> Result<Response<Policy>, Status> {
        let retry = Some(retry.unwrap_or_else(default_retry_setting));
        let resource = &req.resource;
        let action = || async {
            let request = create_request(format!("resource={resource}"), req.clone());
            self.inner.clone().set_iam_policy(request).await
        };
        invoke(retry, action).await
    }

    /// get_iam_policy gets the access control policy for a database or backup resource.
    /// Returns an empty policy if a database or backup exists but does not have a
    /// policy set.
    ///
    /// Authorization requires spanner.databases.getIamPolicy permission on
    /// resource.
    /// For backups, authorization requires spanner.backups.getIamPolicy
    /// permission on resource.
    #[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
    pub async fn get_iam_policy(
        &self,
        req: GetIamPolicyRequest,
        retry: Option<RetrySetting>,
    ) -> Result<Response<Policy>, Status> {
        let retry = Some(retry.unwrap_or_else(default_retry_setting));
        let resource = &req.resource;
        let action = || async {
            let request = create_request(format!("resource={resource}"), req.clone());
            self.inner.clone().get_iam_policy(request).await
        };
        invoke(retry, action).await
    }

    /// test_iam_permissions returns permissions that the caller has on the specified database or backup
    /// resource.
    ///
    /// Attempting this RPC on a non-existent Cloud Spanner database will
    /// result in a NOT_FOUND error if the user has
    /// spanner.databases.list permission on the containing Cloud
    /// Spanner instance. Otherwise returns an empty set of permissions.
    /// Calling this method on a backup that does not exist will
    /// result in a NOT_FOUND error if the user has
    /// spanner.backups.list permission on the containing instance
    #[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
    pub async fn test_iam_permissions(
        &self,
        req: TestIamPermissionsRequest,
        retry: Option<RetrySetting>,
    ) -> Result<Response<TestIamPermissionsResponse>, Status> {
        let retry = Some(retry.unwrap_or_else(default_retry_setting));
        let resource = &req.resource;
        let action = || async {
            let request = create_request(format!("resource={resource}"), req.clone());
            self.inner.clone().test_iam_permissions(request).await
        };
        invoke(retry, action).await
    }

    /// create_backup starts creating a new Cloud Spanner Backup.
    /// The returned backup [long-running operation][google.longrunning.Operation]
    /// will have a name of the format
    /// projects/<project>/instances/<instance>/backups/<backup>/operations/<operation_id>
    /// and can be used to track creation of the backup. The
    /// metadata field type is
    /// CreateBackupMetadata.
    /// The response field type is
    /// Backup, if successful.
    /// Cancelling the returned operation will stop the creation and delete the
    /// backup. There can be only one pending backup creation per database. Backup
    /// creation of different databases can run concurrently.
    #[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
    pub async fn create_backup(
        &self,
        req: CreateBackupRequest,
        retry: Option<RetrySetting>,
    ) -> Result<Operation<Backup>, Status> {
        let retry = Some(retry.unwrap_or_else(default_retry_setting));
        let parent = &req.parent;
        let action = || async {
            let request = create_request(format!("parent={parent}"), req.clone());
            self.inner.clone().create_backup(request).await
        };
        invoke(retry, action)
            .await
            .map(|d| Operation::new(self.lro_client.clone(), d.into_inner()))
    }

    /// get_backup gets metadata on a pending or completed Backup.
    #[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
    pub async fn get_backup(
        &self,
        req: GetBackupRequest,
        retry: Option<RetrySetting>,
    ) -> Result<Response<Backup>, Status> {
        let retry = Some(retry.unwrap_or_else(default_retry_setting));
        let name = &req.name;
        let action = || async {
            let request = create_request(format!("name={name}"), req.clone());
            self.inner.clone().get_backup(request).await
        };
        invoke(retry, action).await
    }

    /// update_backup updates a pending or completed Backup.
    #[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
    pub async fn update_backup(
        &self,
        req: UpdateBackupRequest,
        retry: Option<RetrySetting>,
    ) -> Result<Response<Backup>, Status> {
        let retry = Some(retry.unwrap_or_else(default_retry_setting));
        let name = &req.backup.as_ref().unwrap().name;
        let action = || async {
            let request = create_request(format!("backup.name={name}"), req.clone());
            self.inner.clone().update_backup(request).await
        };
        invoke(retry, action).await
    }

    /// delete_backup deletes a pending or completed Backup.
    #[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
    pub async fn delete_backup(
        &self,
        req: DeleteBackupRequest,
        retry: Option<RetrySetting>,
    ) -> Result<Response<()>, Status> {
        let retry = Some(retry.unwrap_or_else(default_retry_setting));
        let name = &req.name;
        let action = || async {
            let request = create_request(format!("name={name}"), req.clone());
            self.inner.clone().delete_backup(request).await
        };
        invoke(retry, action).await
    }

    /// list_backups lists completed and pending backups.
    /// Backups returned are ordered by create_time in descending order,
    /// starting from the most recent create_time.
    #[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
    pub async fn list_backups(
        &self,
        mut req: ListBackupsRequest,
        retry: Option<RetrySetting>,
    ) -> Result<Vec<Backup>, Status> {
        let retry = Some(retry.unwrap_or_else(default_retry_setting));
        let parent = &req.parent;
        let mut all_backups = vec![];
        //eager loading
        loop {
            let action = || async {
                let request = create_request(format!("parent={parent}"), req.clone());
                self.inner.clone().list_backups(request).await.map(|d| d.into_inner())
            };
            let response = invoke(retry.clone(), action).await?;
            all_backups.extend(response.backups.into_iter());
            if response.next_page_token.is_empty() {
                return Ok(all_backups);
            }
            req.page_token = response.next_page_token;
        }
    }

    /// restore_database create a new database by restoring from a completed backup. The new
    /// database must be in the same project and in an instance with the same
    /// instance configuration as the instance containing
    /// the backup. The returned database [long-running
    /// operation][google.longrunning.Operation] has a name of the format
    /// projects/<project>/instances/<instance>/databases/<database>/operations/<operation_id>,
    /// and can be used to track the progress of the operation, and to cancel it.
    /// The metadata field type is
    /// RestoreDatabaseMetadata.
    /// The response type
    /// is Database, if
    /// successful. Cancelling the returned operation will stop the restore and
    /// delete the database.
    /// There can be only one database being restored into an instance at a time.
    /// Once the restore operation completes, a new restore operation can be
    /// initiated, without waiting for the optimize operation associated with the
    /// first restore to complete.
    #[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
    pub async fn restore_database(
        &self,
        req: RestoreDatabaseRequest,
        retry: Option<RetrySetting>,
    ) -> Result<Operation<Database>, Status> {
        let retry = Some(retry.unwrap_or_else(default_retry_setting));
        let parent = &req.parent;
        let action = || async {
            let request = create_request(format!("parent={parent}"), req.clone());
            self.inner.clone().restore_database(request).await
        };
        invoke(retry, action)
            .await
            .map(|d| Operation::new(self.lro_client.clone(), d.into_inner()))
    }

    /// list_backup_operations lists the backup [long-running operations][google.longrunning.Operation] in
    /// the given instance. A backup operation has a name of the form
    /// projects/<project>/instances/<instance>/backups/<backup>/operations/<operation>.
    /// The long-running operation
    /// metadata field type
    /// metadata.type_url describes the type of the metadata. Operations returned
    /// include those that have completed/failed/canceled within the last 7 days,
    /// and pending operations. Operations returned are ordered by
    /// operation.metadata.value.progress.start_time in descending order starting
    /// from the most recently started operation.
    #[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
    pub async fn list_backup_operations(
        &self,
        mut req: ListBackupOperationsRequest,
        retry: Option<RetrySetting>,
    ) -> Result<Vec<InternalOperation>, Status> {
        let retry = Some(retry.unwrap_or_else(default_retry_setting));
        let parent = &req.parent;
        let mut all_operations = vec![];
        //eager loading
        loop {
            let action = || async {
                let request = create_request(format!("parent={parent}"), req.clone());
                self.inner
                    .clone()
                    .list_backup_operations(request)
                    .await
                    .map(|d| d.into_inner())
            };
            let response = invoke(retry.clone(), action).await?;
            all_operations.extend(response.operations.into_iter());
            if response.next_page_token.is_empty() {
                return Ok(all_operations);
            }
            req.page_token = response.next_page_token;
        }
    }

    /// list_database_operations lists database [longrunning-operations][google.longrunning.Operation].
    /// A database operation has a name of the form
    /// projects/<project>/instances/<instance>/databases/<database>/operations/<operation>.
    /// The long-running operation
    /// metadata field type
    /// metadata.type_url describes the type of the metadata. Operations returned
    /// include those that have completed/failed/canceled within the last 7 days,
    /// and pending operations.
    #[cfg_attr(feature = "trace", tracing::instrument(skip_all))]
    pub async fn list_database_operations(
        &self,
        mut req: ListDatabaseOperationsRequest,
        retry: Option<RetrySetting>,
    ) -> Result<Vec<InternalOperation>, Status> {
        let retry = Some(retry.unwrap_or_else(default_retry_setting));
        let parent = &req.parent;
        let mut all_operations = vec![];
        //eager loading
        loop {
            let action = || async {
                let request = create_request(format!("parent={parent}"), req.clone());
                self.inner
                    .clone()
                    .list_database_operations(request)
                    .await
                    .map(|d| d.into_inner())
            };
            let response = invoke(retry.clone(), action).await?;
            all_operations.extend(response.operations.into_iter());
            if response.next_page_token.is_empty() {
                return Ok(all_operations);
            }
            req.page_token = response.next_page_token;
        }
    }
}
