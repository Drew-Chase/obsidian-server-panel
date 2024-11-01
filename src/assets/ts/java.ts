import $ from "jquery";

export type InstallItems = {
    file: string;
    completed: boolean;
}

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

    async install(onprogress: (items: InstallItems[]) => void, onerror: (msg: string) => void, oncomplete: () => void): Promise<void>
    {
        let eventSource = new EventSource(`/api/java/install/${this.runtime}/sse`);
        eventSource.addEventListener("open", () => console.log("Installation started"));
        eventSource.addEventListener("progress", (event) =>
        {
            onprogress(JSON.parse(event.data));
        });
        eventSource.addEventListener("error", (event) =>
        {
            console.log("Installation failed", event);
            eventSource.close();
            onerror(JSON.stringify(event));
        });
        eventSource.addEventListener("done", () =>
        {
            console.log("Installation done");
            eventSource.close();
            oncomplete();
        });

    }

    async uninstall(): Promise<void>
    {
        return $.ajax({
            method: "DELETE",
            url: `/api/java/versions/${this.runtime}`
        });
    }

    async files(): Promise<string[]>
    {
        return $.get(`/api/java/versions/${this.runtime}/files`);
    }
}

export default class Java
{
    static async versions(): Promise<JavaVersion[]>
    {
        return (await $.get("/api/java/versions")).map((data: any) => new JavaVersion(data));
    }

    static async installed(): Promise<JavaVersion[]>
    {
        return (await Java.versions()).filter(i => i.installed);
    }
}