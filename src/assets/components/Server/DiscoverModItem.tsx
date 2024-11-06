import {Avatar, Button, Chip, Image, Link} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faCalendar, faDownload, faHeart} from "@fortawesome/free-solid-svg-icons";
import {Instance, Modloader} from "../../ts/instances.ts";
import Conversions from "../../ts/conversions.ts";
import ForgeIcon from "../../images/ForgeIcon.svg.tsx";
import FabricIcon from "../../images/fabric-logo.png";
import MinecraftIcon from "../../images/minecraft-chip-logo.png";
import OTooltip from "../Extends/OTooltip.tsx";


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
                    <OTooltip content={`${instance.downloads.toLocaleString("en-us")} Downloads`}>
                        <Chip radius={"sm"} color={"default"} variant={"flat"}><FontAwesomeIcon icon={faDownload} className={"mr-3"}/> {Conversions.formatLargeNumber(instance.downloads, 1)}</Chip>
                    </OTooltip>
                    <Chip radius={"sm"} color={"default"} variant={"flat"}><FontAwesomeIcon icon={faHeart} className={"mr-3 text-red-600"}/>{Conversions.formatLargeNumber(instance.likes, 1)}</Chip>
                    {instance.last_updated && (
                        <OTooltip content={instance.last_updated.toLocaleString()}>
                            <Chip radius={"sm"} color={"default"} variant={"flat"}><FontAwesomeIcon icon={faCalendar} className={"mr-3"}/>{Conversions.formatTimeClosestRelative(instance.last_updated)}</Chip>
                        </OTooltip>
                    )}
                    <Chip radius={"sm"} color={"default"} variant={"flat"}>
                        <span className={"flex flex-row gap-1 items-center"}>

                        {(() =>
                        {

                            switch (instance.modloader)
                            {
                                case Modloader.FABRIC:
                                    return <Image src={FabricIcon} width={20} height={20}/>;
                                case Modloader.FORGE:
                                    return <ForgeIcon size={20}/>;
                                default:
                                    break;
                            }

                            return <></>;
                        })()}
                            <p>{instance.modloader}</p>
                        </span>
                    </Chip>
                    <Chip radius={"sm"} color={"default"} variant={"flat"}>
                        <span className={"flex flex-row gap-2 items-center"}>
                            <Image src={MinecraftIcon} width={16} height={16} radius={"none"}/>{instance.game_versions[0]}
                        </span>
                    </Chip>
                </div>
                <Button color={"primary"} variant={"flat"} startContent={<FontAwesomeIcon icon={faDownload} className={"mr-1"}/>}>Install</Button>
                <Button href={instance.project_url} as={Link} variant={"flat"} showAnchorIcon isExternal target={"_blank"}>View on {instance.platform}</Button>
            </div>
        </div>
    );
}