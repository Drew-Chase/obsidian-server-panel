import {Button, Table, TableBody, TableCell, TableColumn, TableHeader, TableRow, Tooltip} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faFloppyDisk, faPlus, faTrash} from "@fortawesome/free-solid-svg-icons";
import DownloadFile from "../../images/DownloadFile.svg.tsx";

export default function ExtendedBackupsList()
{
    return (
        <div className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 w-full mx-2 max-h-[400px] h-dvh overflow-y-auto grow"}>
            <div className={"flex flex-row"}>

                <p className={"text-lg font-semibold mr-auto"}>Backups</p>
                <Tooltip content={"Create a manual backup"}>
                    <Button><FontAwesomeIcon icon={faPlus}/></Button>
                </Tooltip>
            </div>

            <Table
                isStriped
                removeWrapper
                isHeaderSticky
                className={"max-h-[300px] h-[400px] overflow-y-auto"}
                color={"primary"}
                classNames={{
                    tr: "data-[odd]:bg-neutral-800 data-[hover]:bg-neutral-700",
                    th: "bg-neutral-700/50 backdrop-blur-lg",
                    thead: "bg-neutral-700/50 backdrop-blur-lg"
                }}
                checkboxesProps={{
                    className: "w-0"
                }}
                selectionMode={"multiple"}
            >
                <TableHeader>
                    <TableColumn>Name</TableColumn>
                    <TableColumn>Type</TableColumn>
                    <TableColumn>Timestamp</TableColumn>
                    <TableColumn className={"w-0"}>Actions</TableColumn>
                </TableHeader>

                <TableBody>
                    {Array.from({length: 10}, (_, i) => (
                        <TableRow key={i}>
                            <TableCell>Backup #{i}</TableCell>
                            <TableCell>Automatic backup</TableCell>
                            <TableCell>2:36:50 PM</TableCell>
                            <TableCell>
                                <div className={"flex flex-row items-center"}>
                                    <Tooltip content={"Restore from backup"}>
                                        <Button className={"min-w-0"} variant={"light"}><FontAwesomeIcon icon={faFloppyDisk}/></Button>
                                    </Tooltip>
                                    <Tooltip content={"Download backup"}>
                                        <Button className={"min-w-0"} variant={"light"}><DownloadFile/></Button>
                                    </Tooltip>
                                    <Tooltip content={"Delete backup"}>
                                        <Button className={"min-w-0"} variant={"light"} color={"danger"}><FontAwesomeIcon icon={faTrash}/></Button>
                                    </Tooltip>
                                </div>
                            </TableCell>
                        </TableRow>
                    ))}
                </TableBody>

            </Table>
        </div>
    );
}