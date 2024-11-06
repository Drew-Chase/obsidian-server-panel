import {cn, Tooltip, TooltipProps} from "@nextui-org/react";

export default function OTooltip(props: TooltipProps)
{
    return (
        <Tooltip {...props} classNames={{base: cn("pointer-events-none", props.classNames?.base)}} closeDelay={props.closeDelay ?? 0}>
            {props.children}
        </Tooltip>
    );
}