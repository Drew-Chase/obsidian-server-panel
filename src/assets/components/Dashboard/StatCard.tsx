import {ReactElement} from "react";
import {Dropdown} from "@nextui-org/react";

interface StatCardProps
{
    title: string;
    value: number;
    maxValue: number;
    valueContext: string;
    icon: ReactElement;
    dropdown: ReactElement<typeof Dropdown>;
}

export default function StatCard(props: StatCardProps)
{
    return (
        <>
        </>
    );
}