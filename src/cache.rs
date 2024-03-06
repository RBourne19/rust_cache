use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::thread;
use std::sync::{Arc, Mutex};
use std::hash::Hash;
use std::marker::Send;
#[derive(Debug, Clone)]
pub struct Cache<K,V>{
    pub map: Arc<Mutex<HashMap<K, (V, Instant)>>>,
    expires_in: Duration
}

impl<K: ,V> Cache<K,V>
where K: Eq + PartialEq + Hash + Send + Clone  + 'static,
 V: Send + Clone + 'static
{ 
    pub fn new(expires_in: Duration) -> Self{
        let c: Cache<K, V> = Cache{
            map: Arc::new(Mutex::new(HashMap::new())),
            expires_in: expires_in
        };
        c.udpate_cycle();
        c
        
    }
    //These will lock the main thread
    //Maybe want to switch to async 
    pub fn insert(&self, key: K, value: V) -> Option<(V, Instant)>{
        self.map.lock().unwrap().insert(key, (value, Instant::now()))      
    }
    pub fn remove(&self, key: K) -> Option<(V, Instant)>{
        self.map.lock().unwrap().remove(&key)
    }

    fn udpate_cycle(&self){
        let cache = self.clone();
        let expires_in = self.expires_in;
        let _ = thread::spawn(move||{
            loop{
                thread::sleep(Duration::from_secs(1));
                let now = Instant::now();
                let mut map = cache.map.lock().unwrap();
                for (key, (_, time_inserted)) in map.clone().into_iter(){
                    if (now - time_inserted) >= expires_in{
                        map.remove(&key);
                    }
                }
            }   
            
        });
        
    }
}

