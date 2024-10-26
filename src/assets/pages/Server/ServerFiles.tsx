import {Badge, BreadcrumbItem, Breadcrumbs, Button, Divider, Tooltip} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faFileArchive, faFileDownload, faFolderPlus, faTrashAlt, faUpload} from "@fortawesome/free-solid-svg-icons";
import {setTitle} from "../../../main.tsx";
import {useEffect, useState} from "react";
import FileSystem, {FileItem} from "../../ts/file-system.ts";
import {useSelectedServer} from "../../providers/SelectedServerProvider.tsx";
import ServerFilesList from "../../components/Server/Files/ServerFilesList.tsx";

export default function ServerFiles()
{
    const [files, setFiles] = useState<FileItem[]>([]);
    const [path, setPath] = useState<string>("");
    const {server} = useSelectedServer();
    const [loading, setLoading] = useState(false);
    const [selectedItems, setSelectedItems] = useState<FileItem[]>([]);
    setTitle("Server Files");

    useEffect(() =>
    {
        setLoading(true);
        if (server == null) return;
        new FileSystem(server.id)
            .files(path)
            .then(setFiles)
            .finally(() => setLoading(false));
    }, [path, server]);

    if (server == null) return null;
    return (
        <div className={"flex flex-col bg-neutral-600 rounded-2xl shadow-lg p-8 grow w-full mx-2 gap-4 max-h-[calc(100dvh_-_60px)]"}>
            <div className={"flex flex-row gap-2"}>
                <p className={"text-xl font-semibold mr-2"}>Server Files</p>
                <Breadcrumbs variant={"solid"} classNames={{
                    list: "bg-neutral-800"
                }}>
                    <BreadcrumbItem onClick={() => setPath("")}>Root</BreadcrumbItem>
                    {path.split("/").slice(1).map((part, index) => (
                        <BreadcrumbItem key={index} onClick={() =>
                        {
                            const parts = path.split("/");
                            parts.splice(index + 1);
                            setPath(parts.join("/"));
                        }}>{part}</BreadcrumbItem>
                    ))}
                </Breadcrumbs>
                <div className={"ml-auto flex flex-row gap-2"}>
                    <Tooltip content={"Upload a file or directory."}>
                        <Button className={"min-w-0"} aria-label="Upload a file or directory."><FontAwesomeIcon icon={faUpload}/></Button>
                    </Tooltip>
                    <Tooltip content={"Create a new directory."}>
                        <Button className={"min-w-0"} aria-label="Create a new directory."><FontAwesomeIcon icon={faFolderPlus}/></Button>
                    </Tooltip>

                    {selectedItems.length > 0 && (
                        <div className={"flex flex-row gap-4"}>
                            <Divider orientation={"vertical"}/>
                            <Badge content={selectedItems.length} showOutline={false} color={"primary"}>
                                <Tooltip content={`Archive and download ${selectedItems.length} files`}>
                                    <Button className={"min-w-0"} aria-label="Archive and download files"><FontAwesomeIcon icon={faFileDownload}/></Button>
                                </Tooltip>
                            </Badge>
                            <Badge content={selectedItems.length} showOutline={false} color={"primary"}>
                                <Tooltip content={`Archive ${selectedItems.length} files`}>
                                    <Button className={"min-w-0"} aria-label="Archive files"><FontAwesomeIcon icon={faFileArchive}/></Button>
                                </Tooltip>
                            </Badge>
                            <Badge content={selectedItems.length} showOutline={false} color={"primary"}>
                                <Tooltip content={`Delete ${selectedItems.length} files`}>
                                    <Button className={"min-w-0"} aria-label={`Delete ${selectedItems.length} files`} color={"danger"}><FontAwesomeIcon icon={faTrashAlt}/></Button>
                                </Tooltip>
                            </Badge>
                        </div>
                    )}

                </div>
            </div>
            <ServerFilesList
                files={files}
                selectedItems={selectedItems}
                onSelectionChange={setSelectedItems}
                path={path}
                onPathChange={setPath}
                loading={loading}
            />
        </div>
    );
}