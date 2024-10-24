import {Dropdown, DropdownProps} from "@nextui-org/react";

export default function ODropdown(props: DropdownProps)
{
    return (
        <Dropdown
            classNames={
                {
                    ...props.classNames,
                    content: "w-full bg-default-100/75  backdrop-blur-md"
                }}
            {...props}
        >
            {props.children}
        </Dropdown>
    );
}