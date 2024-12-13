import $ from "jquery";
import FileSystem from "./file-system.ts";
import {ServerPropertiesItem} from "../pages/Server/ServerProperties.tsx";
import ServerSettings from "./server-settings.ts";
import {JavaVersion} from "./java.ts";

export enum ServerStatus
{
    Running = "Running",
    Offline = "Offline",
    Restarting = "Restarting",
    Crashed = "Crashed",
}

export default class Server
{
    id: string;
    name: string;
    owner: string;
    members: string[];
    created_at: Date;
    updated_at: Date;
    instance: string | null;
    size: number;
    auto_start: boolean;
    min_ram: number;
    max_ram: number;
    executable: string | null;
    minecraft_arguments: string | null;
    java_arguments: string | null;
    minecraft_version: string | null;
    loader_type: number;
    loader_version: string | null;
    directory: string | null;
    status: ServerStatus = ServerStatus.Offline;
    uptime: number = 0;
    private serverSideEventSource: EventSource | null = null;

    constructor(
        id: string, name: string, owner: string, members: string[],
        created_at: Date, updated_at: Date, instance: string | null = null, size: number = 0,
        auto_start: boolean = false, min_ram: number = 0, max_ram: number = 0, executable: string | null = null,
        minecraft_arguments: string | null = null, java_arguments: string | null = null, minecraft_version: string | null = null,
        loader_type: number = 0, loader_version: string | null = null, directory: string | null = null,
        status: ServerStatus = ServerStatus.Offline, uptime: number = 0
    )
    {
        this.id = id;
        this.name = name;
        this.owner = owner;
        this.members = members;
        this.created_at = created_at;
        this.updated_at = updated_at;
        this.instance = instance;
        this.size = size;
        this.auto_start = auto_start;
        this.min_ram = min_ram;
        this.max_ram = max_ram;
        this.executable = executable;
        this.minecraft_arguments = minecraft_arguments;
        this.java_arguments = java_arguments;
        this.minecraft_version = minecraft_version;
        this.loader_type = loader_type;
        this.loader_version = loader_version;
        this.directory = directory;
        this.status = status;
        this.uptime = uptime;
    }

    static fromJson(json: any): Server
    {
        return new Server(
            json.id ?? "",
            json.name ?? "",
            json.owner ?? "",
            json.members ?? [],
            new Date(json.created_at ?? Date.now()),
            new Date(json.updated_at ?? Date.now()),
            json.instance ?? null,
            json.size ?? 0,
            json.auto_start ?? false,
            json.min_ram ?? 0,
            json.max_ram ?? 0,
            json.executable ?? null,
            json.minecraft_arguments ?? null,
            json.java_arguments ?? null,
            json.minecraft_version ?? null,
            json.loader_type ?? 0,
            json.loader_version ?? null,
            json.directory ?? null,
            json.status ?? ServerStatus.Offline,
            json.uptime ?? 0
        );
    }

    private updateSelf(other: Server): void
    {
        this.id = other.id;
        this.name = other.name;
        this.owner = other.owner;
        this.members = other.members;
        this.created_at = other.created_at;
        this.updated_at = other.updated_at;
        this.instance = other.instance;
        this.size = other.size;
        this.auto_start = other.auto_start;
        this.min_ram = other.min_ram;
        this.max_ram = other.max_ram;
        this.executable = other.executable;
        this.minecraft_arguments = other.minecraft_arguments;
        this.java_arguments = other.java_arguments;
        this.minecraft_version = other.minecraft_version;
        this.loader_type = other.loader_type;
        this.loader_version = other.loader_version;
        this.directory = other.directory;
        this.status = other.status;
        this.uptime = other.uptime;
    }

    toJson(): any
    {
        return {
            id: this.id,
            name: this.name,
            owner: this.owner,
            members: this.members,
            created_at: this.created_at,
            updated_at: this.updated_at,
            instance: this.instance,
            size: this.size,
            auto_start: this.auto_start,
            min_ram: this.min_ram,
            max_ram: this.max_ram,
            executable: this.executable,
            minecraft_arguments: this.minecraft_arguments,
            java_arguments: this.java_arguments,
            minecraft_version: this.minecraft_version,
            loader_type: this.loader_type,
            loader_version: this.loader_version,
            directory: this.directory,
            status: this.status,
            uptime: this.uptime
        };
    }

    static async list(): Promise<Server[]>
    {
        const response = await $.ajax({
            url: "/api/server",
            method: "GET",
            dataType: "json",
            headers: {
                "X-Authorization-Token": document.cookie.match(/(?:^|;\s*)token=([^;]*)/)?.[1]
            }
        });
        return response.map(Server.fromJson);
    }

    static async create(name: string, port: number, difficulty: string, gameMode: string, hardcore: boolean, maxPlayers: number, minecraftVersion: string, loader: string, loaderVersion: string, javaVersion: JavaVersion): Promise<Server | null>
    {
        return $.ajax({
            url: "/api/server",
            method: "POST",
            contentType: "application/json",
            headers: {
                "X-Authorization-Token": document.cookie.match(/(?:^|;\s*)token=([^;]*)/)?.[1]
            },
            data: JSON.stringify({
                name: name,
                port: port,
                difficulty: difficulty,
                gamemode: gameMode,
                hardcore: hardcore,
                max_players: maxPlayers,
                minecraft_version: minecraftVersion,
                loader: loader,
                loader_version: loaderVersion,
                java_path: javaVersion.executable
            })
        });
    }

    static async get(id: string): Promise<Server | null>
    {
        const response = await $.ajax({
            url: `/api/server/${id}`,
            method: "GET",
            dataType: "json"
        });
        return response ? Server.fromJson(response) : null;
    }

    async delete()
    {
        await $.ajax({
            url: `/api/server/${this.id}`,
            method: "DELETE"
        });
    }

    filesystem(): FileSystem
    {
        return new FileSystem(this.id);
    }

    async properties(): Promise<ServerPropertiesItem[]>
    {
        let response: any = await $.ajax({
            url: `/api/server/${this.id}/properties`,
            method: "GET",
            dataType: "json"
        });

        let properties: ServerPropertiesItem[] = [];
        let keys = Object.keys(response);
        let values = Object.values(response);

        for (let i = 0; i < keys.length; i++)
        {
            let key: string = keys[i];
            let value: string = values[i] as string;
            let type: "string" | "number" | "boolean" = value === "true" || value === "false" ? "boolean" : isNaN(Number(value)) ? "string" : "number";
            properties.push({name: key, value: value, type: type});
        }

        return properties;
    }

    async settings(): Promise<ServerSettings>
    {
        return ServerSettings.getServerSettings(this.id);
    }

    async updateProperty(name: string, value: string | boolean | number): Promise<void>
    {
        console.log("Value: ", value);
        await $.ajax({
            url: `/api/server/${this.id}/properties/${name}`,
            method: "POST",
            contentType: "text/plain",
            data: value.toString()
        });
    }

    onServerUpdate(callback: (a: Server, b: Server, diff: any) => void): void
    {
        this.serverSideEventSource = new EventSource(`/api/server/${this.id}/state/sse`);
        this.serverSideEventSource.onopen = () => console.log(`Connected to server side event for server state management for server ${this.id} (${this.name})`);
        this.serverSideEventSource.addEventListener("update_state", (event) =>
        {
            console.log("Update State: ", event);
            let server = Server.fromJson(event.data);
            if (server.id === this.id)
            {
                const diff = Object.fromEntries(
                    Object.entries(server).filter(([key, value]) => (this as any)[key] !== value)
                );
                callback(this, server, diff);
                this.updateSelf(server);
            }
        });
        this.serverSideEventSource.onerror = () => console.error(`Error connecting to server side event for server state management for server ${this.id} (${this.name})`);
        this.serverSideEventSource.addEventListener("ping", event =>
        {
            console.log("Ping event", event);
        });
    }

    closeServerStateUpdateEvent()
    {
        this.serverSideEventSource?.close();
    }


}