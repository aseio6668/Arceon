# Arceon - Fantasy MMORPG Gameplay Guide

## Starting the Game

### Option 1: GUI Client (Standalone)
```bash
./target/release/arceon.exe
```
- Modern graphical interface with simulated responses
- Point-and-click gameplay
- Built-in console and command input
- Character stats and inventory windows

### Option 2: Client-Server Mode (Recommended)
```bash
# Step 1: Start the dedicated server
./target/release/arceon-server.exe --port 8080

# Step 2: Launch GUI client (in another terminal)
./target/release/arceon.exe
```
- **Real server communication** - Commands are processed by actual server
- **Multiplayer support** - Multiple clients can connect to same server
- **HTTP API** - RESTful endpoints for game operations
- **Live world state** - Real-time updates from server database

### Option 3: Terminal Interface
```bash
./target/release/arceon-terminal.exe
```
- Text-based interface for purists
- Interactive character creation
- Full command-line gameplay

## Getting Started

### 1. Connect to the Game

**In Client-Server Mode:**
```
connect YourCharacterName
```
This will connect to the running server and show real server responses!

**In Standalone Mode:**
```
connect YourCharacterName
```
This simulates the connection locally.

### 2. Basic Commands

**Movement & Exploration:**
- `look` or `l` - Look around your current area
- `move <number>` - Travel to connected areas (use numbers from exits)

**Character Information:**
- `stats` - View your character stats (health, mana, skills)
- `who` - See who's online

**Social Interaction:**
- `say <message>` - Talk to other players
- `talk <npc_name>` - Interact with NPCs
- `npcs` - List NPCs in your current area

**Help:**
- `help` - Show available commands

## World Overview

### The Realm of Espan
Your adventure begins in **Goldenvale**, a proud city with ancient stone walls. From here you can explore:

- **Major Cities**: Alderheart (port city), Silverleaf Enclave, Ironforge
- **Natural Areas**: Ancient Silverleaf Forest, Dragon Spine Mountains
- **Dangerous Regions**: Shifting Crystal Desert, Misty Marshlands
- **Unique Locations**: Skyhold Citadel, Bloodrock Stronghold

### Travel System
Areas are connected by different path types:
- **Highway** - Fast, safe travel between major cities
- **Road** - Standard paths to most locations  
- **Mountain Pass** - Challenging routes through mountains
- **Wilderness Trail** - Paths through natural areas

## Character Progression

### Races Available
- **Human** - Versatile and adaptable
- **Elf** - Graceful masters of magic
- **Gnome** - Clever inventors and crafters
- **Halfling** - Stealthy and peaceful folk
- **Orc** - Strong warriors and shamans
- **Dwarf** - Hardy miners and smiths
- **Dragonborn** - Powerful dragon descendants
- **Tiefling** - Clever and charismatic

### Skills & Abilities
Your character automatically progresses in:
- **Health** - Physical endurance and vitality
- **Defense** - Protective abilities
- **Various Skills** - Based on your actions and activities

## NPC Interactions

### Types of NPCs
- **Merchants** - Found in markets and shops
- **Guards** - Patrol cities and guard posts
- **Scholars** - Located in libraries and temples
- **Rangers** - Roam forests and wilderness
- **Specialized NPCs** - Miners, sailors, mages, etc.

### Talking to NPCs
Use `talk <npc_type>` or `talk <name>` to interact. NPCs provide:
- Area information and lore
- Contextual responses based on location
- Future quest opportunities (coming soon)

## Multiplayer Features

### P2P Networking
- Decentralized player connections
- Real-time area updates
- Shared world state across players

### Social Commands
- See other players in your area with `look`
- Chat with `say <message>`
- Check who's online with `who`

## Tips for New Players

1. **Start with `look`** - Always examine your surroundings first
2. **Use `help`** - Get a quick reference of available commands  
3. **Explore systematically** - Check each area's exits and NPCs
4. **Talk to NPCs** - They provide valuable area information
5. **Check your stats** - Monitor your character's progression

## Server Features

### HTTP API Endpoints
- `GET /api/health` - Server health check
- `POST /api/players` - Create new player characters
- `POST /api/commands` - Process game commands

### Real-Time Features (When Connected to Server)
- **Live World State** - Server maintains persistent world
- **17 Connected Areas** - Full Espan world with rich descriptions
- **NPC Population** - Server spawns and manages NPCs
- **Multiplayer Support** - Multiple clients can connect simultaneously
- **Command Processing** - All game logic handled by server

### Server Administration
```bash
# Start server with custom port
./target/release/arceon-server.exe --port 9999

# Enable debug logging
./target/release/arceon-server.exe --debug

# View server options
./target/release/arceon-server.exe --help
```

## Advanced Features (Coming Soon)

- **Combat System** - Battle monsters and other players
- **Quests & Missions** - Structured storylines and objectives
- **Crafting & Trading** - Create items and trade with players
- **Guilds & Alliances** - Form groups with other players
- **Blockchain Integration** - Secure item ownership and trading

## Technical Notes

### Performance
- Server runs at 10 FPS update rate
- Real-time NPC behavior simulation
- Async command processing for smooth gameplay

### Networking
- P2P mesh networking for decentralization
- Network events broadcast for multiplayer sync
- Connection management for reliability

---

**Enjoy your adventure in the world of Arceon!** 🗡️⚔️🧙‍♂️