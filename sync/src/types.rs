use super::SyncListener;
use futures::Future;
use local_node::LocalNode;
use miner::MemoryPool;
use parking_lot::{Mutex, RwLock};
use std::sync::Arc;
use storage;
use synchronization_client::SynchronizationClient;
use synchronization_executor::LocalSynchronizationTaskExecutor;
use synchronization_peers::Peers;
use synchronization_server::ServerImpl;
use synchronization_verifier::AsyncVerifier;
use utils::SynchronizationState;
use synchronization_shard_block_pool::ShardBlocksPool;

pub use utils::BlockHeight;

/// Network request id
pub type RequestId = u32;

/// Peer is indexed using this type
pub type PeerIndex = usize;

// No-error, no-result future
pub type EmptyBoxFuture = Box<Future<Item = (), Error = ()> + Send>;

/// Reference to storage
pub type StorageRef = storage::SharedStore;

/// Reference to memory pool
pub type MemoryPoolRef = Arc<RwLock<MemoryPool>>;

/// Shared synchronization state reference
pub type SynchronizationStateRef = Arc<SynchronizationState>;

/// Reference to peers
pub type PeersRef = Arc<Peers>;

/// Reference to synchronization tasks executor
pub type ExecutorRef<T> = Arc<T>;

/// Reference to synchronization client
pub type ClientRef<T> = Arc<T>;

/// Reference to synchronization client core
pub type ClientCoreRef<T> = Arc<Mutex<T>>;

/// Reference to synchronization server
pub type ServerRef<T> = Arc<T>;

/// Reference to local node
pub type LocalNodeRef = Arc<
    LocalNode<
        LocalSynchronizationTaskExecutor,
        ServerImpl,
        SynchronizationClient<LocalSynchronizationTaskExecutor, AsyncVerifier>,
    >,
>;

pub type ShardBlocksPoolRef = Arc<RwLock<ShardBlocksPool<SynchronizationClient<LocalSynchronizationTaskExecutor, AsyncVerifier>>>>;

/// Synchronization events listener reference
pub type SyncListenerRef = Box<SyncListener>;
