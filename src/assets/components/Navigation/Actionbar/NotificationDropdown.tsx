import {Badge, Button, Chip, PopoverContent, PopoverTrigger, Tab, Tabs} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faBell} from "@fortawesome/free-solid-svg-icons";
import OPopover from "../../Extends/OPopover.tsx";
import OTooltip from "../../Extends/OTooltip.tsx";

export default function NotificationDropdown()
{
    return (

        <OPopover>
            <PopoverTrigger>
                <div>
                    <Badge content={"12"} showOutline={false} variant={"shadow"} hidden={false}>
                        <div className={"p-1 cursor-pointer"}>
                            <OTooltip content={"Notifications"}>
                                <FontAwesomeIcon icon={faBell} fontSize={18} className={"cursor-pointer text-white opacity-50 hover:opacity-100 transition-all text-xl"}/>
                            </OTooltip>
                        </div>
                    </Badge>
                </div>
            </PopoverTrigger>
            <PopoverContent>
                <div className={"flex flex-col min-w-[380px] p-0"}>
                    <div className="flex w-full items-center justify-between px-5 py-2">
                        <div className="inline-flex items-center gap-1">
                            <h4 className="inline-block align-middle text-large font-medium">Notifications</h4>
                            <Chip>12</Chip>
                        </div>
                        <Button
                            variant={"light"}
                            color={"primary"}
                        >
                            Mark all as read
                        </Button>
                    </div>
                    <Tabs variant={"underlined"} color={"primary"}>
                        <Tab title={<><span className={"mr-2"}>All</span> <Chip>9</Chip></>}/>
                        <Tab title={<><span className={"mr-2"}>Unread</span> <Chip>3</Chip></>}/>
                        <Tab title={<><span className={"mr-2"}>Archived</span> <Chip hidden>0</Chip></>}/>
                    </Tabs>
                </div>
            </PopoverContent>
        </OPopover>
    );
}