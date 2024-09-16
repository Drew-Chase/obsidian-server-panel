import {Button, Checkbox, CheckboxGroup, Input, Modal, ModalBody, ModalContent, ModalFooter, ModalHeader, ScrollShadow, Table, TableBody, TableCell, TableColumn, TableHeader, TableRow, Tooltip, useDisclosure} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faPlus, faTrash, faUserEdit} from "@fortawesome/free-solid-svg-icons";
import {useState} from "react";
import {setTitle} from "../../../main.tsx";

export default function UserGroups()
{
    setTitle("User Groups");
    const {onOpen, isOpen, onOpenChange} = useDisclosure();
    const [isAdmin, setIsAdmin] = useState(false);
    return (
        <>
            <Modal onOpenChange={onOpenChange} isOpen={isOpen} scrollBehavior={"inside"}>
                <ModalContent>
                    {onClose => (
                        <>
                            <ModalHeader>Create New Group</ModalHeader>
                            <ModalBody className={"flex flex-col"}>
                                <Input
                                    label={"Group Name"}
                                    placeholder={"Enter the group name"}
                                    className={"w-full"}
                                />

                                <Checkbox value={"administrator"} onValueChange={setIsAdmin}>Administrator</Checkbox>
                                <CheckboxGroup label={"Group Permissions"} color={"primary"} isDisabled={isAdmin}>
                                    <ScrollShadow className={"max-h-[244px] overflow-y-auto flex flex-col"}>
                                        <Checkbox value={"can-create-servers"}>Can Create Servers</Checkbox>
                                        <Checkbox value={"can-view-all-servers"}>Can View All Servers</Checkbox>
                                        <Checkbox value={"can-modify-servers"}>Can Modify Servers</Checkbox>
                                        <Checkbox value={"can-create-files"}>Can Create Files</Checkbox>
                                        <Checkbox value={"can-delete-files"}>Can Delete Files</Checkbox>
                                        <Checkbox value={"can-download-files"}>Can Download Files</Checkbox>
                                        <Checkbox value={"can-download-servers"}>Can Download Servers</Checkbox>
                                        <Checkbox value={"can-view-logs"}>Can View Logs</Checkbox>
                                        <Checkbox value={"can-view-backups"}>Can View Backups</Checkbox>
                                        <Checkbox value={"can-create-backups"}>Can Create Backups</Checkbox>
                                        <Checkbox value={"can-delete-backups"}>Can Delete Backups</Checkbox>
                                        <Checkbox value={"can-view-players"}>Can View Players</Checkbox>
                                        <Checkbox value={"can-kick-players"}>Can Kick Players</Checkbox>
                                        <Checkbox value={"can-ban-players"}>Can Ban Players</Checkbox>
                                        <Checkbox value={"can-view-settings"}>Can View Settings</Checkbox>
                                        <Checkbox value={"can-edit-settings"}>Can Edit Settings</Checkbox>
                                        <Checkbox value={"can-view-groups"}>Can View Groups</Checkbox>
                                        <Checkbox value={"can-edit-groups"}>Can Edit Groups</Checkbox>
                                        <Checkbox value={"can-view-users"}>Can View Users</Checkbox>
                                    </ScrollShadow>
                                </CheckboxGroup>

                            </ModalBody>

                            <ModalFooter className={"flex flex-row"}>
                                <Button onClick={onClose} variant={"light"}>Cancel</Button>
                                <Button onClick={onClose} color={"primary"}>Create</Button>
                            </ModalFooter>
                        </>
                    )}
                </ModalContent>
            </Modal>
            <div className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 w-full mx-2 overflow-y-auto"}>
                <div className={"flex flex-row items-center mb-6"}>
                    <p className={"text-lg font-semibold mr-auto"}>Manage User Groups</p>
                    <Button className={"ml-4"} endContent={<FontAwesomeIcon icon={faPlus}/>} onClick={onOpen}>Add New Group</Button>
                </div>

                <Table
                    isStriped
                    removeWrapper
                    isHeaderSticky
                    className={"h-[calc(100dvh_-_200px)] max-h-screen overflow-y-auto"}
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
                        <TableColumn>Group Name</TableColumn>
                        <TableColumn className={"w-0"}>Actions</TableColumn>
                    </TableHeader>

                    <TableBody>
                        {Array.from({length: 5}, (_, i) =>
                            {
                                return (
                                    <TableRow key={i}>
                                        <TableCell>Administrator</TableCell>
                                        <TableCell>
                                            <div className={"flex flex-row items-center"}>
                                                <Tooltip content={"Edit users group"}>
                                                    <Button className={"min-w-0"} variant={"light"}><FontAwesomeIcon icon={faUserEdit}/></Button>
                                                </Tooltip>
                                                <Tooltip content={"Delete user"}>
                                                    <Button className={"min-w-0"} variant={"light"} color={"danger"}><FontAwesomeIcon icon={faTrash}/></Button>
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
        </>
    );
}
