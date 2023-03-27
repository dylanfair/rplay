# rplay
This repository holds the code I used to make 'play', a windows CLI to let a user jump into a game from anywhere on the command line just by typing in a partial name of the game

## How to use

> OS Compatability: As of now this program only works on windows machines. If on any other kind of machine it will close the program.

### Set up environment variables
After setting the 'play' executable on the Path, you have to set up your game libraries as environment variables on your computer. At the moment it currently supports a path to your steam library and a path to your blizzard library.

As an administrator, set your system variables to the following:

1. STEAM_PATH="your\path\to\SteamLibrary\steamapps\common"
2. BLIZZARD_PATH="your\path\to\blizzard\games"

That concludes the basic setup for the program. The program will be able to read in the paths assigned to those two variable names, and search for executables in them based on a search term provided. 

### Search the game you want to play
Next step is to provide the program a search term for the game you'd like to play.

For example if we wanted to play Apex Legends, I'd search for the game using 'play' like so:

```powershell
> play apex
Multiple executables found, which one would you like to start?
1. "G:\\SteamLibrary\\steamapps\\common\\Apex Legends\\r5apex.exe"
2. "G:\\SteamLibrary\\steamapps\\common\\Apex Legends\\r5apex_dx12.exe"
3. Cancel

```

### Choose whether to launch the game or exit out
From here if I want to play the game I'd select option #1, and if I wanted to cancel I would select option #3.

```powershell
> play apex
Multiple executables found, which one would you like to start?
1. "G:\\SteamLibrary\\steamapps\\common\\Apex Legends\\r5apex.exe"
2. "G:\\SteamLibrary\\steamapps\\common\\Apex Legends\\r5apex_dx12.exe"
3. Cancel
> 1
Executed process!
```

From there the game should launch and the 'play' program exits out. If you were to select option #3, the program would exit out without launching an executables.

## Known issues
Some games will load wonky, and not bring in some user information that would exist if the game was launched through the client proper. 
