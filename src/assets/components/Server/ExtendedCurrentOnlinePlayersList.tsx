import {Button, Chip, Select, SelectItem, Table, TableBody, TableCell, TableColumn, TableHeader, TableRow, Tooltip} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faUserSlash, faX} from "@fortawesome/free-solid-svg-icons";


export default function ExtendedCurrentOnlinePlayersList()
{
    return (
        <div className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 w-full mx-2 overflow-y-auto"}>
            <div className={"flex flex-row items-center mb-6"}>
                <p className={"text-lg font-semibold mr-auto"}>Players List</p>
                <Select
                    label={"Filter"}
                    className={"w-[200px]"}
                    classNames={{
                        trigger: "bg-neutral-700"
                    }}
                    defaultSelectedKeys={["online"]}
                >
                    <SelectItem key={"all"}>All Players</SelectItem>
                    <SelectItem key={"online"}>Online Players</SelectItem>
                    <SelectItem key={"offline"}>Offline Players</SelectItem>
                </Select>
            </div>

            <Table
                aria-label="Players list table"
                isStriped
                removeWrapper
                isHeaderSticky
                className={"max-h-[500px] h-full overflow-y-auto"}
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
                    <TableColumn>Player Name</TableColumn>
                    <TableColumn>Role</TableColumn>
                    <TableColumn>Status</TableColumn>
                    <TableColumn>Joined</TableColumn>
                    <TableColumn className={"w-0"}>Actions</TableColumn>
                </TableHeader>

                <TableBody>
                    {Array.from({length: 20}, (_, i) =>
                        {
                            const random = Math.random();
                            const isAdmin = random > 0.5;
                            return (
                                <TableRow key={i}>
                                    <TableCell>Shroototem</TableCell>
                                    <TableCell> <Chip color={isAdmin ? "primary" : "default"} variant={"flat"}>{isAdmin ? "Operator" : "Normal"}</Chip> </TableCell>
                                    <TableCell> <Chip color={isAdmin ? "primary" : "default"} variant={"flat"}>{isAdmin ? "Online" : "Offline"}</Chip> </TableCell>
                                    <TableCell>2:36:50 PM</TableCell>
                                    <TableCell>
                                        <div className={"flex flex-row items-center"}>
                                            <Tooltip content={"Kick this Player"}>
                                                <Button className={"min-w-0"} variant={"light"} color={"danger"}><FontAwesomeIcon icon={faX}/></Button>
                                            </Tooltip>
                                            <Tooltip content={"Ban this Player"}>
                                                <Button className={"min-w-0"} variant={"light"} color={"danger"}><FontAwesomeIcon icon={faUserSlash}/></Button>
                                            </Tooltip>
                                        </div>
                                    </TableCell>
                                </TableRow>
                            );
                        }
                    )
                    }
                </TableBody>

            </Table>
        </div>
    );
}