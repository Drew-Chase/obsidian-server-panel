import {cn, Select, SelectProps} from "@nextui-org/react";

export default function OSelect(props: SelectProps)
{
    return (

        <Select
            classNames={
                {
                    ...props.classNames,
                    trigger: cn(
                        "bg-neutral-700 data-[hover]:bg-neutral-800",
                        props.classNames?.trigger ?? ""
                    ),
                    popoverContent: cn(
                        "w-full bg-default-100/75  backdrop-blur-md",
                        props.classNames?.popoverContent ?? ""
                    )
                }}
            {...props}
        >
            {props.children}
        </Select>
    );
}