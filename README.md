# Arceon - Next-Generation Fantasy MMORPG

🌟 **Welcome to Arceon** - A revolutionary fantasy MMORPG with modern authentication, player management, and social systems, featuring three unique launch modes: decentralized P2P, solo gameplay, and traditional server-client architecture.

## 🎮 Game Philosophy

Arceon reimagines the MMORPG experience by combining:

- **Living World**: The realm of Espan with intelligent NPCs who learn, evolve, and build civilizations
- **Flexible Architecture**: Choose your preferred way to play - decentralized, solo, or centralized
- **Modern Authentication**: Secure user accounts with session management and optional wallet binding
- **Social Systems**: Groups, guilds, and comprehensive player management with chat channels
- **Advanced Skills**: 70+ skills across 6 categories with dynamic progression and migration systems
- **Skill Evolution**: Dynamic skill system where NPCs and players discover new abilities through item interaction
- **Emergent Storytelling**: AI-driven NPCs create evolving narratives and construct persistent structures
- **Database Management**: Multi-backend persistence with SQLite, in-memory, and hybrid storage options

## 🔐 Authentication & Server Systems

Arceon features a modern, secure server system with comprehensive player management:

### 🛡️ Authentication System
**Secure User Accounts**
- **Password Security**: SHA256 hashing with unique salts for each user
- **Session Management**: Secure session tokens with configurable timeout
- **Account Features**: User profiles, character slots, and account settings
- **Wallet Integration**: Optional decentralized wallet binding for blockchain features
- **Duplicate Prevention**: Case-insensitive username validation and duplicate detection

### 🎮 Player Management
**Character System**
- **Multiple Characters**: Support for multiple character slots per account
- **Character Data**: Comprehensive character profiles with stats, skills, and progress
- **Database Integration**: Seamless character data persistence and loading
- **Session Tracking**: Active character selection and session state management

### 👥 Social Systems

#### **Group System (Party Play)**
- **Small Groups**: Support for up to 8 players per group
- **Role-Based Permissions**: Leader, Officer, Member, and Trainee roles
- **Group Chat**: Dedicated chat channels with message history and filtering
- **Activity Tracking**: Group activities, objectives, and progress tracking
- **Invitation System**: Secure invitation system with expiration tracking

#### **Guild System (Large Organizations)**
- **Large Scale**: Support for up to 40 members per guild
- **Hierarchical Leadership**: 6-tier rank system (GuildMaster → Officer → Lieutenant → Veteran → Member → Initiate)
- **Advanced Features**: Guild banking, territories, diplomacy, and campaigns
- **Communication**: Multiple chat channels with role-based access controls
- **Management Tools**: Comprehensive guild administration and member tracking

### 📊 Skill Migration System
**Advanced Character Skills**
- **70+ Skills**: Comprehensive skill system across 6 categories
- **Dynamic Migration**: Automatic skill updates for existing players
- **Preservation**: Maintains existing character progress while adding new features
- **Reporting**: Detailed migration reports with success tracking
- **API Endpoint**: Administrative skill migration via REST API

### 🗄️ Database Architecture
**Multi-Backend Support**
- **SQLite**: Production-ready persistent storage
- **In-Memory**: High-performance temporary storage
- **Hybrid**: Combined persistent + memory for optimal performance
- **Migration System**: Automated database schema updates
- **Backup Support**: Automated backup and retention policies

## 🚀 Launch Modes

Arceon offers four distinct ways to experience the world:

### 🌐 Genesis Mode (Decentralized)
**The Future of Gaming**
```bash
cargo run -- --mode genesis
```
- **P2P Network**: No central servers - world runs on player-maintained nodes
- **Blockchain Persistence**: World state stored on decentralized ledger
- **100% Uptime**: Network remains active as long as nodes are online
- **Peer Connections**: Add nodes via CLI or configuration file

**Adding Peers:**
```bash
# Via command line
cargo run -- --mode genesis --addnode 192.168.1.100:7777 --addnode peer.arceon.world:7777

# Via config file (peers.ini)
cargo run -- --mode genesis --peers-config my-peers.ini
```

### 🏠 Solo Mode (Local)
**Personal Adventure**
```bash
cargo run -- --mode solo
```
- **Offline Gameplay**: Complete game experience without network requirements
- **Local Storage**: All progress saved to your device
- **Full Features**: NPCs, skill system, and world evolution work locally
- **Perfect for**: Testing, learning, or private gameplay

### 🖥️ Centralized Mode (Traditional)
**Classic MMO Experience**
```bash
cargo run -- --mode centralized --server your.gameserver.com:8080
```
- **Traditional Server**: Connect to a dedicated game server
- **Low Latency**: Direct client-server communication
- **Admin Control**: Server administrators manage world state
- **Familiar**: Classic MMO architecture with modern features

### 🌐 Server Mode (New!)
**Run Your Own Game Server**
```bash
cargo run --bin arceon-server
```
- **Full Game Server**: Complete server implementation with all modern features
- **REST API**: RESTful API endpoints for authentication, player management, and social systems
- **Multi-User Support**: Secure user accounts, sessions, and character management
- **Social Features**: Groups, guilds, and comprehensive player interaction systems
- **Admin Tools**: Skill migration, database management, and administrative controls
- **Production Ready**: Designed for hosting real multiplayer experiences

## 🗺️ The World of Arceon

### Living, Breathing Locations

Each area in Arceon is a living ecosystem:

**Central Plains (Starting Area)**
- Scholars studying ancient markers
- Guards patrolling trade routes  
- Wildlife and natural inhabitants
- Player/NPC-built structures like the Waypoint Inn

**Alderheart (Trade Hub)**
- Bustling port with merchants and smiths
- Harbor workers and sea life
- Established structures: shops, forges, lighthouse
- Dynamic trade and commerce

**Silverleaf Enclave (Elven Lands)**
- Mystical elves and forest guardians
- Magical creatures and ancient trees
- Sacred structures: temples, libraries
- Nature magic and ancient wisdom

### Dynamic Structure System

NPCs and players can construct persistent buildings:
- **Houses** and **Town Halls** for community
- **Shops** and **Taverns** for commerce and social interaction
- **Temples** and **Libraries** for knowledge and spirituality
- **Castles** and **Towers** for defense and grandeur

**Enter and Explore:**
```
> look
You see the Waypoint Inn built by Innkeeper Marta (enter available)

> enter inn
You enter the Waypoint Inn and find yourself inside a cozy tavern...
Location: Waypoint Inn Interior - Espan, Central Plains

> exit
You exit the structure and find yourself back in Espan, Central Plains
```

## 🤖 Advanced NPC AI System

### Intelligent Beings

NPCs in Arceon are not simple scripts - they are intelligent entities with:

**Learning Capabilities**
- NPCs observe player actions and adapt their behavior
- Machine learning patterns enable realistic personality development
- Memory systems allow NPCs to remember interactions and relationships

**Social Evolution**
- NPCs form relationships, alliances, and rivalries
- Dynamic faction systems with political intrigue
- Marriage, families, and generational storylines

**Skill Development**
- NPCs discover new abilities by finding items (like players)
- First sword pickup unlocks "Sword" skill
- Crafting tools unlock crafting abilities
- Books and scrolls unlock knowledge skills

**Autonomous Construction**
- NPCs gather resources and plan buildings
- Collaborative construction projects (town halls, bridges)
- Structures placed dynamically in appropriate locations
- Player can enter and explore NPC-built interiors

### Pre-Genesis Training

NPCs begin with sophisticated capabilities:

**Archaic Speech Patterns**
- NPCs speak in period-appropriate medieval/fantasy language
- Context-aware dialogue based on social status and situation
- Regional dialects and cultural speech differences

**Survival and Cooperation**
- Basic survival instincts: food, shelter, safety
- Cooperation algorithms for group projects
- Leadership emergence for community organization

**Knowledge Systems**
- Reading and writing abilities for information transfer
- Library usage and book creation
- Educational institutions and skill sharing

**Governance and Leadership**
- Autonomous town and city management
- Election systems and democratic processes
- Conflict resolution and legal frameworks

## ⚔️ Enhanced Gameplay Systems

### Skill Discovery and Evolution

**Dynamic Skill Unlocking**
- Pick up your first sword → Unlock "Sword" skill
- Find a lute → Unlock "Musical Performance"
- Discover spellbooks → Unlock "Intelligence" and magic abilities
- Use crafting tools → Unlock profession skills

**Racial Bonuses and Starting Skills**
- **Humans**: Versatile learners with tool-making bonuses
- **Elves**: Magical affinity and nature skills
- **Dwarves**: Master craftsmen with mining and smithing
- **Gnomes**: Engineers with illusion magic
- **Halflings**: Lucky gatherers with stealth abilities
- **Orcs**: Fierce warriors with intimidation skills

### Advanced Command System

**Targeting and Interaction**
```bash
target merchant    # Target an NPC or object
hail              # Greet targeted NPC
tell alice hello  # Private message to alice
say hello all     # Public speech to area
pickup sword      # Pick up targeted or named item
enter tavern      # Enter structure with interior
exit              # Leave structure and return outside
```

**Combat and Skills**
```bash
attack            # Attack current target
autoattack on     # Enable automatic combat
skill kick        # Use combat skill
macro             # Manage skill combinations
```

**Movement and Exploration**
```bash
look              # Examine current area (shows NPCs, creatures, structures)
move north        # Travel to connected areas
go 1              # Use numbered exit
map               # View world map
```

**Server Mode Commands**
```bash
stats             # View character stats and health
profile           # Display user profile and account info
skills            # Show character skill categories and progression
who               # List online players
```

## 🌐 Server API Endpoints

Arceon's server mode provides a comprehensive REST API for all game functions:

### 🔐 Authentication Endpoints
```bash
POST /api/auth/register      # Create new user account
POST /api/auth/login         # User login and session creation  
POST /api/auth/logout        # User logout and session cleanup
GET  /api/user/profile       # Get user profile information
```

### 🎮 Player Management
```bash
POST /api/players            # Create new character
POST /api/players/command    # Execute game commands
GET  /api/health            # Server health check and status
```

### 🔗 Wallet Integration
```bash
POST /api/wallet/bind        # Bind decentralized wallet to account
POST /api/wallet/unbind      # Unbind wallet from account  
GET  /api/wallet/status      # Check wallet binding status
```

### 👥 Group System (Party Play)
```bash
POST /api/groups/create      # Create new group
POST /api/groups/invite      # Invite player to group
POST /api/groups/join        # Accept/decline group invitation
POST /api/groups/leave/{id}  # Leave specific group
POST /api/groups/message     # Send message to group chat
GET  /api/groups/status      # Get group status and invitations
```

### 🏰 Guild System (Large Organizations)
```bash
POST /api/guilds/create      # Create new guild
POST /api/guilds/invite      # Invite player to guild
POST /api/guilds/join        # Respond to guild invitation
GET  /api/guilds/status      # Get guild status and membership
```

### ⚙️ Administrative Tools
```bash
POST /api/admin/migrate-skills  # Run skill migration for all players
```

All endpoints require proper authentication via session tokens and support JSON request/response formats.

## 🏗️ Architecture

### Technology Stack

**Core Systems**
- **Rust**: Memory-safe systems programming
- **Bevy ECS**: Entity-component-system for game logic
- **egui**: Cross-platform immediate mode GUI
- **tokio**: Async runtime for networking and concurrency
- **warp**: Fast HTTP server framework for REST API
- **libp2p**: Peer-to-peer networking (Genesis mode)
- **serde**: Serialization for save/load systems
- **tracing**: Structured logging and observability
- **anyhow**: Error handling and propagation

**Modular Design**
- **`arceon-core`**: Game entities, logic, and ECS systems
- **`arceon-gui`**: User interface and windowing
- **`arceon-world`**: World generation and area management
- **`arceon-ai`**: NPC artificial intelligence and behavior
- **`arceon-blockchain`**: Decentralized storage and consensus
- **`arceon-network`**: P2P networking and communication

### World Persistence

**Genesis Mode (Decentralized)**
- World state distributed across peer network
- Blockchain consensus for authoritative changes
- Automatic replication and synchronization
- Fault-tolerant with graceful node disconnection

**Solo Mode (Local)**
- JSON-based save files for all game data
- Automatic backup and versioning
- Portable - copy saves between devices
- No internet required

**Centralized Mode (Traditional)**
- Server manages authoritative game state
- Client receives updates and sends commands
- Low-latency gameplay experience
- Server admin tools and moderation

## 🚀 Getting Started

### Prerequisites

- **Rust**: Install from [rustup.rs](https://rustup.rs/)
- **Git**: For cloning the repository

### Installation

1. **Clone and build:**
```bash
git clone <repository-url>
cd Arceon[rust]
cargo build --release
```

2. **Choose your adventure:**

**Solo Mode (Recommended for new players):**
```bash
cargo run --release -- --mode solo
```

**Genesis Mode (Decentralized):**
```bash
# First time - creates default peer config
cargo run --release -- --mode genesis

# With specific peers
cargo run --release -- --mode genesis --addnode peer1.arceon.world:7777 --addnode 192.168.1.100:7777
```

**Centralized Mode:**
```bash
# Connect to server
cargo run --release -- --mode centralized --server game.arceon.world:8080
```

**Server Mode (Host Your Own):**
```bash
# Run the game server
cargo run --release --bin arceon-server

# Server will start on http://localhost:8080
# Players can connect via REST API or web interface
```

### First Steps

1. **Connect to the world:**
```
> connect YourName
```

2. **Look around:**
```
> look
You stand in Espan's Central Plains...
=== Inhabitants ===
🧙 Eldara the Wise - An ancient scholar
⚔️ Captain Marcus - A seasoned guard
=== Structures ===
🏗️ The Waypoint Inn - Built by Innkeeper Marta (enter available)
```

3. **Explore and interact:**
```
> hail eldara
> enter inn
> move north
> pickup sword    # This unlocks Sword skill!
```

### Server Mode Usage

**Start the Server:**
```bash
cargo run --bin arceon-server
```

**Create Account via API:**
```bash
curl -X POST http://localhost:8080/api/auth/register \
  -H "Content-Type: application/json" \
  -d '{"username": "player1", "password": "SecurePass123!", "email": "player@example.com"}'
```

**Login and Get Session:**
```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"username": "player1", "password": "SecurePass123!"}'
```

**Create Character:**
```bash
curl -X POST http://localhost:8080/api/players \
  -H "Content-Type: application/json" \
  -H "session-id: your-session-id" \
  -d '{"character_name": "Hero", "race": "Human", "starting_area": "Central Plains"}'
```

**Execute Commands:**
```bash
curl -X POST http://localhost:8080/api/players/command \
  -H "Content-Type: application/json" \
  -H "session-id: your-session-id" \
  -d '{"command": "look"}'
```

**Run Skill Migration:**
```bash
curl -X POST http://localhost:8080/api/admin/migrate-skills \
  -H "session-id: your-session-id"
```

### Configuration

**Peer Configuration (peers.ini):**
```ini
[peers]
node1 = 127.0.0.1:7777
node2 = peer.arceon.world:7777
node3 = 192.168.1.100:7777

[settings]
max_peers = 50
timeout = 30
auto_discovery = true
```

**Game Configuration (config.toml):**
```toml
[network]
port = 7777
discovery_port = 7778

[world]
auto_save_interval = 300
max_areas = 1000

[ai]
npc_decision_interval = 5
learning_rate = 0.01
```

## 🌟 Current Features

### ✅ Fully Implemented

**Core Systems:**
- Four launch modes with CLI options (Genesis, Solo, Centralized, Server)
- Area-based world with movement system
- Rich NPC populations in all areas
- Dynamic structure placement and interiors
- Skill discovery through item interaction
- Comprehensive command system
- GUI with height-managed console
- Production-ready game server with REST API

**Authentication & Security:**
- Secure user account system with SHA256 + salt password hashing
- Session management with configurable timeouts
- Duplicate username prevention (case-insensitive)
- Optional decentralized wallet binding for blockchain features
- Comprehensive error handling and validation

**Player Management:**
- Multiple character slots per account
- Character creation with race selection and customization
- Account profiles with settings and preferences
- Session tracking and authentication state management
- Comprehensive player data persistence

**Social Systems:**
- **Group System**: Up to 8 players with role-based permissions and chat channels
- **Guild System**: Up to 40 members with hierarchical leadership (6 rank tiers)
- Group/Guild invitations with expiration tracking
- Dedicated chat channels with message history and filtering
- Activity tracking and group objectives

**Advanced Skills System:**
- **70+ Skills** across 6 categories (Combat, Crafting, Gathering, Social, Magic, Survival)
- Dynamic skill migration system for existing players
- Continuous progression without level caps
- Skill discovery through item interaction
- Administrative migration tools with detailed reporting

**Database Architecture:**
- Multi-backend support (SQLite, In-Memory, Hybrid)
- Automated schema management and migrations
- Backup and retention policies
- Thread-safe concurrent access
- Comprehensive data structures for all game entities

**NPC Intelligence:**
- Machine learning behavior patterns
- Pre-genesis speech training
- Construction and crafting abilities
- Knowledge transfer systems
- Autonomous governance and leadership
- Realistic conversation patterns

**World Systems:**
- 7 unique areas with connections
- NPCs and creatures in every location
- Enterable structures with detailed interiors
- Item-type skill unlocking (47+ item types)
- Racial bonuses and character creation
- Combat with autoattack and skill macros

### 🚧 In Development

**Decentralized Network:**
- P2P consensus mechanisms
- Blockchain integration for world state
- Cross-node synchronization

**Advanced Features:**
- Economic systems and trading
- Advanced crafting systems
- Multi-plane exploration
- Mobile client applications

**Quality of Life:**
- Web-based administration panel
- Real-time player statistics dashboard
- Advanced guild management tools
- In-game messaging system enhancements

## 🤝 Contributing

Arceon thrives on community contribution:

**Priority Areas:**
- **P2P Networking**: Consensus algorithms and synchronization
- **NPC AI**: Advanced behavior patterns and learning systems  
- **World Building**: New areas, storylines, and structures
- **Game Balance**: Skill progression and economic systems
- **Performance**: Optimization for large player counts

**Getting Involved:**
1. Join our community discussions
2. Submit issues and feature requests
3. Create pull requests for improvements
4. Help test different launch modes
5. Design new areas and NPCs

## 📈 Roadmap

### Phase 1: Foundation ✅ COMPLETE
- ✅ Four launch modes (Genesis, Solo, Centralized, Server)
- ✅ Advanced AI and world systems
- ✅ Core gameplay mechanics
- ✅ Authentication and player management
- ✅ Social systems (Groups and Guilds)
- ✅ Advanced skill system with migration

### Phase 2: Network Maturity (Current)
- ✅ Production-ready game server with REST API
- ✅ Comprehensive database architecture
- ✅ Advanced skill systems and character progression
- 🚧 Robust P2P consensus
- 🚧 Blockchain integration
- 🚧 Cross-node synchronization

### Phase 3: World Expansion
- 📋 Additional continents and planes
- 📋 Advanced economic systems and trading
- 📋 Enhanced crafting and production systems
- 📋 Player housing and territory control

### Phase 4: Ecosystem
- 📋 Web-based administration panel
- 📋 Real-time analytics and monitoring
- 📋 Mod support and scripting
- 📋 Mobile client applications
- 📋 Community tools and third-party integrations

## 🌟 Vision Statement

*"Arceon represents the evolution of online gaming - a world that truly belongs to its players, where artificial intelligence creates believable characters, and where technology serves storytelling. Whether you choose to play solo, join a decentralized network, or connect to a traditional server, you're entering a world where every choice matters and every story is unique."*

---

**🎮 Ready to enter Arceon? Choose your mode and begin your adventure!**

*Join thousands of players and NPCs in the ever-evolving realm of Espan, where magic meets technology and every journey is an epic tale waiting to unfold.*