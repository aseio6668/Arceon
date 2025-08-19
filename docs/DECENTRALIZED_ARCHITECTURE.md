# Arceon Decentralized Architecture

## 🌐 Genesis Mode: P2P World Persistence

### Overview

The Genesis mode implements a fully decentralized MMORPG where world state is maintained across a peer-to-peer network without central servers. This document outlines the architecture for achieving 100% uptime and persistent world state.

## 🏗️ Core Components

### 1. Distributed World State

**World Sharding**
```
World divided into areas, each area has:
- Primary Node: Authoritative for real-time changes
- Replica Nodes: Backup copies for redundancy
- Observer Nodes: Read-only access for players
```

**State Distribution**
- Each area assigned to multiple nodes (3-5 replicas)
- Dynamic rebalancing when nodes join/leave
- Consensus required for permanent changes
- Local caching for performance

### 2. Consensus Mechanisms

**Raft Consensus for Area Authority**
```rust
struct AreaAuthority {
    area_id: String,
    primary_node: NodeId,
    replica_nodes: Vec<NodeId>,
    version: u64,
    last_update: Timestamp,
}
```

**Byzantine Fault Tolerance**
- Minimum 3 nodes per area for safety
- 2/3 majority required for state changes
- Automatic leader election on failures
- Merkle trees for state verification

### 3. Network Protocol

**Message Types**
```rust
enum P2PMessage {
    // World State
    StateSync { area_id: String, state: WorldState },
    StateRequest { area_id: String, version: u64 },
    StateUpdate { area_id: String, changes: Vec<Change> },
    
    // Consensus
    ProposeChange { area_id: String, change: Change },
    VoteChange { change_id: Uuid, vote: Vote },
    CommitChange { change_id: Uuid },
    
    // Node Management
    JoinNetwork { node_info: NodeInfo },
    LeaveNetwork { node_id: NodeId },
    HeartBeat { node_id: NodeId, timestamp: Timestamp },
    
    // Player Actions
    PlayerAction { player_id: Uuid, action: Action },
    PlayerMove { player_id: Uuid, from: String, to: String },
}
```

### 4. Data Persistence

**Blockchain Integration**
```rust
struct WorldBlock {
    block_number: u64,
    previous_hash: Hash,
    timestamp: Timestamp,
    area_updates: Vec<AreaUpdate>,
    player_actions: Vec<PlayerAction>,
    npc_decisions: Vec<NpcDecision>,
    structure_changes: Vec<StructureChange>,
}
```

**Local Storage**
- SQLite for fast queries and caching
- JSON files for human-readable backups
- Compressed state snapshots for sync
- Automatic cleanup of old data

## 🔄 World Synchronization

### Initial Network Join

1. **Discovery Phase**
   - Connect to known peers from config
   - Request network topology
   - Identify area authorities

2. **State Download**
   - Download current world state
   - Verify integrity with merkle proofs
   - Cache locally for performance

3. **Role Assignment**
   - Volunteer for area authority if needed
   - Become replica for load balancing
   - Start as observer until trusted

### Real-Time Updates

**Player Actions**
```
Player -> Local Node -> Area Authority -> Consensus -> All Replicas -> Update Confirmed
```

**NPC Decisions**
```
NPC AI -> Area Authority -> Consensus (if major) -> Update World State
```

**Structure Placement**
```
Build Request -> Resource Check -> Area Authority -> Consensus -> Permanent Placement
```

### Conflict Resolution

**Simultaneous Actions**
- Timestamp ordering for minor conflicts
- Consensus voting for major conflicts
- Rollback mechanism for invalid states
- Player notification of conflicts

**Network Partitions**
- Continue with majority partition
- Merge states when partition heals
- Conflict resolution algorithms
- Player data protection

## 🛡️ Security and Trust

### Node Validation

**Reputation System**
```rust
struct NodeReputation {
    node_id: NodeId,
    uptime_score: f64,        // 0.0 - 1.0
    consensus_participation: f64,
    invalid_proposals: u32,
    trust_score: f64,
}
```

**Stake Requirements**
- Minimum ArcM stake to become authority
- Slashing for malicious behavior
- Rewards for good behavior
- Economic incentives alignment

### Anti-Cheat Measures

**Action Validation**
- Physics validation for movement
- Resource checks for crafting
- Skill requirements for actions
- Rate limiting for spam prevention

**State Verification**
- Merkle tree state proofs
- Cryptographic signatures
- Periodic state audits
- Anomaly detection

## ⚡ Performance Optimization

### Network Efficiency

**Message Batching**
- Group similar updates together
- Compress message payloads
- Priority queues for urgent updates
- Adaptive batch sizes

**Caching Strategy**
```rust
struct WorldCache {
    area_states: LRU<String, AreaState>,
    player_locations: HashMap<Uuid, String>,
    npc_states: LRU<Uuid, NpcState>,
    structure_index: HashMap<String, Vec<Structure>>,
}
```

### Load Balancing

**Dynamic Sharding**
- Split busy areas into sub-areas
- Migrate players between shards
- Automatic load distribution
- Performance monitoring

**Resource Management**
- CPU usage monitoring
- Memory pool management
- Network bandwidth throttling
- Graceful degradation

## 🔧 Implementation Phases

### Phase 1: Basic P2P (Current)
- Node discovery and connection
- Simple state sharing
- Basic consensus for testing

### Phase 2: Consensus Integration
- Raft implementation for areas
- State synchronization
- Conflict resolution

### Phase 3: Blockchain Storage
- World state blockchain
- Transaction verification
- Persistent storage

### Phase 4: Advanced Features
- Economic incentives
- Cross-shard transactions
- Mobile node support

## 🚀 Deployment Strategy

### Network Bootstrap

**Genesis Nodes**
```
Initial trusted nodes that bootstrap the network:
- genesis1.arceon.world:7777
- genesis2.arceon.world:7777  
- genesis3.arceon.world:7777
```

**Community Expansion**
- Player-run nodes join network
- Gradual decentralization
- Trust transfer to community
- Eventual genesis node retirement

### Monitoring and Health

**Network Metrics**
- Node count and distribution
- Area coverage and redundancy
- Consensus performance
- Player experience metrics

**Health Checks**
- Automatic node health monitoring
- Performance degradation detection
- Network partition detection
- Recovery procedures

## 🎯 Success Criteria

### Technical Goals
- 99.9% uptime for world availability
- <100ms latency for player actions
- Support for 10,000+ concurrent players
- Graceful handling of 50% node failures

### Player Experience Goals
- Seamless gameplay across node changes
- No data loss during network issues
- Transparent decentralization
- Better performance than centralized servers

---

*This architecture enables Arceon to achieve true decentralization while maintaining the seamless gameplay experience players expect from modern MMORPGs.*