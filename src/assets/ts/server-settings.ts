import Server from "./servers.ts";

export default class ServerSettings
{
    public name: string;
    public min_ram: number;
    public max_ram: number;
    public minecraft_arguments: string;
    public java_arguments: string;
    public minecraft_version: string;
    public loader_type: number;
    public loader_version: string;
    public executable: string;

    constructor(name: string, min_ram: number, max_ram: number, minecraft_arguments: string, java_arguments: string, minecraft_version: string, loader_type: number, loader_version: string, executable: string)
    {
        this.name = name;
        this.min_ram = min_ram;
        this.max_ram = max_ram;
        this.minecraft_arguments = minecraft_arguments;
        this.java_arguments = java_arguments;
        this.minecraft_version = minecraft_version;
        this.loader_type = loader_type;
        this.loader_version = loader_version;
        this.executable = executable;
    }

    static async getServerSettings(id: string): Promise<ServerSettings>
    {
        let server = await Server.get(id);
        if (server === null) throw new Error("Server not found");

        return new ServerSettings(
            server.name,
            server.min_ram,
            server.max_ram,
            server.minecraft_arguments ?? "",
            server.java_arguments ?? "",
            server.minecraft_version ?? "",
            server.loader_type,
            server.loader_version ?? "",
            server.executable ?? ""
        );
    }

}