use dashmap::DashMap;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use thiserror::Error;
use uuid::Uuid;

type Wrap<T> = Arc<Mutex<T>>;

#[derive(Error, Debug)]
pub enum CacheError {
    #[error("The Session is poisoned, please create another one")]
    Poisoned,
    #[error("The session {0} was not found")]
    NotFound(Uuid),
}

pub struct SessionManager<T>
where
    T: Default,
{
    sessions: DashMap<Uuid, Wrap<T>>,
}
impl<T> Default for SessionManager<T>
where
    T: Default,
{
    fn default() -> Self {
        return SessionManager {
            sessions: DashMap::new(),
        };
    }
}
impl<T> SessionManager<T>
where
    T: Default,
{
    pub fn register_new_session(&self) -> (Uuid, Wrap<T>) {
        let uuid = Uuid::new_v4();
        let val = Arc::new(Mutex::new(T::default()));
        self.sessions.insert(uuid, val.clone());
        (uuid, val)
    }
    pub fn register_existing_session(&self, uuid: Uuid) -> Wrap<T> {
        let val = Arc::new(Mutex::new(T::default()));
        self.sessions.insert(uuid, val.clone());
        val
    }

    pub fn save_val(&self, id: Uuid, val: Wrap<T>) -> Result<(), CacheError> {
        if !self.sessions.contains_key(&id) {
            return Err(CacheError::NotFound(id));
        }
        self.sessions.insert(id, val);
        Ok(())
    }

    pub fn get_val(&self, id: Uuid) -> Result<Wrap<T>, CacheError> {
        if !self.sessions.contains_key(&id) {
            return Err(CacheError::NotFound(id));
        }
        match self.sessions.get(&id) {
            Some(e) => Ok(e.clone()),
            None => Err(CacheError::NotFound(id)),
        }
    }
}
