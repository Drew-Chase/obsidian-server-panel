import {Button, Input, Table, TableBody, TableCell, TableColumn, TableHeader, TableRow} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faPlus, faUserCheck} from "@fortawesome/free-solid-svg-icons";
import OTooltip from "../Extends/OTooltip.tsx";

export default function BanPlayersList()
{
    return (
        <div className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 w-full mx-2 overflow-y-auto"}>
            <p className={"text-lg font-semibold mr-auto mb-8"}>Ban Players</p>

            <div className={"flex flex-row items-center mb-6"}>
                <Input
                    label={"Player Name"}
                    placeholder={"Add a player to the ban list"}
                    className={"w-full"}
                    classNames={{
                        inputWrapper: "bg-neutral-700"
                    }}
                />
                <Button className={"ml-4"} endContent={<FontAwesomeIcon icon={faPlus}/>}>Ban</Button>
            </div>

            <Table
                isStriped
                removeWrapper
                isHeaderSticky
                className={"max-h-[500px] h-full overflow-y-auto"}
                color={"primary"}
                aria-label="Ban Players Table"
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
                    <TableColumn className={"w-0"}>Actions</TableColumn>
                </TableHeader>

                <TableBody>
                    {Array.from({length: 20}, (_, i) =>
                        {
                            return (
                                <TableRow key={i}>
                                    <TableCell>Shroototem</TableCell>
                                    <TableCell>
                                        <div className={"flex flex-row items-center"}>
                                            <OTooltip content={"Unban Player"}>
                                                <Button className={"min-w-0"} variant={"light"} color={"danger"}><FontAwesomeIcon icon={faUserCheck}/></Button>
                                            </OTooltip>
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