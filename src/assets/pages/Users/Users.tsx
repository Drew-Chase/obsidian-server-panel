import {Button, Chip, Table, TableBody, TableCell, TableColumn, TableHeader, TableRow} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faPlus, faTrash, faUserEdit} from "@fortawesome/free-solid-svg-icons";
import {setTitle} from "../../../main.tsx";
import OInput from "../../components/Extends/OInput.tsx";
import OTooltip from "../../components/Extends/OTooltip.tsx";

export default function Users()
{
    setTitle("Users");
    return (
        <div className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 w-full mx-2 overflow-y-auto"}>
            <p className={"text-lg font-semibold mr-auto mb-8"}>Manage Users</p>

            <div className={"flex flex-row items-center mb-6"}>
                <OInput
                    label={"Username"}
                    placeholder={"Add a managed user"}
                    className={"w-full"}
                />
                <Button className={"ml-4"} endContent={<FontAwesomeIcon icon={faPlus}/>}>Add</Button>
            </div>

            <Table
                aria-label="Users Management Table"
                isStriped
                removeWrapper
                isHeaderSticky
                className={"max-h-[calc(100dvh_-_260px)] h-screen overflow-y-auto"}
                color={"primary"}
                classNames={{
                    tr: "data-[odd]:bg-neutral-800 data-[hover]:bg-neutral-700",
                    th: "bg-neutral-700/50 backdrop-blur-lg",
                    thead: "bg-neutral-700/50 backdrop-blur-lg"
                }}
                checkboxesProps={{
                    className: "w-0"
                }}

            >
                <TableHeader>
                    <TableColumn>Username</TableColumn>
                    <TableColumn>Group/Role</TableColumn>
                    <TableColumn className={"w-0"}>Actions</TableColumn>
                </TableHeader>

                <TableBody>
                    {
                        Array.from({length: 5}, (_, i) =>
                            {
                                return (
                                    <TableRow key={i}>
                                        <TableCell>Shroototem</TableCell>
                                        <TableCell><Chip color={"primary"} variant={"flat"}>Administrator</Chip></TableCell>
                                        <TableCell>
                                            <div className={"flex flex-row items-center"}>
                                                <OTooltip content={"Edit users group"}>
                                                    <Button className={"min-w-0"} variant={"light"}><FontAwesomeIcon icon={faUserEdit}/></Button>
                                                </OTooltip>
                                                <OTooltip content={"Delete user"}>
                                                    <Button className={"min-w-0"} variant={"light"} color={"danger"}><FontAwesomeIcon icon={faTrash}/></Button>
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