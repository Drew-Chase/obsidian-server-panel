import {Button, Chip, Link, Table, TableBody, TableCell, TableColumn, TableHeader, TableRow, Tooltip} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faAward, faBook, faCirclePlus, faCopy, faHardDrive, faLayerGroup, faPlus} from "@fortawesome/free-solid-svg-icons";
import DownloadFile from "../../images/DownloadFile.svg.tsx";

export default function InstancesList()
{
    return (
        <div className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 w-full mx-2 h-[calc(100dvh_-_330px)] min-h-[300px] overflow-y-auto grow relative"}>
            <div className={"flex flex-row w-full items-center"}>
                <p className={"text-lg font-semibold"}>Instances</p>

                <Tooltip content={"Create a new custom instance, this will be used to create new servers."}>
                    <Button
                        color={"primary"}
                        endContent={<FontAwesomeIcon icon={faPlus}/>}
                        className={"ml-auto"}
                    >
                        Custom Instance
                    </Button>
                </Tooltip>
            </div>

            <Table
                selectionMode={"multiple"}
                className={"mt-8 max-h-full overflow-y-auto"}
                color={"primary"}
                isHeaderSticky
                classNames={{
                    tr: "data-[odd]:bg-neutral-800 data-[hover]:bg-neutral-700",
                    th: "bg-neutral-700/50 backdrop-blur-lg",
                    thead: "bg-neutral-700/50 backdrop-blur-lg"
                }}
                checkboxesProps={{
                    className: "w-0"
                }}
                isStriped
                removeWrapper
            >
                <TableHeader>
                    <TableColumn>Name</TableColumn>
                    <TableColumn><FontAwesomeIcon icon={faAward} className={"mr-1"}/> Loader</TableColumn>
                    <TableColumn><FontAwesomeIcon icon={faBook} className={"mr-1"}/> Minecraft Version</TableColumn>
                    <TableColumn><FontAwesomeIcon icon={faLayerGroup} className={"mr-1"}/> Source</TableColumn>
                    <TableColumn><FontAwesomeIcon icon={faHardDrive} className={"mr-1"}/> Servers</TableColumn>
                    <TableColumn className={"w-0"}>Actions</TableColumn>
                </TableHeader>
                <TableBody>
                    {Array.from({length: 20}).map((_, index) =>
                    {
                        return (
                            <TableRow key={index}>
                                <TableCell>All The Mods 6</TableCell>
                                <TableCell><Link>Forge</Link></TableCell>
                                <TableCell><Link>1.20.4</Link></TableCell>
                                <TableCell><Link>Modrinth</Link></TableCell>
                                <TableCell><Chip color={"success"} variant={"flat"}>4</Chip></TableCell>
                                <TableCell>
                                    <div className={"flex flex-row items-center"}>
                                        <Tooltip content={"Create Server"}>
                                            <Button variant={"light"} className={"min-w-0 w-2 text-neutral-400 data-[hover]:text-foreground"}> <FontAwesomeIcon icon={faCirclePlus}/> </Button>
                                        </Tooltip>
                                        <Tooltip content={`Duplicate Instance`}>
                                            <Button variant={"light"} className={"min-w-0 w-2 data-[hover]:text-foreground text-neutral-400"}> <FontAwesomeIcon icon={faCopy}/> </Button>
                                        </Tooltip>
                                        <Tooltip content={"Download Instance"}>
                                            <Button variant={"light"} className={"min-w-0 w-2 text-neutral-400 data-[hover]:text-foreground"}> <DownloadFile/> </Button>
                                        </Tooltip>
                                    </div>
                                </TableCell>
                            </TableRow>
                        );
                    })}
                </TableBody>
            </Table>

        </div>
    );
}