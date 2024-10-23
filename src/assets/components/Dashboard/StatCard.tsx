import {ReactElement} from "react";
import {Progress} from "@nextui-org/react";

interface StatCardProps
{
    title?: string;
    value?: number;
    valueDisplay?: string;
    maxValue?: number;
    maxValueDisplay?: string;
    icon?: ReactElement;
}

export default function StatCard(props: StatCardProps)
{
    return (
        <div className={"flex flex-col bg-neutral-600 rounded-2xl shadow-lg p-8 max-w-md min-w-sm grow shrink"}>
            <div className={"flex flex-row w-full items-center mb-3"}>
                <span className={"text-primary"}>{props.icon}</span>
                <p className={"text-tiny font-medium ml-2 mr-auto"}>{props.title}</p>
            </div>
            <p className={"text-4xl font-semibold mb-4"}>{props.valueDisplay}</p>
            {props.maxValue && (
                <>
                    <p className={"text-sm text-neutral-500 ml-auto"}>of {props.maxValueDisplay}</p>
                    <Progress
                        aria-label={props.title}
                        size={"sm"}
                        value={props.value}
                        maxValue={props.maxValue}
                    />
                </>
            )}
        </div>
    );
}