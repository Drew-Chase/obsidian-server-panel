export interface Instance
{
    id: string;
    name: string;
    author: string;
    description: string;
    downloads: number;
    likes: number;
    lastUpdated: Date;
    published: Date;
    platform: Platforms;
}

export interface InstalledInstance extends Instance{
    servers: string[];
    owner: string;
}

export enum Platforms
{
    MODRINTH = "modrinth",
    CURSEFORGE = "curseforge",
    ATLAUNCHER = "at-launcher",
}

export enum SortOptions
{
    RELEVANT = "relevant",
    POPULAR = "popular",
    NEWEST = "newest",
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
    static async browse(options: BrowseOptions): Promise<Instance[]>
    {
        return $.get("/api/");
    }
}