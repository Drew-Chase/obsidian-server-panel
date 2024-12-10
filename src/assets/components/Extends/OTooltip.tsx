import {cn, Tooltip, TooltipProps} from "@nextui-org/react";
import {forwardRef} from "react";

const OTooltip = forwardRef<HTMLDivElement, TooltipProps>((props, ref) => {
    return (
        <Tooltip
            ref={ref} // Pass the forwarded ref to the child component
            classNames={{base: cn("pointer-events-none", props.classNames?.base)}}
            closeDelay={props.closeDelay ?? 0}
            {...props}
        >
            {props.children}
        </Tooltip>
    );
});

export default OTooltip;