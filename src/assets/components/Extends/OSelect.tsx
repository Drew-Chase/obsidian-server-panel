import {cn, Select, SelectProps} from "@nextui-org/react";
import {forwardRef} from "react";

const OSelect = forwardRef<HTMLSelectElement, SelectProps>((props, ref) => {
    return (
        <Select
            ref={ref} // Pass the forwarded ref to the child component
            classNames={{
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
});

export default OSelect;