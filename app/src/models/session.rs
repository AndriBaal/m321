use actix_session::storage::{LoadError, SaveError, SessionKey, SessionStore, UpdateError};
use actix_web::cookie::time::Duration as TimeDuration;
use anyhow::Error;
use bson::serde_helpers::chrono_datetime_as_bson_datetime;
use chrono::{DateTime, TimeDelta, offset::Utc};
use log::{error, trace};
use mongodb::{
    Collection, Database, IndexModel,
    bson::doc,
    options::IndexOptions,
    results::{InsertOneResult, UpdateResult},
};
use rand::{
    rngs::SmallRng,
    {Rng, SeedableRng},
};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;

use crate::app::AppState;

pub const COLLECTION_NAME: &'static str = "sessions";

thread_local! {
    pub static THREAD_RNG: RefCell<Option<SmallRng>> = const { RefCell::new(None) };
}

pub async fn setup_index(app: &AppState) {
    let collection = app.db.collection::<Session>(COLLECTION_NAME);

    let _ = collection.drop_indexes().await;
    // setup key index
    {
        let options = IndexOptions::builder().unique(true).build();
        let model = IndexModel::builder()
            .keys(doc! {
                "key": 1,
            })
            .options(options)
            .build();

        collection
            .create_index(model)
            .await
            .expect("unable to setup unique key index");
    }

    // setup TTL index
    {
        let options = IndexOptions::builder()
            .expire_after(std::time::Duration::from_secs(0))
            .build();
        let model = IndexModel::builder()
            .keys(doc! {
                "valid_until": 1,
            })
            .options(options)
            .build();

        collection
            .create_index(model)
            .await
            .expect("unable to setup TTL auto cleanup index");
    }
}

/// Session keys are stored as cookies, therefore they cannot be arbitrary long. Session keys are required to be smaller than 4064 bytes.
fn generate_session_key() -> String {
    let mut random: u128 = 0;

    THREAD_RNG.with_borrow_mut(|o: &mut Option<SmallRng>| {
        if o.is_none() {
            *o = Some(SmallRng::from_rng(&mut rand::rng()));
        }

        if let Some(rng) = o {
            random = rng.random();
        }
    });

    format!("{random:032x}")
}

pub async fn connect(db: &Database) -> SessionManager {
    SessionManager {
        collection: db.collection(COLLECTION_NAME),
    }
}

#[derive(Clone)]
pub struct SessionManager {
    pub collection: Collection<Session>,
}

impl SessionStore for SessionManager {
    // Required methods
    async fn load(
        &self,
        session_key: &SessionKey,
    ) -> Result<Option<HashMap<String, String>>, LoadError> {
        let maybe_session = Session::load(&self.collection, session_key.as_ref())
            .await
            .map_err(|err| {
                error!("Failed to load session state... {err:?}");

                LoadError::Other(anyhow::anyhow!("Failed to save session state..."))
            })?;

        Ok(maybe_session.map(|s| s.session_state))
    }

    async fn save(
        &self,
        session_state: HashMap<String, String>,
        ttl: &TimeDuration,
    ) -> Result<SessionKey, SaveError> {
        let session = Session {
            key: generate_session_key(),
            session_state,
            valid_until: now_plus(ttl),
        };

        session.save(&self.collection).await.map_err(|err| {
            error!("Failed to save session state... {err:?}");

            SaveError::Other(anyhow::anyhow!("Failed to save session state..."))
        })?;

        Ok(session
            .key
            .try_into()
            .expect("unable to generate SessionKey"))
    }

    async fn update(
        &self,
        session_key: SessionKey,
        session_state: HashMap<String, String>,
        ttl: &TimeDuration,
    ) -> Result<SessionKey, UpdateError> {
        let maybe_session = Session::load(&self.collection, session_key.as_ref())
            .await
            .map_err(|err| {
                error!("Failed to update session state loading... {err:?}");

                UpdateError::Other(anyhow::anyhow!("Failed to update session state loading..."))
            })?;

        let session = if let Some(mut session) = maybe_session {
            session.session_state = session_state;
            session.valid_until = now_plus(ttl);

            session.update(&self.collection).await.map_err(|err| {
                error!("Failed to update session state updating... {err:?}");
                UpdateError::Other(anyhow::anyhow!(
                    "Failed to update session state updating..."
                ))
            })?;
            session
        } else {
            let session = Session {
                key: generate_session_key(),
                session_state,
                valid_until: now_plus(ttl),
            };
            session.save(&self.collection).await.map_err(|err| {
                error!("Failed to update session state saving... {err:?}");
                UpdateError::Other(anyhow::anyhow!("Failed to update session state saving..."))
            })?;
            session
        };

        Ok(session
            .key
            .try_into()
            .expect("unable to generate session_key"))
    }

    async fn update_ttl(&self, session_key: &SessionKey, ttl: &TimeDuration) -> Result<(), Error> {
        if let Some(mut session) = Session::load(&self.collection, session_key.as_ref()).await? {
            session.valid_until = now_plus(ttl);

            session.update(&self.collection).await?;
        }

        trace!("Update TTL for {}", session_key.as_ref());
        Ok(())
    }

    async fn delete(&self, session_key: &SessionKey) -> Result<(), Error> {
        self.collection
            .delete_one(doc! {
                "key": session_key.as_ref(),
            })
            .await?;

        trace!("Deleted {}", session_key.as_ref());
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Session {
    key: String,
    #[serde(with = "chrono_datetime_as_bson_datetime")]
    valid_until: DateTime<Utc>,
    session_state: HashMap<String, String>,
}
impl Session {
    async fn save(
        &self,
        coll: &Collection<Self>,
    ) -> Result<InsertOneResult, mongodb::error::Error> {
        coll.insert_one(self).await
    }

    async fn update(&self, coll: &Collection<Self>) -> Result<UpdateResult, mongodb::error::Error> {
        coll.replace_one(doc! { "key": &self.key, }, self).await
    }

    async fn load(
        coll: &Collection<Self>,
        session_key: &str,
    ) -> Result<Option<Self>, mongodb::error::Error> {
        coll.find_one(doc! {
            "key": session_key,
        })
        .await
        .map_err(|err| {
            error!("Failed to load Session from MongoDB... {err:?}");
            err
        })
    }
}

fn now_plus(ttl: &TimeDuration) -> DateTime<Utc> {
    Utc::now() + TimeDelta::new(ttl.whole_seconds(), 0).expect("unable to calculate Duration")
}
