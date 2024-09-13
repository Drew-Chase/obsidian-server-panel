import {ReactElement} from "react";
import {Progress} from "@nextui-org/react";

interface StatCardProps
{
    title?: string;
    value?: number | string;
    maxValue?: number;
    icon?: ReactElement;
}

export default function StatCard(props: StatCardProps)
{
    return (
        <div className={"flex flex-col bg-neutral-600 rounded-2xl shadow-lg p-8 max-w-md w-full mx-2"}>
            <div className={"flex flex-row w-full items-center mb-3"}>
                <span className={"text-primary"}>{props.icon}</span>
                <p className={"text-tiny font-medium ml-2 mr-auto"}>{props.title}</p>
            </div>
            <p className={"text-4xl font-semibold mb-4"}>{props.value}</p>
            {props.maxValue && (
                <>
                    <p className={"text-sm text-neutral-500 ml-auto"}>of {props.maxValue}</p>
                    <Progress
                        size={"sm"}
                        value={Number.parseInt(props.value?.toString().replace(/\D/g, "") ?? "0")}
                        maxValue={props.maxValue}
                    />
                </>
            )}
        </div>
    );
}