import {Button, Divider, Dropdown, DropdownItem, DropdownMenu, DropdownTrigger} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faChevronDown} from "@fortawesome/free-solid-svg-icons";

export default function StopButton()
{
    return (
        <Button color={"danger"} className={"pr-0"} endContent={
            <div className={"flex flex-row items-center"}>
                <Divider orientation={"vertical"} className={"shrink-0 grow h-4 bg-white/50 mr-2"}/>
                <Dropdown>
                    <DropdownTrigger>
                        <Button variant={"light"} className={"min-w-0 bg-neutral-800/20"}><FontAwesomeIcon icon={faChevronDown}/></Button>
                    </DropdownTrigger>
                    <DropdownMenu>
                        <DropdownItem key="stop-server">Stop Server</DropdownItem>
                        <DropdownItem key="restart-server">Restart Server</DropdownItem>
                        <DropdownItem key="kill-server">Kill Server</DropdownItem>
                        <DropdownItem key="stop-and-backup">Stop and Backup</DropdownItem>
                        <DropdownItem key="restart-and-backup">Restart and Backup</DropdownItem>
                    </DropdownMenu>
                </Dropdown>
            </div>
        }>
            <span className={"w-full text-center"}>Stop Server</span>
        </Button>
    );
}