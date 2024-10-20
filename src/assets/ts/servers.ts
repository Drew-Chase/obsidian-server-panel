import $ from "jquery";

export class Server
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
    loader: number;
    loader_version: string | null;
    directory: string | null;

    constructor(
        id: string, name: string, owner: string, members: string[],
        created_at: Date, updated_at: Date, instance: string | null = null, size: number = 0,
        auto_start: boolean = false, min_ram: number = 0, max_ram: number = 0, executable: string | null = null,
        minecraft_arguments: string | null = null, java_arguments: string | null = null, minecraft_version: string | null = null,
        loader: number = 0, loader_version: string | null = null, directory: string | null = null
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
        this.loader = loader;
        this.loader_version = loader_version;
        this.directory = directory;
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
            json.loader ?? 0,
            json.loader_version ?? null,
            json.directory ?? null
        );
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
            loader: this.loader,
            loader_version: this.loader_version,
            directory: this.directory
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

}