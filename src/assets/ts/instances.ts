type InstanceSearchResult = {
    hits: Instance[];
    total_hits: number;
    limit: number;
    offset: number;
}

export interface Instance
{
    id: string;
    name: string;
    author: string;
    description: string;
    downloads: number;
    likes: number;
    last_updated?: Date;
    published?: Date;
    platform: Platforms;
    icon: string;
    gallery: string[];
    versions: string[];
    game_versions: string[];
    categories: string[];
    project_url: string;
    modloader: Modloader;
}

export interface InstalledInstance extends Instance
{
    servers: string[];
    owner: string;
}

export enum Platforms
{
    ALL = "All",
    MODRINTH = "Modrinth",
    CURSEFORGE = "Curseforge",
    ATLAUNCHER = "AtLauncher",
}

export enum Modloader
{
    FABRIC = "Fabric",
    FORGE = "Forge",
    NEOFORGE = "NeoForge",
    QUILT = "Quilt",
    RIFT = "Rift",
    UNKNOWN = "Unknown"
}

export enum SortOptions
{
    RELEVANT = "Relevance",
    POPULAR = "Downloads",
    NEWEST = "Newest",
    FOLLOWS = "Follows",
    UPDATED = "Updated",
}

type BrowseOptions = {
    platform: Platforms;
    search: string;
    sort: SortOptions;
    limit: number;
    offset: number;
}

export default class Instances
{
    static async browse(options: BrowseOptions, signal: AbortSignal): Promise<InstanceSearchResult>
    {
        return fetch(`/api/instances/discover?search=${options.search}&sort=${options.sort}&platform=${options.platform}&limit=${options.limit}&offset=${options.offset}`, {
            signal: signal
        })
            .then(response => response.json())
            .then((result: InstanceSearchResult) =>
            {
                result.hits.forEach(hit =>
                {
                    switch (hit.platform)
                    {
                        case Platforms.MODRINTH:
                            hit.modloader = Object.values(Modloader).find(modloader => hit.categories.map(i => i.toLowerCase()).includes(modloader.toLowerCase())) || Modloader.UNKNOWN;
                            hit.categories = hit.categories.filter(category => category.toLowerCase() !== hit.modloader.toLowerCase());
                            break;
                        case Platforms.CURSEFORGE:
                            hit.modloader = Object.values(Modloader).find(modloader => hit.game_versions.map(i => i.toLowerCase()).includes(modloader.toLowerCase())) || Modloader.UNKNOWN;
                            hit.game_versions = hit.game_versions.filter(version => version.toLowerCase() !== hit.modloader.toLowerCase());
                            break;

                    }
                    if(hit.modloader === Modloader.UNKNOWN) console.log(hit);
                    if (hit.last_updated) hit.last_updated = new Date(hit.last_updated);
                    if (hit.published) hit.published = new Date(hit.published);
                });
                return result;
            });
    }
}