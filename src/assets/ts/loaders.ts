import $ from "jquery";

export async function getFabricVersions(): Promise<string[]>
{
    return $.get("/api/loaders/FABRIC/0");
}
export async function getForgeVersions(minecraft_version: String): Promise<string[]>
{
    return $.get(`/api/loaders/FORGE/${minecraft_version}`);
}