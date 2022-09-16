use super::defines::NetworkType;
use ethers::providers::Http;
use once_cell::sync::OnceCell;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

type MyProvider = ethers::providers::Provider<Http>;

pub struct ProviderManager {
    pub providers: Mutex<HashMap<NetworkType, Arc<MyProvider>>>,
}

static INSTANCE: OnceCell<ProviderManager> = OnceCell::new();

impl ProviderManager {
    pub fn instance() -> &'static ProviderManager {
        INSTANCE.get_or_init(|| ProviderManager {
            providers: Mutex::new(HashMap::new()),
        })
    }

    pub fn set_provider(&self, network_type: NetworkType, provider: MyProvider) {
        self.providers
            .lock()
            .unwrap()
            .insert(network_type, Arc::new(provider));
    }

    pub fn get_provider(&self, network_type: NetworkType) -> Option<Arc<MyProvider>> {
        let provider = (*self.providers.lock().unwrap())
            .get(&network_type)?
            .to_owned();
        Some(provider.clone())
    }
}
