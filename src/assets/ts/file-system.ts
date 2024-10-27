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

    constructor(serverId: string)
    {
        this.serverId = serverId;
    }

    async files(subPath: String): Promise<FileItem[]>
    {
        return $.ajax({
            url: `/api/server/${this.serverId}/files`,
            method: "POST",
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
            data: formData,
            contentType: false,
            processData: false
        });
    }

    async download(file: FileItem): Promise<void>
    {
        return $.ajax({
            url: `/api/server/${this.serverId}/files/download/${encodeURIComponent(file.path)}`,
            method: "GET"
        });
    }

}