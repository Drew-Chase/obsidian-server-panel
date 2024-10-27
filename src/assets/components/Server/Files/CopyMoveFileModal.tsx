import {BreadcrumbItem, Breadcrumbs, Button, Chip, Modal, ModalBody, ModalContent, ModalFooter, ModalHeader, Spinner, Table, TableBody, TableCell, TableColumn, TableHeader, TableRow} from "@nextui-org/react";
import FileSystem, {FileItem} from "../../../ts/file-system.ts";
import Conversions from "../../../ts/conversions.ts";
import {useEffect, useState} from "react";
import {useSelectedServer} from "../../../providers/SelectedServerProvider.tsx";

interface CopyMoveFileModalProps
{
    isOpen: boolean;
    file: FileItem | null;
    onClose: () => void;
}

export default function CopyMoveFileModal(props: CopyMoveFileModalProps)
{
    const [files, setFiles] = useState<FileItem[]>([]);
    const [path, setPath] = useState<string>("");
    const {server} = useSelectedServer();
    const [loading, setLoading] = useState(false);
    const [selectedItems, setSelectedItems] = useState<FileItem | null>(null);
    console.log(props.file);

    useEffect(() =>
    {
        setLoading(true);
        if (server == null) return;
        setSelectedItems(null)
        new FileSystem(server.id)
            .files(path)
            .then(files => setFiles(files.filter(file => file.is_dir)))
            .finally(() => setLoading(false));
    }, [path, server]);

    useEffect(() =>
    {
        console.log(selectedItems);
    }, [selectedItems]);

    return (
        <Modal size={"5xl"} isOpen={props.isOpen} onClose={props.onClose}>
            <ModalContent>
                {onClose => (
                    <>
                        <ModalHeader className={"flex flex-col"}>
                            <span>Copy/Move {props.file?.name}</span>
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
                        </ModalHeader>
                        <ModalBody>
                            <Table
                                isStriped
                                removeWrapper
                                isHeaderSticky
                                className={"h-full overflow-y-auto"}
                                color={"primary"}
                                classNames={{
                                    tr: "data-[odd]:bg-neutral-800 data-[hover]:bg-neutral-700 data-[odd=true]:!bg-neutral-700/10",
                                    th: "bg-neutral-700/50 backdrop-blur-lg",
                                    thead: "bg-neutral-700/50 backdrop-blur-lg"
                                }}
                                checkboxesProps={{
                                    className: "w-10"
                                }}
                                selectionMode={"single"}
                                aria-label="Server Files Table"
                                onSelectionChange={(selected) =>
                                {
                                    setSelectedItems(files.find(file => file.name === [...selected][0] as string) ?? null);
                                }}
                                selectedKeys={[selectedItems?.name] as string[]}
                            >
                                <TableHeader>
                                    <TableColumn className={"w-full"}>Filename</TableColumn>
                                    <TableColumn>Size</TableColumn>
                                    <TableColumn>Type</TableColumn>
                                </TableHeader>

                                <TableBody emptyContent={"No items found"} isLoading={loading} loadingContent={<Spinner size={"lg"}/>}>
                                    {files.map(file => (
                                        <TableRow
                                            key={file.name}
                                            onDoubleClick={() =>
                                            {
                                                if (file.is_dir)
                                                    setPath(`${path}${path.endsWith("/") ? "" : "/"}${file.name}`);
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
                                        </TableRow>
                                    ))}
                                </TableBody>

                            </Table>

                        </ModalBody>
                        <ModalFooter>
                            <Button onClick={onClose} isDisabled={selectedItems === null}>Move</Button>
                            <Button onClick={onClose} isDisabled={selectedItems === null}>Copy</Button>
                            <Button color={"danger"} variant={"light"} onClick={onClose}>Cancel</Button>
                        </ModalFooter>
                    </>
                )}
            </ModalContent>
        </Modal>
    );
}