import {Button, Chip, Table, TableBody, TableCell, TableColumn, TableHeader, TableRow} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faUserCheck, faUserSlash, faX} from "@fortawesome/free-solid-svg-icons";
import OTooltip from "../Extends/OTooltip";


enum ResponseType
{
    Success,
    WhitelistError,
    BanError,
    TimedoutError,
    UnknownError
}

export default function ExtendedRecentConnectionsStat()
{
    return (
        <div className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 w-full mx-2 overflow-y-auto"}>
            <p className={"text-lg font-semibold mr-auto mb-8"}>Recent Connections</p>

            <Table
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
                aria-label="Recent Connections Table"
            >
                <TableHeader>
                    <TableColumn>Player Name</TableColumn>
                    <TableColumn>Response</TableColumn>
                    <TableColumn>Timestamp</TableColumn>
                    <TableColumn className={"w-0"}>Actions</TableColumn>
                </TableHeader>

                <TableBody>
                    {Array.from({length: 20}, (_, i) =>
                        {
                            const random = Math.floor(Math.random() * 5);
                            let responseType: ResponseType;
                            switch (random)
                            {
                                case 1:
                                    responseType = ResponseType.WhitelistError;
                                    break;
                                case 2:
                                    responseType = ResponseType.BanError;
                                    break;
                                case 4:
                                    responseType = ResponseType.TimedoutError;
                                    break;
                                case 5:
                                    responseType = ResponseType.UnknownError;
                                    break;
                                default:
                                    responseType = ResponseType.Success;
                                    break;
                            }

                            let chip = <Chip color={"success"} variant={"flat"}>Success</Chip>;
                            switch (responseType)
                            {
                                case ResponseType.WhitelistError:
                                    chip = <Chip color={"warning"} variant={"flat"}>Whitelist error</Chip>;
                                    break;
                                case ResponseType.BanError:
                                    chip = <Chip color={"danger"} variant={"flat"}>Ban error</Chip>;
                                    break;
                                case ResponseType.TimedoutError:
                                    chip = <Chip color={"danger"} variant={"flat"}>Timed out</Chip>;
                                    break;
                                case ResponseType.UnknownError:
                                    chip = <Chip color={"danger"} variant={"flat"}>Unknown error</Chip>;
                                    break;
                            }


                            return (
                                <TableRow key={i}>
                                    <TableCell>Shroototem</TableCell>
                                    <TableCell>{chip}</TableCell>
                                    <TableCell>2:36:50 PM</TableCell>
                                    <TableCell>
                                        {responseType === ResponseType.Success ? (
                                            <div className={"flex flex-row items-center"}>
                                                <OTooltip content={"Kick this Player"}>
                                                    <Button className={"min-w-0"} variant={"light"} color={"danger"}><FontAwesomeIcon icon={faX}/></Button>
                                                </OTooltip>
                                                <OTooltip content={"Ban this Player"}>
                                                    <Button className={"min-w-0"} variant={"light"} color={"danger"}><FontAwesomeIcon icon={faUserSlash}/></Button>
                                                </OTooltip>
                                            </div>
                                        ) : responseType === ResponseType.WhitelistError ? (
                                            <div className={"flex flex-row items-center"}>
                                                <OTooltip content={"Add to whitelist"}>
                                                    <Button className={"min-w-0"} variant={"light"}><FontAwesomeIcon icon={faUserCheck}/></Button>
                                                </OTooltip>
                                            </div>
                                        ) : responseType === ResponseType.BanError ? (
                                            <div className={"flex flex-row items-center"}>
                                                <OTooltip content={"Unban Player"}>
                                                    <Button className={"min-w-0"} variant={"light"}><FontAwesomeIcon icon={faUserCheck}/></Button>
                                                </OTooltip>
                                            </div>
                                        ) : (<></>)}
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