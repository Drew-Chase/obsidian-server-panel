import {cn, Listbox, ListboxProps} from "@nextui-org/react";

interface ExtendedListboxProps extends ListboxProps
{
}

export default function ExtendedListbox(props: ExtendedListboxProps)
{
    return (
        <Listbox
            classNames={{
                list: cn("bg-background-L-100 p-4 rounded-lg max-h-[400px] overflow-y-auto", props.classNames?.list)
            }}
            itemClasses={{
                base: cn(
                    "p-4",
                    "rounded-md",
                    "text-default-800 dark:text-default-500",
                    "transition-opacity",
                    "data-[hover=true]:text-foreground",
                    "data-[hover=true]:bg-background-L100",
                    "data-[selectable=true]:focus:bg-default-50",
                    "data-[pressed=true]:opacity-70",
                    "data-[focus-visible=true]:ring-default-500",
                    props.itemClasses?.base
                )
            }}
        >
            {props.children}
        </Listbox>
    );
}