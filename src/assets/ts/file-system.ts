import $ from "jquery";
import {toast} from "sonner";

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

    async files(subPath: string): Promise<FileItem[]>
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
        toast("Downloading file...", {description: `Started downloading ${file.name}`});
        const response = await fetch(`/api/server/${this.serverId}/files/download/${encodeURIComponent(file.path)}`, {
            method: "GET"
        });

        if (!response.ok)
        {
            throw new Error("Network response was not ok");
        }

        const blob = await response.blob();
        const url = window.URL.createObjectURL(blob);
        const a = document.createElement("a");
        a.style.display = "none";
        a.href = url;
        a.download = file.name;
        document.body.appendChild(a);
        a.click();
        window.URL.revokeObjectURL(url);
    }

    async getFileContents(file: FileItem): Promise<string>
    {
        return $.ajax({
            url: `/api/server/${this.serverId}/files/download/${encodeURIComponent(file.path)}`,
            method: "GET"
        });
    }

    async createDirectory(path: string, name: string): Promise<void>
    {
        return $.ajax({
            url: `/api/server/${this.serverId}/files/create/directory`,
            method: "POST",
            data: `${path}/${name}`,
            contentType: "text/plain"
        });
    }

    async createFile(path: string, name: string): Promise<void>
    {
        return $.ajax({
            url: `/api/server/${this.serverId}/files/create/file`,
            method: "POST",
            data: `${path}/${name}`,
            contentType: "text/plain"
        });
    }

    async delete(file: FileItem): Promise<void>
    {
        return $.ajax({
            url: `/api/server/${this.serverId}/files`,
            method: "DELETE",
            data: file.path,
            contentType: "text/plain"
        });
    }

}