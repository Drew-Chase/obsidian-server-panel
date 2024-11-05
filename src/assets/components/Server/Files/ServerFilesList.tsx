import {Button, Chip, cn, Spinner, Table, TableBody, TableCell, TableColumn, TableHeader, TableRow, Tooltip} from "@nextui-org/react";
import Conversions from "../../../ts/conversions.ts";
import DownloadFile from "../../../images/DownloadFile.svg.tsx";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faEllipsis, faFile, faFolder, faTrash} from "@fortawesome/free-solid-svg-icons";
import {FileItem} from "../../../ts/file-system.ts";
import {useState} from "react";
import {useSelectedServer} from "../../../providers/SelectedServerProvider.tsx";
import {useAlertModal} from "../../../providers/AlertModalProvider.tsx";
import {useSearchParams} from "react-router-dom";
import FileEntryContextDropdown from "./FileEntryContextDropdown.tsx";

interface ServerFilesListProps
{
    files: FileItem[];
    selectedItems: FileItem[];
    onSelectionChange: (selected: FileItem[]) => void;
    path: string;
    onPathChange: (path: string) => void;
    loading: boolean;
    selectionMode: boolean;
    refresh: () => void;
}

export default function ServerFilesList(props: ServerFilesListProps)
{
    const {server} = useSelectedServer();
    const {alert} = useAlertModal();
    const [params] = useSearchParams();
    const [filter] = useState<string[]>((params.get("filter") ?? "").split(","));
    const [currentFile, setCurrentFile] = useState<FileItem | null>(null);
    const [contextPosition, setContextPosition] = useState<{ x: number, y: number }>({x: 0, y: 0});


    return (
        <>
            <FileEntryContextDropdown file={currentFile} position={contextPosition} onClose={() => setCurrentFile(null)} refresh={props.refresh}/>
            <Table
                isStriped
                removeWrapper
                isHeaderSticky
                className={"h-full overflow-y-auto"}
                color={"primary"}
                classNames={{
                    tr: cn(
                        "data-[odd]:!bg-neutral-700/50 data-[selected]:data-[odd]:!bg-primary/10 hover:!bg-neutral-700  transition-colors",
                        "data-[odd=true]:hover:!bg-neutral-700 data-[odd=true]:data-[hover]:!bg-neutral-700 data-[hover]:!bg-neutral-700",
                        "data-[has-open-context-menu=true]:!bg-primary/10 data-[has-open-context-menu=true]:data-[odd=true]:hover:!bg-primary/10"
                    ),
                    th: "bg-neutral-700/50 backdrop-blur-lg",
                    thead: "bg-neutral-700/50 backdrop-blur-lg"
                }}
                checkboxesProps={{
                    className: "w-10"
                }}
                selectionMode={props.selectionMode ? "multiple" : "none"}
                aria-label="Server Files Table"
                onSelectionChange={(selected) =>
                {
                    if (selected === "all") return props.onSelectionChange(props.files);
                    let selectedPaths = [...selected] as string[];
                    props.onSelectionChange(props.files.filter(file => selectedPaths.includes(file.name)));
                }}
                selectedKeys={props.selectedItems.map(item => item.name)}
            >
                <TableHeader>
                    <TableColumn className={"w-full"}>Filename</TableColumn>
                    <TableColumn>Size</TableColumn>
                    <TableColumn>Type</TableColumn>
                    <TableColumn>Actions</TableColumn>
                </TableHeader>

                <TableBody emptyContent={"No items found"} isLoading={props.loading} loadingContent={<Spinner size={"lg"}/>}>
                    {props.files
                        .filter(i =>
                        {
                            if (filter.length === 0) return true;
                            return filter.some(filterItem => i.name.includes(filterItem) || i.is_dir);
                        })
                        .map(file => (
                            <TableRow
                                key={file.name}
                                onContextMenu={e =>
                                {
                                    e.preventDefault();
                                    setCurrentFile(file);
                                    setContextPosition({x: e.pageX - 75, y: e.pageY});
                                }}
                                onDoubleClick={() =>
                                {
                                    if (!props.selectionMode && file.is_dir)
                                        props.onPathChange(`${props.path}${props.path.endsWith("/") ? "" : "/"}${file.name}`);
                                }}
                                data-has-open-context-menu={currentFile === file}
                            >
                                <TableCell>
                                    <div className={"inline-flex items-center"}>
                                        <div className={"text-lg w-5"}>
                                            {file.is_dir ? <FontAwesomeIcon icon={faFolder} className={"text-purple-500"}/> : <FontAwesomeIcon className={"text-blue-400"} icon={faFile}/>}
                                        </div>
                                        <p className={"ml-4 max-w-[30vw] truncate"}>{file.name}</p>
                                    </div>
                                </TableCell>
                                <TableCell>
                                    <div className={"flex flex-row min-w-[100px]"}>
                                        <Chip variant={"flat"} color={"default"}>{Conversions.bytesToSize(file.size)}</Chip>
                                    </div>
                                </TableCell>
                                <TableCell>
                                    <div className={"flex flex-row min-w-[100px]"}>
                                        <Chip variant={"flat"} color={"default"}>{file.is_dir ? "Directory" : file.type}</Chip>
                                    </div>
                                </TableCell>
                                <TableCell>
                                    <div className={"flex flex-row"}>
                                        <Tooltip content={"Download File"} closeDelay={0} classNames={{base: "pointer-events-none"}}>
                                            <Button
                                                className={"min-w-0"}
                                                variant={"light"}
                                                aria-label="Download File"
                                                onClick={() =>
                                                {
                                                    server?.filesystem().download(file);
                                                }}
                                            ><DownloadFile/></Button>
                                        </Tooltip>
                                        <Tooltip content={"Delete File"} closeDelay={0} classNames={{base: "pointer-events-none"}}>
                                            <Button
                                                className={"min-w-0"}
                                                variant={"light"}
                                                color={"danger"}
                                                aria-label="Delete File"
                                                onClick={() =>
                                                {
                                                    alert({
                                                        title: "Delete File",
                                                        message: `Are you sure you want to delete ${file.name}?`,
                                                        type: "warning",
                                                        actions: [
                                                            {
                                                                label: "Delete",
                                                                color: "danger",
                                                                onClick: () =>
                                                                {
                                                                    server?.filesystem().delete(file);
                                                                    props.refresh();
                                                                }
                                                            },
                                                            {
                                                                label: "Cancel",
                                                                color: "default",
                                                                variant: "light"
                                                            }
                                                        ]
                                                    });
                                                }}
                                            >
                                                <FontAwesomeIcon icon={faTrash}/>
                                            </Button>
                                        </Tooltip>
                                        <div>
                                            <Tooltip content={"More options..."} closeDelay={0} classNames={{base: "pointer-events-none"}}>
                                                <Button
                                                    className={"min-w-0"}
                                                    variant={"light"}
                                                    aria-label="More options"
                                                    onPressStart={e => e.continuePropagation()}
                                                    onClick={e =>
                                                    {
                                                        e.preventDefault();
                                                        const rect = e.currentTarget.getBoundingClientRect();
                                                        const x = rect.left + rect.width;
                                                        const y = rect.top + rect.height;

                                                        setCurrentFile(file);
                                                        setContextPosition({x, y});
                                                    }}
                                                >
                                                    <FontAwesomeIcon icon={faEllipsis}/>
                                                </Button>
                                            </Tooltip>
                                        </div>
                                    </div>
                                </TableCell>
                            </TableRow>
                        ))}
                </TableBody>

            </Table>
        </>
    );
}