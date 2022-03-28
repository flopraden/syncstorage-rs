use std::collections::HashSet;

use diesel::{
    self,
    dsl::sql,
    insert_into,
    result::{DatabaseErrorKind::UniqueViolation, Error as DieselError},
    sql_query,
    sql_types::{BigInt, Integer},
    ExpressionMethods, QueryDsl,
};

use super::{
    diesel_ext::{OptionalExtension, RunAsyncQueryDsl},
    models::MysqlDb,
    schema::{batch_upload_items, batch_uploads},
};

use crate::{
    db::{params, results, Db, DbError, DbErrorKind, BATCH_LIFETIME},
    error::{ApiErrorKind, ApiResult},
    web::extractors::HawkIdentifier,
};

const MAXTTL: i32 = 2_100_000_000;

pub async fn create(db: &MysqlDb, params: params::CreateBatch) -> ApiResult<results::CreateBatch> {
    let user_id = params.user_id.legacy_id as i64;
    let collection_id = db.get_collection_id(params.collection.clone()).await?;
    // Careful, there's some weirdness here!
    //
    // Sync timestamps are in seconds and quantized to two decimal places, so
    // when we convert one to a bigint in milliseconds, the final digit is
    // always zero. But we want to use the lower digits of the batchid for
    // sharding writes via (batchid % num_tables), and leaving it as zero would
    // skew the sharding distribution.
    //
    // So we mix in the lowest digit of the uid to improve the distribution
    // while still letting us treat these ids as millisecond timestamps.  It's
    // yuck, but it works and it keeps the weirdness contained to this single
    // line of code.
    let batch_id = db.timestamp().as_i64() + (user_id % 10);
    insert_into(batch_uploads::table)
        .values((
            batch_uploads::batch_id.eq(batch_id),
            batch_uploads::user_id.eq(user_id),
            batch_uploads::collection_id.eq(collection_id),
        ))
        .execute(db.clone())
        .await
        .map_err(|e| {
            match e.kind() {
                // The user tried to create two batches with the same timestamp
                ApiErrorKind::Db(db_error)
                    if matches!(
                        db_error.kind(),
                        DbErrorKind::DieselQuery(DieselError::DatabaseError(UniqueViolation, _))
                    ) =>
                {
                    DbErrorKind::Conflict.into()
                }
                _ => e,
            }
        })?;

    do_append(db, batch_id, params.user_id, collection_id, params.bsos).await?;
    Ok(results::CreateBatch {
        id: encode_id(batch_id),
        size: None,
    })
}

pub async fn validate(db: &MysqlDb, params: params::ValidateBatch) -> ApiResult<bool> {
    let batch_id = decode_id(&params.id)?;
    // Avoid hitting the db for batches that are obviously too old.  Recall
    // that the batchid is a millisecond timestamp.
    if (batch_id + BATCH_LIFETIME) < db.timestamp().as_i64() {
        return Ok(false);
    }

    let user_id = params.user_id.legacy_id as i64;
    let collection_id = db.get_collection_id(params.collection).await?;
    let exists = batch_uploads::table
        .select(sql::<Integer>("1"))
        .filter(batch_uploads::batch_id.eq(batch_id))
        .filter(batch_uploads::user_id.eq(user_id))
        .filter(batch_uploads::collection_id.eq(collection_id))
        .get_result::<i32>(db.clone())
        .await
        .optional()?;
    Ok(exists.is_some())
}

pub async fn append(db: &MysqlDb, params: params::AppendToBatch) -> ApiResult<()> {
    let exists = validate(
        db,
        params::ValidateBatch {
            user_id: params.user_id.clone(),
            collection: params.collection.clone(),
            id: params.batch.id.clone(),
        },
    )
    .await?;

    if !exists {
        Err(DbErrorKind::BatchNotFound)?
    }

    let batch_id = decode_id(&params.batch.id)?;
    let collection_id = db.get_collection_id(params.collection.clone()).await?;
    do_append(db, batch_id, params.user_id, collection_id, params.bsos).await?;
    Ok(())
}

pub async fn get(db: &MysqlDb, params: params::GetBatch) -> ApiResult<Option<results::GetBatch>> {
    let is_valid = validate(
        db,
        params::ValidateBatch {
            user_id: params.user_id,
            collection: params.collection.clone(),
            id: params.id.clone(),
        },
    )
    .await?;
    let batch = if is_valid {
        Some(results::GetBatch { id: params.id })
    } else {
        None
    };
    Ok(batch)
}

pub async fn delete(db: &MysqlDb, params: params::DeleteBatch) -> ApiResult<()> {
    let batch_id = decode_id(&params.id)?;
    let user_id = params.user_id.legacy_id as i64;
    let collection_id = db.get_collection_id(params.collection.clone()).await?;
    diesel::delete(batch_uploads::table)
        .filter(batch_uploads::batch_id.eq(batch_id))
        .filter(batch_uploads::user_id.eq(user_id))
        .filter(batch_uploads::collection_id.eq(collection_id))
        .execute(db.clone())
        .await?;
    diesel::delete(batch_upload_items::table)
        .filter(batch_upload_items::batch_id.eq(batch_id))
        .filter(batch_upload_items::user_id.eq(user_id))
        .execute(db.clone())
        .await?;
    Ok(())
}

/// Commits a batch to the bsos table, deleting the batch when succesful
pub async fn commit(db: &MysqlDb, params: params::CommitBatch) -> ApiResult<results::CommitBatch> {
    let batch_id = decode_id(&params.batch.id)?;
    let user_id = params.user_id.legacy_id as i64;
    let collection_id = db.get_collection_id(params.collection.clone()).await?;
    let timestamp = db.timestamp();
    sql_query(include_str!("batch_commit.sql"))
        .bind::<BigInt, _>(user_id as i64)
        .bind::<Integer, _>(collection_id)
        .bind::<BigInt, _>(db.timestamp().as_i64())
        .bind::<BigInt, _>(db.timestamp().as_i64())
        .bind::<BigInt, _>((MAXTTL as i64) * 1000) // XXX:
        .bind::<BigInt, _>(batch_id)
        .bind::<BigInt, _>(user_id as i64)
        .bind::<BigInt, _>(db.timestamp().as_i64())
        .bind::<BigInt, _>(db.timestamp().as_i64())
        .execute(db.clone())
        .await?;

    db.update_collection(user_id as u32, collection_id).await?;

    delete(
        db,
        params::DeleteBatch {
            user_id: params.user_id,
            collection: params.collection.clone(),
            id: params.batch.id,
        },
    )
    .await?;
    Ok(timestamp)
}

pub async fn do_append(
    db: &MysqlDb,
    batch_id: i64,
    user_id: HawkIdentifier,
    _collection_id: i32,
    bsos: Vec<params::PostCollectionBso>,
) -> ApiResult<()> {
    fn exist_idx(user_id: u64, batch_id: i64, bso_id: &str) -> String {
        // Construct something that matches the key for batch_upload_items
        format!(
            "{batch_id}-{user_id}-{bso_id}",
            batch_id = batch_id,
            user_id = user_id,
            bso_id = bso_id,
        )
    }

    // It's possible for the list of items to contain a duplicate key entry.
    // This means that we can't really call `ON DUPLICATE` here, because that's
    // more about inserting one item at a time. (e.g. it works great if the
    // values contain a key that's already in the database, less so if the
    // the duplicate is in the value set we're inserting.
    #[derive(Debug, QueryableByName)]
    #[table_name = "batch_upload_items"]
    struct ExistsResult {
        batch_id: i64,
        id: String,
    }

    #[derive(AsChangeset)]
    #[table_name = "batch_upload_items"]
    struct UpdateBatches {
        payload: Option<String>,
        payload_size: Option<i64>,
        ttl_offset: Option<i32>,
    }

    let mut existing = HashSet::new();

    // pre-load the "existing" hashset with any batched uploads that are already in the table.
    for item in sql_query(
        "SELECT userid as user_id, batch as batch_id, id FROM batch_upload_items WHERE userid=? AND batch=?;",
    )
    .bind::<BigInt, _>(user_id.legacy_id as i64)
    .bind::<BigInt, _>(batch_id)
    .get_results::<ExistsResult>(db.clone())
    .await?
    {
        existing.insert(exist_idx(
            user_id.legacy_id,
            item.batch_id,
            &item.id.to_string(),
        ));
    }

    for bso in bsos {
        let payload_size = bso.payload.as_ref().map(|p| p.len() as i64);
        let exist_idx = exist_idx(user_id.legacy_id, batch_id, &bso.id);

        if existing.contains(&exist_idx) {
            diesel::update(
                batch_upload_items::table
                    .filter(batch_upload_items::user_id.eq(user_id.legacy_id as i64))
                    .filter(batch_upload_items::batch_id.eq(batch_id)),
            )
            .set(UpdateBatches {
                payload: bso.payload,
                payload_size,
                ttl_offset: bso.ttl.map(|ttl| ttl as i32),
            })
            .execute(db.clone())
            .await?;
        } else {
            diesel::insert_into(batch_upload_items::table)
                .values((
                    batch_upload_items::batch_id.eq(batch_id),
                    batch_upload_items::user_id.eq(user_id.legacy_id as i64),
                    batch_upload_items::id.eq(bso.id.clone()),
                    batch_upload_items::sortindex.eq(bso.sortindex),
                    batch_upload_items::payload.eq(bso.payload),
                    batch_upload_items::payload_size.eq(payload_size),
                    batch_upload_items::ttl_offset.eq(bso.ttl.map(|ttl| ttl as i32)),
                ))
                .execute(db.clone())
                .await?;
            // make sure to include the key into our table check.
            existing.insert(exist_idx);
        }
    }

    Ok(())
}

pub fn validate_batch_id(id: &str) -> ApiResult<()> {
    decode_id(id).map(|_| ())
}

fn encode_id(id: i64) -> String {
    base64::encode(&id.to_string())
}

fn decode_id(id: &str) -> ApiResult<i64> {
    let bytes = base64::decode(id).unwrap_or_else(|_| id.as_bytes().to_vec());
    let decoded = std::str::from_utf8(&bytes).unwrap_or(id);
    decoded
        .parse::<i64>()
        .map_err(|e| DbError::internal(&format!("Invalid batch_id: {}", e)).into())
}
