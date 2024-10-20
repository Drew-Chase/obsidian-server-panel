import $ from 'jquery';


export class MinecraftVersion{
    public id: string;
    public type: string;
    public latest: boolean;
    constructor(id: string, type: string, latest: boolean){
        this.id = id;
        this.type = type;
        this.latest = latest;
    }
}

export default class MinecraftVersions{
    static async versions(): Promise<MinecraftVersion[]> {
        return $.get("/api/minecraft/versions");
    }
    static async releases(): Promise<MinecraftVersion[]> {
        return $.get("/api/minecraft/versions/releases");
    }
    static async snapshots(): Promise<MinecraftVersion[]> {
        return $.get("/api/minecraft/versions/snapshots");
    }
}