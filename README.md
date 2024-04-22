# HackAndGothic
An action-packed Hack&Slash adventure set in the Gothic universe.
The primary focus is on delivering exceptional gameplay and maximum enjoyment.
With this goal in mind, priority is given to aspects such as itemization, balance, and randomization to ensure dynamic and satisfying experiences.

> [!WARNING]
> The project is currently a prototype, focusing on implementing core gameplay mechanics.  
> I am not yet 100% sure if this idea is possible in ZenGin.

## Building
### Prerequisites
1. Use `direnv` or `nix develop` or install Cargo manually
2. Link `Content` directory to `<GOTHIC_ROOT>/_work/Data/Scripts/Content`.
3. Generate instances
    1. `cd Generator`
    2. `cargo run -- -s ../Content/Game/Items/Templates -t ../Content/Game/Items/Generated`
    2. `cargo run -- -s ../Content/Game/NPCs/Templates -t ../Content/Game/NPCs/Generated`

### Linux (Proton)
`protontricks-launch --appid 39510 Gothic2.exe -zreparse`

### Windows
`Gothic2.exe -zreparse`