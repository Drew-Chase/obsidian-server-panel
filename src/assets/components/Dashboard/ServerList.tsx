import {Avatar, Button, Chip, cn, Link, Modal, ModalBody, ModalContent, ModalFooter, ModalHeader, Table, TableBody, TableCell, TableColumn, TableHeader, TableRow} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faCalendar, faCheckCircle, faCheckSquare, faCircle, faClock, faEllipsis, faLayerGroup, faPlay, faPlus, faStop, faUser} from "@fortawesome/free-solid-svg-icons";
import DownloadFile from "../../images/DownloadFile.svg.tsx";
import {useScreenSize} from "../../providers/ScreenSizeProvider.tsx";
import {useEffect, useState} from "react";
import Server, {ServerStatus} from "../../ts/servers.ts";
import {useSelectedServer} from "../../providers/SelectedServerProvider.tsx";
import OTooltip from "../Extends/OTooltip.tsx";

export default function ServerList()
{
    const {width} = useScreenSize();
    const [isActionModalOpen, setIsActionModalOpen] = useState(false);
    const [servers, setServers] = useState<Server[]>([]);

    const {setSelectedServerId} = useSelectedServer();

    useEffect(() =>
    {
        Server.list().then(setServers);
    }, []);


    return (
        <div className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 w-full mx-2 h-[calc(100dvh_-_330px)] min-h-[300px] overflow-y-auto grow relative"}>
            <div className={"flex flex-row w-full items-center"}>
                <p className={"text-lg font-semibold"}>Servers</p>
                <OTooltip content={"Create a new server"}>
                    <Button
                        color={"primary"}
                        endContent={<FontAwesomeIcon icon={faPlus}/>}
                        className={"ml-auto"}
                        as={Link}
                        href={"/app/create-server/"}
                    >
                        Create server
                    </Button>
                </OTooltip>
            </div>

            <Modal size={"2xl"} isOpen={isActionModalOpen} onClose={() => setIsActionModalOpen(false)}>
                <ModalContent>
                    {onClose => (<>
                        <ModalHeader>Actions</ModalHeader>
                        <ModalBody>
                            <div className={"flex flex-col w-full gap-4"}>
                                <Button className={"h-16 text-neutral-400 data-[hover]:text-foreground"}>View Server</Button>
                                <Button className={cn("h-16 data-[hover]:text-foreground", true ? "text-neutral-400" : "text-danger")}> Stop Server </Button>
                                <Button className={"h-16 text-neutral-400 data-[hover]:text-foreground"}> Download Server </Button>
                            </div>
                        </ModalBody>
                        <ModalFooter>
                            <Button onClick={onClose}>Cancel</Button>
                        </ModalFooter>
                    </>)}
                </ModalContent>
            </Modal>

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
                    className: "w-6"
                }}
                isStriped
                removeWrapper
                aria-label="List of Servers"
            >
                <TableHeader>
                    <TableColumn aria-label="Server Name">Name</TableColumn>
                    <TableColumn aria-label="Server Owner" hidden={width < 1120}><FontAwesomeIcon icon={faUser} className={"mr-1"}/> Owner</TableColumn>
                    <TableColumn aria-label="Creation Date" hidden={width < 1500}><FontAwesomeIcon icon={faCalendar} className={"mr-1"}/> Creation Date</TableColumn>
                    <TableColumn aria-label="Server Uptime" hidden={width < 1300}><FontAwesomeIcon icon={faClock} className={"mr-1"}/> Uptime</TableColumn>
                    <TableColumn aria-label="Server Instance" hidden={width < 1000}><FontAwesomeIcon icon={faLayerGroup} className={"mr-1"}/> Instance</TableColumn>
                    <TableColumn aria-label="Server Status"><FontAwesomeIcon icon={faCheckSquare} className={"mr-1"}/> Status</TableColumn>
                    <TableColumn className={"w-0"} aria-label="Actions">Actions</TableColumn>
                </TableHeader>
                <TableBody>
                    {servers.map((server, index) =>
                    {
                        const isRunning = server.status === ServerStatus.Running;
                        const statusColor = server.status === ServerStatus.Running ? "success" : server.status === ServerStatus.Crashed || server.status === ServerStatus.Offline ? "danger" : "default";
                        return (
                            <TableRow key={index} className={"flex-nowrap"}>
                                <TableCell>
                                    <Link href={"/app/server/"} onPressStart={() => setSelectedServerId(server.id)}>
                                        <div className={"flex flex-row items-center gap-2"}><Avatar src={`/api/server/${server.id}/icon`}/> {server.name}</div>
                                    </Link>
                                </TableCell>
                                <TableCell hidden={width < 1120}>{server.owner}</TableCell>
                                <TableCell hidden={width < 1500}>{server.created_at.toDateString()}</TableCell>
                                <TableCell hidden={width < 1300}>1y 6m 24d 14h 30m</TableCell>
                                <TableCell hidden={width < 1000}>{server.instance ? <Link className={"max-w-[120px] truncate"}>{server.instance}</Link> : "Custom"}</TableCell>
                                <TableCell><Chip color={statusColor} variant={"flat"}> <FontAwesomeIcon icon={faCircle} width={5}/> {server.status}</Chip></TableCell>
                                <TableCell>
                                    {(width < 900) ?
                                        (
                                            <Button
                                                radius={"full"}
                                                className={"min-w-0 w-11 h-11 text-neutral-400 data-[hover]:text-foreground"}
                                                aria-label={"Open Actions"}
                                                onClick={() => setIsActionModalOpen(true)}
                                            >
                                                <FontAwesomeIcon icon={faEllipsis}/>
                                            </Button>
                                        ) : (
                                            <div className={"flex flex-row items-center"}>
                                                <OTooltip content={"Select Server"}>
                                                    <Button variant={"light"} className={"min-w-0 w-2 text-neutral-400 data-[hover]:text-foreground"} onClick={() => setSelectedServerId(server.id)}> <FontAwesomeIcon icon={faCheckCircle}/> </Button>
                                                </OTooltip>
                                                <OTooltip content={`${isRunning ? "Start" : "Stop"} the Server`} color={isRunning ? "default" : "danger"}>
                                                    <Button variant={"light"} className={cn("min-w-0 w-2 data-[hover]:text-foreground", isRunning ? "text-neutral-400" : "text-danger")}> <FontAwesomeIcon icon={isRunning ? faPlay : faStop}/> </Button>
                                                </OTooltip>
                                                <OTooltip content={"Download Server"}>
                                                    <Button variant={"light"} className={"min-w-0 w-2 text-neutral-400 data-[hover]:text-foreground"}> <DownloadFile/> </Button>
                                                </OTooltip>
                                            </div>
                                        )}
                                </TableCell>
                            </TableRow>
                        );
                    })}
                </TableBody>
            </Table>

        </div>
    );
}