import $ from "jquery";

export type FileItem = {
    name: string;
    path: string;
    is_dir: boolean;
    size: number;
    type: string;
    last_modified: Date;
    created: Date;
}

export default class FileSystem
{
    private readonly serverId: string;
    private readonly token: string;

    constructor(serverId: string)
    {
        this.serverId = serverId;
        this.token = document.cookie.match(/(?:^|;\s*)token=([^;]*)/)?.[1] as string;
    }

    async files(subPath: String): Promise<FileItem[]>
    {
        return $.ajax({
            url: `/api/server/${this.serverId}/files`,
            method: "POST",
            headers: {
                "X-Authorization-Token": this.token
            },
            data: subPath,
            contentType: "text/plain"
        });
    }

    async upload(file: File, path: string, filename?: string): Promise<void>
    {
        let formData = new FormData();
        formData.append("file", file);
        formData.append("json", new Blob([JSON.stringify({filename: filename || file.name, directory: path})], {type: "application/json"}));
        return $.ajax({
            url: `/api/server/${this.serverId}/files/upload`,
            method: "POST",
            headers: {
                "X-Authorization-Token": this.token
            },
            data: formData,
            contentType: false,
            processData: false
        });
    }

}