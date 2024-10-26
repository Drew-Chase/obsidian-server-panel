import $ from "jquery";

export async function getLoaderVersions(loader: string, minecraft_version:string): Promise<string[]>
{
    return $.get(`/api/loaders/${loader}/${minecraft_version}`);
}

export async function getSupportedLoaders(minecraft_version: string, is_snapshot:boolean): Promise<string[]>{
    return $.get(`/api/loaders/supported_loaders/${minecraft_version}?snapshot=${is_snapshot}`);
}