import $ from "jquery";

export default class ServerSettings
{
    public name: string;
    public min_ram: number;
    public max_ram: number;
    public minecraft_arguments: string;
    public java_arguments: string;
    public minecraft_version: string;
    public loader: string;
    public loader_version: string;
    public executable: string;

    constructor(name: string, min_ram: number, max_ram: number, minecraft_arguments: string, java_arguments: string, minecraft_version: string, loader: string, loader_version: string, executable: string)
    {
        this.name = name;
        this.min_ram = min_ram;
        this.max_ram = max_ram;
        this.minecraft_arguments = minecraft_arguments;
        this.java_arguments = java_arguments;
        this.minecraft_version = minecraft_version;
        this.loader = loader;
        this.loader_version = loader_version;
        this.executable = executable;
    }

    static async getServerSettings(id: string): Promise<ServerSettings>
    {
        const settings = await $.get(`/api/server/${id}/settings`) as ServerSettings;
        return new ServerSettings(
            settings.name,
            settings.min_ram,
            settings.max_ram,
            settings.minecraft_arguments ?? "",
            settings.java_arguments ?? "",
            settings.minecraft_version,
            settings.loader,
            settings.loader_version,
            settings.executable
        );
    }

}