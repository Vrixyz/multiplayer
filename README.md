# TODO:

## Server
- [x] receive move commands
- [x] create and update units locally
- [x] send world state to all clients
- [ ] merge steer project to avoid teleportations

## Client
- [x] send cursor position as move commands
- [x] receive world state from server
- [x] update local entites from server message
- [x] display world entities
- [ ] display correct scale for entities
- [x] try multiple local port before failing

## Multiplayer plugin
- [ ] **TECH:** in client and server, VecDequeue is not really useful because "a first in first out" structure would be preferred.
- [ ] **TECH:** in client, only last received packet is read (because for now only the laster packet is relevant), but in shared com, every packet is serialized, that wastes quite a lot of CPU time