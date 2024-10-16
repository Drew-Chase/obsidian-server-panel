import {Avatar, Button, Chip, cn, Link, Table, TableBody, TableCell, TableColumn, TableHeader, TableRow, Tooltip} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faCalendar, faCheckCircle, faCheckSquare, faCircle, faClock, faLayerGroup, faPlay, faPlus, faStop, faUser} from "@fortawesome/free-solid-svg-icons";
import testIcon from "../../images/demo/test-server.png";
import DownloadFile from "../../images/DownloadFile.svg.tsx";

export default function ServerList()
{
    return (
        <div className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 w-full mx-2 h-[calc(100dvh_-_330px)] min-h-[300px] overflow-y-auto grow relative"}>
            <div className={"flex flex-row w-full items-center"}>
                <p className={"text-lg font-semibold"}>Servers</p>
                <Tooltip content={"Create a new server"}>
                    <Button
                        color={"primary"}
                        endContent={<FontAwesomeIcon icon={faPlus}/>}
                        className={"ml-auto"}
                    >
                        Create server
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
                aria-label="List of Servers"
            >
                <TableHeader>
                    <TableColumn aria-label="Server Name">Name</TableColumn>
                    <TableColumn aria-label="Server Owner"><FontAwesomeIcon icon={faUser} className={"mr-1"}/> Owner</TableColumn>
                    <TableColumn aria-label="Creation Date"><FontAwesomeIcon icon={faCalendar} className={"mr-1"}/> Creation Date</TableColumn>
                    <TableColumn aria-label="Server Status"><FontAwesomeIcon icon={faCheckSquare} className={"mr-1"}/> Status</TableColumn>
                    <TableColumn aria-label="Server Instance"><FontAwesomeIcon icon={faLayerGroup} className={"mr-1"}/> Instance</TableColumn>
                    <TableColumn aria-label="Server Uptime"><FontAwesomeIcon icon={faClock} className={"mr-1"}/> Uptime</TableColumn>
                    <TableColumn className={"w-0"} aria-label="Actions">Actions</TableColumn>
                </TableHeader>
                <TableBody>
                    {Array.from({length: 20}).map((_, index) =>
                    {
                        const random = Math.floor(Math.random() * 3);
                        const statusName = random === 0 ? "Online" : random === 1 ? "Offline" : "Restarting";
                        const isRunning = random === 0;
                        const statusColor = random === 0 ? "success" : random === 1 ? "danger" : "warning";
                        return (
                            <TableRow key={index}>
                                <TableCell>
                                    <div className={"flex flex-row items-center gap-2"}><Avatar src={testIcon}/> SMP Server</div>
                                </TableCell>
                                <TableCell>Drew Chase</TableCell>
                                <TableCell>Jan 27, 2024</TableCell>
                                <TableCell><Chip color={statusColor} variant={"flat"}> <FontAwesomeIcon icon={faCircle} width={5}/> {statusName}</Chip></TableCell>
                                <TableCell><Link>All the Mods 6</Link></TableCell>
                                <TableCell>1y 6m 24d 14h 30m</TableCell>
                                <TableCell>
                                    <div className={"flex flex-row items-center"}>
                                        <Tooltip content={"Select Server"}>
                                            <Button variant={"light"} className={"min-w-0 w-2 text-neutral-400 data-[hover]:text-foreground"}> <FontAwesomeIcon icon={faCheckCircle}/> </Button>
                                        </Tooltip>
                                        <Tooltip content={`${isRunning ? "Start" : "Stop"} the Server`} color={isRunning ? "default" : "danger"}>
                                            <Button variant={"light"} className={cn("min-w-0 w-2 data-[hover]:text-foreground", isRunning ? "text-neutral-400" : "text-danger")}> <FontAwesomeIcon icon={isRunning ? faPlay : faStop}/> </Button>
                                        </Tooltip>
                                        <Tooltip content={"Download Server"}>
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