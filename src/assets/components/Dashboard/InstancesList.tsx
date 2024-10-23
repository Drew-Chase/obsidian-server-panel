import {Button, Chip, Link, Modal, ModalBody, ModalContent, ModalFooter, ModalHeader, Table, TableBody, TableCell, TableColumn, TableHeader, TableRow, Tooltip} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faAward, faBook, faCirclePlus, faCopy, faEllipsis, faHardDrive, faLayerGroup, faPlus} from "@fortawesome/free-solid-svg-icons";
import DownloadFile from "../../images/DownloadFile.svg.tsx";
import {useScreenSize} from "../../providers/ScreenSizeProvider.tsx";
import {useState} from "react";

export default function InstancesList()
{
    const {width} = useScreenSize();
    const [isActionModalOpen, setIsActionModalOpen] = useState(false);
    return (
        <div className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 w-full mx-2 h-[calc(100dvh_-_330px)] min-h-[300px] overflow-y-auto grow relative"}>
            <Modal size={"2xl"} isOpen={isActionModalOpen} onClose={() => setIsActionModalOpen(false)}>
                <ModalContent>
                    {onClose => (<>
                        <ModalHeader>Actions</ModalHeader>
                        <ModalBody>
                            <div className={"flex flex-col w-full gap-4"}>
                                <Button className={"h-16 text-neutral-400 data-[hover]:text-foreground"} aria-label={"Create Server"}> Create Server </Button>
                                <Button className={"h-16 data-[hover]:text-foreground text-neutral-400"} aria-label={"Duplicate Instance"}> Duplicate Instance </Button>
                                <Button className={"h-16 text-neutral-400 data-[hover]:text-foreground"} aria-label={"Download Instance"}> Download Instance </Button>
                            </div>
                        </ModalBody>
                        <ModalFooter>
                            <Button onClick={onClose}>Cancel</Button>
                        </ModalFooter>
                    </>)}
                </ModalContent>
            </Modal>
            <div className={"flex flex-row w-full items-center"}>
                <p className={"text-lg font-semibold"}>Instances</p>

                <Tooltip content={"Create a new custom instance, this will be used to create new servers."}>
                    <Button
                        color={"primary"}
                        endContent={<FontAwesomeIcon icon={faPlus}/>}
                        className={"ml-auto"}
                        aria-label={"Create Custom Instance"}
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
                aria-label={"Instances Table"}
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
            >
                <TableHeader>
                    <TableColumn>Name</TableColumn>
                    <TableColumn hidden={width < 900}><FontAwesomeIcon icon={faAward} className={"mr-1"}/> Loader</TableColumn>
                    <TableColumn hidden={width < 1020}><FontAwesomeIcon icon={faBook} className={"mr-1"}/> Minecraft Version</TableColumn>
                    <TableColumn hidden={width < 900}><FontAwesomeIcon icon={faLayerGroup} className={"mr-1"}/> Source</TableColumn>
                    <TableColumn hidden={width < 1120}><FontAwesomeIcon icon={faHardDrive} className={"mr-1"}/> Servers</TableColumn>
                    <TableColumn className={"w-0"}>Actions</TableColumn>
                </TableHeader>
                <TableBody>
                    {Array.from({length: 20}).map((_, index) =>
                    {
                        return (
                            <TableRow key={index}>
                                <TableCell><span className={"max-w-[100px] truncate"}>All The Mods 6</span></TableCell>
                                <TableCell hidden={width < 900}><Link>Forge</Link></TableCell>
                                <TableCell hidden={width < 1020}><Link>1.20.4</Link></TableCell>
                                <TableCell hidden={width < 900}><Link>Modrinth</Link></TableCell>
                                <TableCell hidden={width < 1120}><Chip color={"success"} variant={"flat"}>4</Chip></TableCell>
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
                                                <Tooltip content={"Create Server"}>
                                                    <Button variant={"light"} className={"min-w-0 w-2 text-neutral-400 data-[hover]:text-foreground"} aria-label={"Create Server"}> <FontAwesomeIcon icon={faCirclePlus}/> </Button>
                                                </Tooltip>
                                                <Tooltip content={`Duplicate Instance`}>
                                                    <Button variant={"light"} className={"min-w-0 w-2 data-[hover]:text-foreground text-neutral-400"} aria-label={"Duplicate Instance"}> <FontAwesomeIcon icon={faCopy}/> </Button>
                                                </Tooltip>
                                                <Tooltip content={"Download Instance"}>
                                                    <Button variant={"light"} className={"min-w-0 w-2 text-neutral-400 data-[hover]:text-foreground"} aria-label={"Download Instance"}> <DownloadFile/> </Button>
                                                </Tooltip>
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