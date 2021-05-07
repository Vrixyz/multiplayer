# TODO:

## Server
- [x] receive move commands
- [x] receive shoot commands and spawn bullets
- [x] send bullets to clients
- [x] bullets should kill
- [x] collisions not killing
- [ ] collisions on border with kinematic obstacles, remove border_collision system
- [ ] basic level design with kinematic obstacles
- [ ] respawn logic with delay + safezone
- [x] create and update units locally
- [x] send world state to all clients
- [x] merge steer project to avoid teleportations
- [ ] HARD: visibility algorithm : https://www.redblobgames.com/articles/visibility/
- [ ] adapt steer to take time into account rather than frames

## Client
- [x] send cursor position as move commands
- [x] receive world state from server
- [x] update local entites from server message
- [x] display world units
- [x] display world bullets
- [ ] display correct scale for world entities
- [ ] display different sprites for different world entities
- [x] try multiple local port before failing
- [ ] move camera to focus our player
- [ ] add grid to understand space
- [ ] show map borders

## Multiplayer plugin
- [ ] **TECH:** in client and server, VecDequeue is not really useful because "a first in first out" structure would be preferred.
- [ ] **TECH:** in client, only last received packet is read (because for now only the most recent packet is relevant), but in shared com, every packet is serialized, that wastes quite a lot of CPU time
- [ ] **TECH:** nos that physics are becoming more and more complex, rapier2d might be useful to consider ; create an abstraction to avoid being locked in ?
