import {Badge, BreadcrumbItem, Breadcrumbs, Button, Divider} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faCheckCircle, faFileArchive, faFileDownload, faFolderPlus, faRefresh, faTrashAlt, faUpload} from "@fortawesome/free-solid-svg-icons";
import {setTitle} from "../../../main.tsx";
import {useEffect, useState} from "react";
import {FileItem} from "../../ts/file-system.ts";
import {useSelectedServer} from "../../providers/SelectedServerProvider.tsx";
import ServerFilesList from "../../components/Server/Files/ServerFilesList.tsx";
import NewPathModal from "../../components/Server/Files/NewPathModal.tsx";
import OTooltip from "../../components/Extends/OTooltip.tsx";

export default function ServerFiles()
{
    const [files, setFiles] = useState<FileItem[]>([]);
    const [path, setPath] = useState<string>("");
    const {server} = useSelectedServer();
    const [loading, setLoading] = useState(false);
    const [selectedItems, setSelectedItems] = useState<FileItem[]>([]);
    const [selectionMode, setSelectionMode] = useState<boolean>(false);
    const [isNewPathModalOpen, setIsNewPathModalOpen] = useState(false);
    setTitle("Server Files");

    const refresh = () =>
    {
        setLoading(true);
        if (server == null) return;
        server
            .filesystem()
            .files(path)
            .then(files=>files.entries)
            .then(setFiles)
            .finally(() => setLoading(false));
    };

    useEffect(() =>
    {
        refresh();
    }, [path, server]);

    useEffect(() =>
    {
        if (!selectionMode)
        {
            setSelectedItems([]);
        }
    }, [selectionMode]);

    if (server == null) return null;
    return (
        <>
            <NewPathModal
                isOpen={isNewPathModalOpen}
                onClose={() =>
                {
                    setIsNewPathModalOpen(false);
                    refresh();
                }}
                currentPath={path}
            />
            <div className={"flex flex-col bg-neutral-600 rounded-2xl shadow-lg p-8 grow w-full mx-2 gap-4 h-dvh max-h-[calc(100dvh_-_125px)]"}>
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
                        <OTooltip content={`${selectionMode ? "Disable" : "Enable"} Selection Mode`}>
                            <Button
                                className={"min-w-0"}
                                aria-label={`${selectionMode ? "Disable" : "Enable"} Selection Mode`}
                                color={selectionMode ? "primary" : "default"}
                                onClick={() => setSelectionMode(prev => !prev)}
                            >
                                <FontAwesomeIcon icon={faCheckCircle}/>
                            </Button>
                        </OTooltip>
                        <OTooltip content={"Upload a file or directory."}>
                            <Button className={"min-w-0"} aria-label="Upload a file or directory."><FontAwesomeIcon icon={faUpload}/></Button>
                        </OTooltip>
                        <OTooltip content={"Create a new file or directory."}>
                            <Button className={"min-w-0"} aria-label="Create a new directory." onClick={() => setIsNewPathModalOpen(true)}><FontAwesomeIcon icon={faFolderPlus}/></Button>
                        </OTooltip>
                        <OTooltip content={"Refresh current directory."}>
                            <Button className={"min-w-0"} aria-label="Refresh current directory." onClick={refresh}><FontAwesomeIcon icon={faRefresh}/></Button>
                        </OTooltip>

                        {selectedItems.length > 0 && (
                            <div className={"flex flex-row gap-4"}>
                                <Divider orientation={"vertical"}/>
                                <Badge content={selectedItems.length} showOutline={false} color={"primary"}>
                                    <OTooltip content={`Archive and download ${selectedItems.length} files`}>
                                        <Button className={"min-w-0"} aria-label="Archive and download files"><FontAwesomeIcon icon={faFileDownload}/></Button>
                                    </OTooltip>
                                </Badge>
                                <Badge content={selectedItems.length} showOutline={false} color={"primary"}>
                                    <OTooltip content={`Archive ${selectedItems.length} files`}>
                                        <Button className={"min-w-0"} aria-label="Archive files"><FontAwesomeIcon icon={faFileArchive}/></Button>
                                    </OTooltip>
                                </Badge>
                                <Badge content={selectedItems.length} showOutline={false} color={"primary"}>
                                    <OTooltip content={`Delete ${selectedItems.length} files`}>
                                        <Button className={"min-w-0"} aria-label={`Delete ${selectedItems.length} files`} color={"danger"}><FontAwesomeIcon icon={faTrashAlt}/></Button>
                                    </OTooltip>
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
                    selectionMode={selectionMode}
                    refresh={refresh}
                />
            </div>
        </>
    );
}
