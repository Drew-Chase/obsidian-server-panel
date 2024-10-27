import {Button, Chip, DropdownItem, DropdownMenu, DropdownSection, DropdownTrigger, Spinner, Table, TableBody, TableCell, TableColumn, TableHeader, TableRow, Tooltip} from "@nextui-org/react";
import Conversions from "../../../ts/conversions.ts";
import DownloadFile from "../../../images/DownloadFile.svg.tsx";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faCopy, faEllipsis, faEye, faFileDownload, faPencil, faTrash, faTrashAlt} from "@fortawesome/free-solid-svg-icons";
import ODropdown from "../../Extends/ODropdown.tsx";
import {FileItem} from "../../../ts/file-system.ts";
import {useState} from "react";
import RenameModal from "./RenameModal.tsx";
import CopyMoveFileModal from "./CopyMoveFileModal.tsx";

interface ServerFilesListProps
{
    files: FileItem[];
    selectedItems: FileItem[];
    onSelectionChange: (selected: FileItem[]) => void;
    path: string;
    onPathChange: (path: string) => void;
    loading: boolean;
}

export default function ServerFilesList(props: ServerFilesListProps)
{
    const [renameFile, setRenameFile] = useState<FileItem | null>(null);
    const [copyMoveFile, setCopyMoveFile] = useState<FileItem | null>(null);

    return (
        <>
            <RenameModal
                file={renameFile}
                onClose={name =>
                {
                    if (name !== null)
                    {
                        console.log(name);
                    }
                    setRenameFile(null);

                }}
            />
            <CopyMoveFileModal isOpen={copyMoveFile !== null} file={copyMoveFile} onClose={() => setCopyMoveFile(null)}/>
            <Table
                isStriped
                removeWrapper
                isHeaderSticky
                className={"h-full overflow-y-auto"}
                color={"primary"}
                classNames={{
                    tr: "data-[odd]:bg-neutral-800 data-[hover]:bg-neutral-700 data-[odd=true]:!bg-neutral-700",
                    th: "bg-neutral-700/50 backdrop-blur-lg",
                    thead: "bg-neutral-700/50 backdrop-blur-lg"
                }}
                checkboxesProps={{
                    className: "w-10"
                }}
                selectionMode={"multiple"}
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
                    {props.files.map(file => (
                        <TableRow
                            key={file.name}
                            onDoubleClick={() =>
                            {
                                if (file.is_dir)
                                    props.onPathChange(`${props.path}${props.path.endsWith("/") ? "" : "/"}${file.name}`);
                            }}>
                            <TableCell>
                                <p className={"max-w-[30vw] truncate"}>{file.name}</p>
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
                                        <Button className={"min-w-0"} variant={"light"} aria-label="Download File"><DownloadFile/></Button>
                                    </Tooltip>
                                    <Tooltip content={"Delete File"} closeDelay={0} classNames={{base: "pointer-events-none"}}>
                                        <Button className={"min-w-0"} variant={"light"} color={"danger"} aria-label="Delete File"><FontAwesomeIcon icon={faTrash}/></Button>
                                    </Tooltip>
                                    <ODropdown>
                                        <DropdownTrigger>
                                            <div>
                                                <Tooltip content={"More options..."} closeDelay={0} classNames={{base: "pointer-events-none"}}>
                                                    <Button
                                                        className={"min-w-0"}
                                                        variant={"light"}
                                                        aria-label="More options"
                                                        onPressStart={e => e.continuePropagation()}
                                                    >
                                                        <FontAwesomeIcon icon={faEllipsis}/>
                                                    </Button>
                                                </Tooltip>
                                            </div>
                                        </DropdownTrigger>
                                        <DropdownMenu>
                                            <DropdownSection showDivider>
                                                <DropdownItem
                                                    endContent={<FontAwesomeIcon icon={faPencil}/>}
                                                    onClick={() =>
                                                    {
                                                        setRenameFile(file);
                                                    }}
                                                >
                                                    Rename
                                                </DropdownItem>
                                                <DropdownItem
                                                    endContent={<FontAwesomeIcon icon={faCopy}/>}
                                                    onClick={() =>
                                                    {
                                                        setCopyMoveFile(file);
                                                    }}
                                                >
                                                    Copy/Move
                                                </DropdownItem>
                                                <DropdownItem
                                                    endContent={<FontAwesomeIcon icon={faEye}/>}
                                                >
                                                    Edit/View
                                                </DropdownItem>
                                                <DropdownItem
                                                    endContent={<FontAwesomeIcon icon={faFileDownload}/>}
                                                >
                                                    Download
                                                </DropdownItem>
                                            </DropdownSection>
                                            <DropdownSection title={"Danger Zone"}>
                                                <DropdownItem className={"text-danger"} endContent={<FontAwesomeIcon icon={faTrashAlt}/>} color={"danger"}>Delete</DropdownItem>
                                            </DropdownSection>

                                        </DropdownMenu>
                                    </ODropdown>
                                </div>
                            </TableCell>
                        </TableRow>
                    ))}
                </TableBody>

            </Table>
        </>
    );
}