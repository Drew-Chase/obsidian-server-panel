import {FileItem, FileMimeCategory} from "../../../ts/file-system.ts";
import {cn, Listbox, ListboxItem, ListboxSection} from "@nextui-org/react";
import {useEffect, useState} from "react";
import $ from "jquery";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faCopy, faEye, faFileDownload, faPencil, faTrashAlt} from "@fortawesome/free-solid-svg-icons";
import {useSelectedServer} from "../../../providers/SelectedServerProvider.tsx";
import RenameModal from "./RenameModal.tsx";
import CopyMoveFileModal from "./CopyMoveFileModal.tsx";
import DownloadFileModal from "./DownloadFileModal.tsx";
import EditorModal from "../../EditorModal.tsx";
import ImageModal from "./ImageModal.tsx";

interface FileEntryContextDropdownProps
{
    file: FileItem | null;
    onClose: () => void;
    position: { x: number, y: number };
    refresh: () => void;
}

export default function FileEntryContextDropdown(props: FileEntryContextDropdownProps)
{

    const [id, setId] = useState("");
    const [renameFile, setRenameFile] = useState<FileItem | null>(null);
    const [copyMoveFile, setCopyMoveFile] = useState<FileItem | null>(null);
    const [viewFile, setViewFile] = useState<FileItem | null>(null);
    const {server} = useSelectedServer();


    useEffect(() =>
    {
        const id = Math.random().toString(36).substring(7);
        setId(id);
        $(document)
            .off("click")
            .on("click", e =>
            {
                const target = $(e.target);
                if (target.closest(`#file-entry-context-dropdown-${id}`).length === 0)
                {
                    props.onClose();
                }
            });
    }, []);

    useEffect(() =>
    {
        // check if position is outside the window
        const dropdown = $(`#file-entry-context-dropdown-${id}`);
        const body = $("body");
        if (dropdown.length === 0) return;
        const {x, y} = props.position;
        const width = dropdown.width() ?? 0;
        const height = dropdown.height() ?? 0;
        const windowWidth = body.width() ?? window.screen.availWidth;
        const windowHeight = body.height() ?? window.screen.availHeight;
        if ((x + width) > windowWidth)
        {
            dropdown.css("left", x - width);
        } else
        {
            dropdown.css("left", x);
        }
        if ((y + height) > windowHeight)
        {
            dropdown.css("top", y - height);
        } else
        {
            dropdown.css("top", y);
        }


    }, [props.position]);

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

            {viewFile !== null &&
                (() =>
                {
                    const type = viewFile.category;
                    console.log(type == FileMimeCategory.TEXT);
                    switch (type)
                    {
                        case FileMimeCategory.IMAGE:
                            return <ImageModal image={server?.filesystem().getFileUrl(viewFile)} onClose={() => setViewFile(null)}/>;
                        case FileMimeCategory.TEXT:
                            return <EditorModal isOpen={true} onClose={() => setViewFile(null)} title={viewFile.name} file={viewFile}/>;
                        default:
                            return <DownloadFileModal file={viewFile} onClose={() => setViewFile(null)}/>;
                    }
                })()
            }

            <div id={`file-entry-context-dropdown-${id}`} className={
                cn(
                    "flex flex-col fixed z-10",
                    "min-w-[200px] max-h-[50dvh] min-h-10 overflow-y-auto",
                    "bg-default-100/75 backdrop-blur-md rounded-large shadow-medium",
                    "subpixel-antialiased outline-none box-border text-small p-1",
                    "data-[hidden=true]:opacity-0 data-[hidden=true]:pointer-events-none pointer-events-auto opacity-100 transition-[opacity] duration-200"
                )
            }
                 data-hidden={props.file === null}
            >
                <Listbox>
                    <ListboxSection title={"Actions"} showDivider>
                        <ListboxItem
                            key={"rename-file"}
                            endContent={<FontAwesomeIcon icon={faPencil}/>}
                            onClick={() =>
                            {
                                setRenameFile(props.file);
                                props.onClose();
                            }}
                        >
                            Rename
                        </ListboxItem>
                        <ListboxItem
                            key={"copy-move-file"}
                            endContent={<FontAwesomeIcon icon={faCopy}/>}
                            onClick={() =>
                            {
                                setCopyMoveFile(props.file);
                                props.onClose();
                            }}
                        >
                            Copy/Move
                        </ListboxItem>
                        <ListboxItem
                            key={"edit-view-file"}
                            endContent={<FontAwesomeIcon icon={faEye}/>}
                            hidden={props.file?.is_dir}
                            onClick={() =>
                            {
                                setViewFile(props.file);
                                props.onClose();
                            }}
                        >
                            Edit/View
                        </ListboxItem>
                        <ListboxItem
                            key={"download-file"}
                            endContent={<FontAwesomeIcon icon={faFileDownload}/>}
                            onClick={() =>
                            {
                                server?.filesystem().download(props.file!);
                                props.onClose();
                            }}
                        >
                            {props.file?.is_dir ? "Archive and " : ""} Download
                        </ListboxItem>
                    </ListboxSection>
                    <ListboxSection title={"Danger Zone"}>
                        <ListboxItem
                            key={"delete-file"}
                            className={"text-danger"}
                            endContent={<FontAwesomeIcon icon={faTrashAlt}/>}
                            color={"danger"}
                            onClick={() =>
                            {
                                alert({
                                    title: "Delete File",
                                    message: `Are you sure you want to delete ${props.file?.name}?`,
                                    type: "warning",
                                    actions: [
                                        {
                                            label: "Delete",
                                            color: "danger",
                                            onClick: () =>
                                            {
                                                server?.filesystem().delete(props.file!);
                                                props.onClose();
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
                            Delete
                        </ListboxItem>
                    </ListboxSection>
                </Listbox>
            </div>
        </>
    );
}
