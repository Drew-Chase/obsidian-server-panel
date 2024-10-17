import {Avatar, Button, Chip, Link, Tooltip} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faCalendar, faDownload, faHeart} from "@fortawesome/free-solid-svg-icons";
import {Instance} from "../../ts/instances.ts";
import Conversions from "../../ts/conversions.ts";


export default function DiscoverModItem(instance: Instance)
{
    return (
        <div className={"flex flex-col shrink-0 p-4 justify-between"}>
            <div className={"flex flex-row shrink-0"}>
                {instance.icon && <Avatar src={instance.icon} radius={"sm"} size={"lg"} className={"mr-4"}/>}
                <div className={"flex flex-col items-start"}>
                    <div className={"flex flex-row items-center"}>
                        <p className={"text-large font-bold mr-2"}>{instance.name}</p>
                        <p className={"text-tiny opacity-70"}>By {instance.author}</p>
                    </div>
                    <p className={"opacity-70"}>{instance.description}</p>
                </div>
            </div>
            <div className={"flex flex-row mt-4 gap-3 items-center"}>
                <div className={"flex flex-row gap-3 mr-auto"}>
                    <Tooltip content={`${instance.downloads.toLocaleString("en-us")} Downloads`} closeDelay={0}>
                        <Chip radius={"sm"} color={"default"} variant={"flat"}><FontAwesomeIcon icon={faDownload} className={"mr-3"}/> {Conversions.formatLargeNumber(instance.downloads, 1)}</Chip>
                    </Tooltip>
                    <Chip radius={"sm"} color={"default"} variant={"flat"}><FontAwesomeIcon icon={faHeart} className={"mr-3 text-red-600"}/>{Conversions.formatLargeNumber(instance.likes, 1)}</Chip>
                    {instance.last_updated && (
                        <Chip radius={"sm"} color={"default"} variant={"flat"}><FontAwesomeIcon icon={faCalendar} className={"mr-3"}/>{Conversions.formatTimeClosestRelative(instance.last_updated)}</Chip>
                    )}
                </div>
                <Button color={"primary"} variant={"flat"} startContent={<FontAwesomeIcon icon={faDownload} className={"mr-1"}/>}>Install</Button>
                <Button href={instance.project_url} as={Link} variant={"flat"} showAnchorIcon isExternal target={"_blank"}>View on {instance.platform}</Button>
            </div>
        </div>
    );
}