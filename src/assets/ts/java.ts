import $ from "jquery";

export class JavaVersion
{
    executable: string | null = null;
    installed: boolean = false;
    operating_system: string = "";
    runtime: string = "";
    version: string = "";

    constructor(data: any)
    {
        this.executable = data.executable;
        this.installed = data.installed;
        this.operating_system = data.operating_system;
        this.runtime = data.runtime;
        this.version = data.version;
    }

    async install(): Promise<void>
    {
        let websocket = new WebSocket(`/api/java/install/${this.version}/ws`);
        websocket.onmessage = (event) =>
        {
            console.log(event.data);
        };
        websocket.onclose = () =>
        {
            console.log("WebSocket closed");
        };
        websocket.onerror = (event) =>
        {
            console.error(event);
        };
        websocket.onopen = () =>
        {
            console.log("WebSocket opened");
        };
    }
}

export default class Java
{
    static async versions(): Promise<JavaVersion[]>
    {
        return (await $.get("/api/java/versions")).map((data: any) => new JavaVersion(data));
    }
}