import {Avatar, Button, Switch, Tooltip} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faArrowsRotate, faCheck, faTrash} from "@fortawesome/free-solid-svg-icons";
import {useEffect, useState} from "react";

interface ModItemProps
{
    icon?: string;
    name: string;
    author?: string;
    version: string;
}

export default function InstalledModItem(props: ModItemProps)
{
    const [disabled, setDisabled] = useState(false);
    const [requiresUpdate, setRequiresUpdate] = useState(false);

    useEffect(() =>
    {
        const random = Math.random();
        setDisabled(random > 0.5);
        setRequiresUpdate(random > 0.5);
    }, []);


    return (
        <div className={"flex flex-row shrink-0 p-4 justify-between items-center"}>
            <div className={"flex flex-row shrink-0"}>
                {props.icon && <Avatar src={props.icon} radius={"sm"} className={"mr-4"}/>}
                <div className={"flex flex-col"}>
                    <p>{props.name}</p>
                    <p className={"text-tiny opacity-70"}>By {props.author}</p>
                </div>
            </div>
            <p className={"opacity-70"}>{props.version}</p>
            <div className={"flex flex-row items-center gap-4"}>
                <OTooltip content={`Delete '${props.name}'`}>
                    <Button color={"danger"} className={"min-w-0"}><FontAwesomeIcon icon={faTrash}/></Button>
                </OTooltip>
                <OTooltip content={`Update '${props.name}'`}>
                    <Button isDisabled={!requiresUpdate} className={"min-w-0"}><FontAwesomeIcon icon={requiresUpdate ? faArrowsRotate : faCheck}/></Button>
                </OTooltip>
                <Switch isSelected={!disabled} onValueChange={value => setDisabled(!value)}/>
            </div>
        </div>
    );
}