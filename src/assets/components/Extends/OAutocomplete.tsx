import {Autocomplete, AutocompleteProps, cn} from "@nextui-org/react";

export default function OAutocomplete(props: AutocompleteProps)
{
    return (
        <Autocomplete
            inputProps={{
                classNames:
                    {
                        inputWrapper: cn(
                            "bg-neutral-700 data-[focus]:!bg-neutral-800 data-[hover]:!bg-neutral-800",
                            props.inputProps?.classNames?.inputWrapper ?? ""
                        )
                    },
                ...props.inputProps
            }}
            popoverProps={{
                classNames:
                    {
                        content: cn(
                            "bg-default-100/75 backdrop-blur-md",
                            props.popoverProps?.classNames?.content ?? ""
                        )
                    },
                ...props.popoverProps
            }}
            {...props}
        >
            {props.children}
        </Autocomplete>
    );
}