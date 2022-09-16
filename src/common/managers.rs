use super::defines::NetworkType;
use ethers::providers::Http;
use ethers_core::k256::pkcs8::der::oid::Arc;
use once_cell::sync::OnceCell;
use std::{cell::RefCell, collections::HashMap, sync::Mutex};

type Provider = ethers::providers::Provider<Http>;

pub struct ProviderManager {
    pub providers: Mutex<RefCell<HashMap<NetworkType, Provider>>>,
}

static INSTANCE: OnceCell<ProviderManager> = OnceCell::new();

impl ProviderManager {
    pub fn instance() -> &'static ProviderManager {
        INSTANCE.get_or_init(|| ProviderManager {
            providers: Mutex::new(RefCell::new(HashMap::new())),
        })
    }

    pub fn set_provider(&self, network_type: NetworkType, provider: Provider) {
        self.providers
            .lock()
            .unwrap()
            .borrow_mut()
            .insert(network_type, provider);
    }

    pub fn get_provider(&self, network_type: NetworkType) -> Option<Provider> {
        let provider = (*self.providers.lock().unwrap().borrow())
            .get(&network_type)
            .unwrap()
            .to_owned();
        Some(provider)
    }
}
