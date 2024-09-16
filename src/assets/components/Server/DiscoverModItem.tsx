import {Avatar, Button, Chip, Link} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faCalendar, faDownload, faHeart} from "@fortawesome/free-solid-svg-icons";

interface ModItemProps
{
    icon?: string;
    name: string;
    description?: string;
    author?: string;
}

export default function DiscoverModItem(props: ModItemProps)
{
    return (
        <div className={"flex flex-col shrink-0 p-4 justify-between"}>
            <div className={"flex flex-row shrink-0"}>
                {props.icon && <Avatar src={props.icon} radius={"sm"} size={"lg"} className={"mr-4"}/>}
                <div className={"flex flex-col items-start"}>
                    <div className={"flex flex-row items-center"}>
                        <p className={"text-large font-bold mr-2"}>{props.name}</p>
                        <p className={"text-tiny opacity-70"}>By {props.author}</p>
                    </div>
                    <p className={"opacity-70"}>{props.description}</p>
                </div>
            </div>
            <div className={"flex flex-row mt-4 gap-3 items-center"}>
                <div className={"flex flex-row gap-3 mr-auto"}>
                    <Chip radius={"sm"} color={"default"} variant={"flat"}><FontAwesomeIcon icon={faDownload} className={"mr-3"}/> 20.44M</Chip>
                    <Chip radius={"sm"} color={"default"} variant={"flat"}><FontAwesomeIcon icon={faHeart} className={"mr-3 text-red-600"}/>16.6K</Chip>
                    <Chip radius={"sm"} color={"default"} variant={"flat"}><FontAwesomeIcon icon={faCalendar} className={"mr-3"}/>4 hours ago</Chip>
                </div>
                <Button color={"primary"} variant={"flat"} startContent={<FontAwesomeIcon icon={faDownload} className={"mr-1"}/>}>Install</Button>
                <Button href={"https://modrinth.com/mod/sodium"} as={Link} variant={"flat"} showAnchorIcon isExternal>View on Modrinth</Button>
            </div>
        </div>
    );
}