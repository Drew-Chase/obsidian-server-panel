import {Button, Table, TableBody, TableCell, TableColumn, TableHeader, TableRow} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faFloppyDisk, faPlus, faTrash} from "@fortawesome/free-solid-svg-icons";
import DownloadFile from "../../images/DownloadFile.svg.tsx";
import OTooltip from "../Extends/OTooltip.tsx";

export default function ExtendedBackupsList()
{
    return (
        <div className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 w-full mx-2 overflow-y-auto"}>
            <div className={"flex flex-row"}>

                <p className={"text-lg font-semibold mr-auto"}>Backups</p>
                <OTooltip content={"Create a manual backup"}>
                    <Button><FontAwesomeIcon icon={faPlus}/></Button>
                </OTooltip>
            </div>

            <Table
                isStriped
                removeWrapper
                isHeaderSticky
                className={"h-full overflow-y-auto"}
                color={"primary"}
                aria-label={"Backups Table"}
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
                    {Array.from({length: 20}, (_, i) => (
                        <TableRow key={i}>
                            <TableCell>Backup #{i}</TableCell>
                            <TableCell>Automatic backup</TableCell>
                            <TableCell>2:36:50 PM</TableCell>
                            <TableCell>
                                <div className={"flex flex-row items-center"}>
                                    <OTooltip content={"Restore from backup"}>
                                        <Button className={"min-w-0"} variant={"light"}><FontAwesomeIcon icon={faFloppyDisk}/></Button>
                                    </OTooltip>
                                    <OTooltip content={"Download backup"}>
                                        <Button className={"min-w-0"} variant={"light"}><DownloadFile/></Button>
                                    </OTooltip>
                                    <OTooltip content={"Delete backup"}>
                                        <Button className={"min-w-0"} variant={"light"} color={"danger"}><FontAwesomeIcon icon={faTrash}/></Button>
                                    </OTooltip>
                                </div>
                            </TableCell>
                        </TableRow>
                    ))}
                </TableBody>

            </Table>
        </div>
    );
}