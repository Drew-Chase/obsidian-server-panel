# API Roadmap

This is used as more of a todo list for the project. It is not a strict roadmap, but more of a list of things that need to be done.

## Dashboard Pages

- [x] Add endpoint for creating a server
- [x] Add endpoint for getting all servers
- [x] Add endpoint for getting system usage
    - CPU and Memory
- [x] Add endpoint for getting storage usage
- [ ] Add endpoint for getting all online players for all servers
- [ ] Add endpoint for getting all online players for all servers for every month
- [ ] Add endpoint to export storage usage csv
- [ ] Add endpoint to export all online players csv
- [ ] Add endpoint to get all servers recent connections.
- [ ] Add endpoint to get all servers crash reports.
- [ ] Add endpoint to get all servers backups.
- [ ] Add endpoint to get instances.
- [ ] Add endpoint to create instances.
- [ ] Add endpoint to download instances.
- [ ] Add endpoint to delete instances.
- [ ] Add endpoint to create server from existing instance

## Server Pages

- [x] Add endpoint for getting a single server
- [x] Add endpoint for getting server settings
- [x] Add endpoint for updating server settings
- [x] Add endpoint for getting server properties
- [x] Add endpoint for updating server properties
- [ ] Add endpoint for deleting a server
- [ ] Add endpoint for changing minecraft version
- [ ] Add endpoint for changing loader version
- [ ] Add endpoint for listing loaders and versions
- [ ] Add endpoint for starting the server
- [ ] Add endpoint for stopping the server
- [ ] Add endpoint for getting server process resources
    - CPU and RAM usage
- [ ] Add endpoint for getting current online players.
- [ ] Add endpoint for getting all players (online and offline)
- [x] Add endpoint for getting files in the file system.
- [ ] Add endpoint for uploading files to the file system.
- [ ] Add endpoint for getting text file contents.
- [ ] Add endpoint for updating existing files.
- [ ] Add endpoint for archiving files
- [ ] Add endpoint for downloading files.
- [ ] Add endpoint for getting total server storage and a breakdown of large files.
- [ ] Add endpoint for manually backing up a server
    - using incremental backups or full backups.
- [ ] Add endpoint for setting up a backup schedule.
    - using incremental backups or full backups.
- [ ] Add endpoint for viewing crash reports
    - This should attempt to parse the crash-report and glean a cause and a mod
- [ ] Add endpoint for getting latest console output.
- [ ] Add endpoint for sending console commands.
- [ ] Add endpoint for getting console autocomplete
    - using the minecraft `/help` command
- [ ] Add endpoint to create instance from server.
- [ ] Add endpoint to get mods from [Modrinth](https://modrinth.com) and [Curseforge](https://curseforge.com).

## Discover

- [ ] Add endpoint for listing modpacks from curseforge, modrinth and the atlauncher
- [ ] Add endpoint for creating an instance from modpack
- [ ] Add endpoint for searching for modpacks.

## Java

- [x] Add endpoint for listing java versions
- [ ] Add endpoint for listing installed java versions
- [ ] Add endpoint for downloading java versions
- [ ] Add endpoint for deleting java versions

## Users

- [x] Add endpoint for logging in using username and password.
- [x] Add endpoint for logging in using token.
- [x] Add endpoint for creating a user.
- [x] Add endpoint for listing users.
- [x] Add endpoint for inviting new users.
- [x] Add endpoint for listing invite tokens.
- [x] Add endpoint for validating tokens.