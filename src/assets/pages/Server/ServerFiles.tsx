import {Button, Chip, Table, TableBody, TableCell, TableColumn, TableHeader, TableRow} from "@nextui-org/react";
import DownloadFile from "../../images/DownloadFile.svg.tsx";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faTrash} from "@fortawesome/free-solid-svg-icons";
import {setTitle} from "../../../main.tsx";

export default function ServerFiles()
{
    setTitle("Server Files");
    return (
        <div className={"flex flex-col bg-neutral-600 rounded-2xl shadow-lg p-8 grow w-full mx-2 gap-4 max-h-[calc(100dvh_-_60px)]"}>
            <div className={"flex flex-row"}>
                <p className={"text-xl font-semibold mr-auto"}>Server Files</p>
            </div>
            <Table
                isStriped
                removeWrapper
                isHeaderSticky
                className={"h-full overflow-y-auto"}
                color={"primary"}
                classNames={{
                    tr: "data-[odd]:bg-neutral-800 data-[hover]:bg-neutral-700",
                    th: "bg-neutral-700/50 backdrop-blur-lg",
                    thead: "bg-neutral-700/50 backdrop-blur-lg"
                }}
                checkboxesProps={{
                    className: "w-10"
                }}
                selectionMode={"multiple"}
            >
                <TableHeader>
                    <TableColumn className={"w-full"}>Filename</TableColumn>
                    <TableColumn>Size</TableColumn>
                    <TableColumn>Type</TableColumn>
                    <TableColumn>Actions</TableColumn>
                </TableHeader>

                <TableBody>
                    {Array.from({length: 20}, () => (
                        <TableRow>
                            <TableCell>File.txt</TableCell>
                            <TableCell>
                                <div className={"flex flex-row min-w-[100px]"}>
                                    <Chip variant={"flat"} color={"default"}>1.25KB</Chip>
                                </div>
                            </TableCell>
                            <TableCell>
                                <div className={"flex flex-row min-w-[100px]"}>
                                    <Chip variant={"flat"} color={"default"}>File</Chip>
                                </div>
                            </TableCell>
                            <TableCell>
                                <div className={"flex flex-row"}>
                                    <Button className={"min-w-0"} variant={"light"}><DownloadFile/></Button>
                                    <Button className={"min-w-0"} variant={"light"} color={"danger"}><FontAwesomeIcon icon={faTrash}/></Button>
                                </div>
                            </TableCell>
                        </TableRow>
                    ))}
                </TableBody>

            </Table>
        </div>
    );
}