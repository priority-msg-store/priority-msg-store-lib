use crate::api::configuration::{StoreConfig, GroupConfig, update_config};
use crate::api::{ApiErrorTy, ApiError, NoErr, lock};
use crate::core::store::Store;
use crate::api_err;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Mutex;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Info {
    priority: u32,
}

pub fn try_rm(
    store_mutex: &Mutex<Store>,
    configuration_mutex: &Mutex<StoreConfig>,
    configuration_path_option: &Option<PathBuf>, 
    priority: u32) -> Result<(), ApiError<NoErr, NoErr>> {
    {
        let mut store = lock(&store_mutex)?;
        store.delete_group_defaults(priority);
    }
    {
        let mut config = lock(&configuration_mutex)?;
        let groups = match &config.groups {
            Some(groups) => groups,
            None => {
                return Ok(());
            }
        };
        let new_groups: Vec<GroupConfig> = groups
            .iter()
            .filter(|group| {
                if group.priority != priority {
                    true
                } else {
                    false
                }
            })
            .map(|group| group.clone())
            .collect();
        config.groups = Some(new_groups);
        if let Err(error) = update_config(&config, configuration_path_option) {
            return Err(api_err!(ApiErrorTy::ConfigurationError(error)))
        }
    }
    Ok(())
}
