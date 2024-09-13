import {ReactElement} from "react";
import {Button, Dropdown, DropdownItem, DropdownMenu, DropdownTrigger} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faEllipsisH} from "@fortawesome/free-solid-svg-icons";

interface StatCardProps
{
    title?: string;
    value?: number;
    maxValue?: number;
    valueContext?: string;
    icon?: ReactElement;
    children?: ReactElement<typeof DropdownItem>[] | ReactElement<typeof DropdownItem>;
}

export default function StatCard(props: StatCardProps)
{
    return (
        <div className={"flex flex-col bg-neutral-600 rounded-lg shadow-lg"}>
            <div className={"flex flex-row w-full"}>

                <span className={"text-primary"}>{props.icon}</span>
                <p className={"text-lg font-semibold ml-2 mr-auto"}>{props.title}</p>
                {props.children && (
                    <Dropdown>
                        <DropdownTrigger><Button variant={"ghost"}><FontAwesomeIcon icon={faEllipsisH}/></Button></DropdownTrigger>
                        <DropdownMenu>
                            {
                                (() =>
                                    {
                                        if (Array.isArray(props.children))
                                            return props.children.map((child) => (<>{child}</>));
                                        return <>{props.children}</>;
                                    }
                                )();
                            }
                        </DropdownMenu>
                    </Dropdown>
                )}


            </div>
        </div>
    );
}