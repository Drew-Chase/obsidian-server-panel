import {cn, Input, InputProps} from "@nextui-org/react";
import {useEffect, useState} from "react";
import $ from "jquery";

interface DurationInputProps
{
    value?: Duration;
    onChange?: (value: Duration) => void;
    label?: string;
    placeholders?: Placeholder;
    className?: string;
    inputProps?: InputProps;
    isInvalid?: boolean;
    errorMessage?: string;
    description?: string;
}

interface Placeholder
{
    days: string;
    hours: string;
    minutes: string;
}

export interface Duration
{
    days: number;
    hours: number;
    minutes: number;
    cron: string;
}

export default function DurationInput(props: DurationInputProps)
{
    const [minutes, setMinutes] = useState<string>(props.value?.minutes.toString() || "00");
    const [hours, setHours] = useState<string>(props.value?.hours.toString() || "00");
    const [days, setDays] = useState<string>(props.value?.days.toString() || "00");
    const [cron, setCron] = useState<string>(props.value?.cron || "");
    const id = `duration-input-${Math.random().toString(36).replace(/[^a-z]+/g, "")}`;
    useEffect(() =>
    {
        $(`#${id} input`).on("focus", e =>
        {
            // select all text on focus
            $(e.currentTarget).trigger("select");
        });
    }, []);
    
    
    return (
        <div id={id} className={"flex flex-col rounded-lg bg-neutral-700 overflow-hidden p-2 shrink-0"}>
            <p className={"text-sm opacity-90 font-light mb-1 ml-1"}>{props.label || ""}</p>
            <div className={cn("flex flex-row rounded-lg overflow-hidden", props.className)}>
                <Input
                    {...props.inputProps}
                    label={"Days"}
                    placeholder={props.placeholders?.days || "DD"}
                    tabIndex={60}
                    radius={"none"}
                    classNames={{inputWrapper: "bg-neutral-700 group-data-[focus=true]:bg-neutral-800"}}
                    value={days}
                    onValueChange={(value) =>
                    {
                        const days = value.replace(/\D/g, "");
                        setDays(days);
                        if (props.onChange) props.onChange({days: +days, hours: +hours, minutes: +minutes});
                    }}
                    onFocusChange={(focused) =>
                    {
                        if (!focused)
                        {
                            if (days === "") setDays("00");
                            if ((+days) < 10) setDays(`0${+days}`);
                            if (props.onChange) props.onChange({days: +days, hours: +hours, minutes: +minutes});
                        }
                    }}
                />
                <Input
                    {...props.inputProps}
                    label={"Hours"}
                    placeholder={props.placeholders?.hours || "HH"}
                    tabIndex={61}
                    radius={"none"}
                    classNames={{inputWrapper: "bg-neutral-700 group-data-[focus=true]:bg-neutral-800"}}
                    value={hours}
                    onValueChange={(value) =>
                    {
                        const hours = value.replace(/\D/g, "");
                        if ((+hours) > 23) setHours("23");
                        else setHours(hours);
                        if (props.onChange) props.onChange({days: +days, hours: +hours, minutes: +minutes});
                    }}
                    onFocusChange={(focused) =>
                    {
                        if (!focused)
                        {
                            if (hours === "") setHours("00");
                            if ((+hours) > 23) setHours("23");
                            if ((+hours) < 10) setHours(`0${+hours}`);
                            if (props.onChange) props.onChange({days: +days, hours: +hours, minutes: +minutes});
                        }
                    }}
                />
                <Input
                    {...props.inputProps}
                    label={"Minutes"}
                    placeholder={props.placeholders?.minutes || "MM"}
                    tabIndex={62}
                    radius={"none"}
                    classNames={{inputWrapper: "bg-neutral-700 group-data-[focus=true]:bg-neutral-800"}}
                    value={minutes}
                    onValueChange={(value) =>
                    {
                        const minutes = value.replace(/\D/g, "");
                        if ((+minutes) > 59) setMinutes("59");
                        else setMinutes(minutes.replace(/\D/g, ""));
                        if (props.onChange) props.onChange({days: +days, hours: +hours, minutes: +minutes});
                    }}
                    onFocusChange={(focused) =>
                    {
                        if (!focused)
                        {
                            if (minutes === "") setMinutes("00");
                            if ((+minutes) > 59) setMinutes("59");
                            if ((+minutes) < 10) setMinutes(`0${+minutes}`);
                            if (props.onChange) props.onChange({days: +days, hours: +hours, minutes: +minutes});
                        }
                    }}
                />
            </div>

            <p className={
                cn(
                    "text-tiny font-light mt-1 ml-1 text-wrap",
                    (props.isInvalid && props.errorMessage) ? "text-danger" : "text-foreground opacity-50"
                )
            }
            >
                {(props.isInvalid && props.errorMessage) || props.description || ""}
            </p>
        </div>
    );
}

/**
 * Converts a given Duration object into a corresponding Cron expression.
 *
 * @param {Duration} duration - The duration object containing days, hours, and minutes.
 * @return {string} A string representing the Cron expression.
 */
function convertDurationToCron(duration: Duration): string
{
    const days = duration.days >= 0 ? `${duration.days}` : "0";
    const hours = duration.hours >= 0 && duration.hours <= 23 ? `${duration.hours}` : "0";
    const minutes = duration.minutes >= 0 && duration.minutes <= 59 ? `${duration.minutes}` : "0";

    return `${minutes} ${hours} */${days} * *`;
}