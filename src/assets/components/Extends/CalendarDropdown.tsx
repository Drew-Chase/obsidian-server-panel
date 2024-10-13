import {Button, ButtonProps, Calendar, CalendarProps, DateValue, Dropdown, DropdownItem, DropdownMenu, DropdownMenuProps, DropdownProps, DropdownTrigger} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faChevronDown} from "@fortawesome/free-solid-svg-icons";
import {getLocalTimeZone} from "@internationalized/date";
import {ReactElement, useState} from "react";


interface CalendarDropdownProps
{
    value?: DateValue;
    onValueChange?: (date: DateValue) => void;
    showYear?: boolean;
    showMonth?: boolean;
    showDay?: boolean;
    calendarProps?: CalendarProps;
    dropdownProps?: DropdownProps;
    dropdownMenuProps?: DropdownMenuProps;
    triggerProps?: ButtonProps;
    useButton?: (date: DateValue) => ReactElement<typeof Button>;
}


export default function CalendarDropdown(props: CalendarDropdownProps)
{
    let mutableProps = {...props};
    if (mutableProps.showYear === undefined) mutableProps.showYear = true;
    if (mutableProps.showMonth === undefined) mutableProps.showMonth = true;
    if (!mutableProps.value) return null;

    const [date, setDate] = useState<DateValue>(mutableProps.value);
    return (

        <Dropdown
            {...mutableProps.dropdownProps}
            classNames={{
                content: "bg-neutral-800"
            }}
        >
            <DropdownTrigger>
                {mutableProps.useButton ? mutableProps.useButton(date) : (
                    <Button
                        {...mutableProps.triggerProps}
                        endContent={<FontAwesomeIcon icon={faChevronDown}/>}
                    >
                        <div className={"flex flex-row gap-1"}>
                            <span>{mutableProps.showMonth && date.toDate(getLocalTimeZone()).toLocaleString("default", {month: "long"})}</span>
                            <span>{mutableProps.showDay && date.toDate(getLocalTimeZone()).toLocaleString("default", {day: "2-digit"})},</span>
                            <span>{mutableProps.showYear && date.toDate(getLocalTimeZone()).getFullYear()}</span>
                        </div>
                    </Button>
                )}
            </DropdownTrigger>
            <DropdownMenu
                closeOnSelect={false}
                classNames={{
                    base: "bg-neutral-800"
                }}
                itemClasses={{
                    base: "data-[hover]:bg-neutral-800"
                }}
            >
                <DropdownItem closeOnSelect={false}>
                    <Calendar
                        {...mutableProps.calendarProps}
                        showMonthAndYearPickers
                        onChange={(date) =>
                        {
                            setDate(date);
                            if (mutableProps.onValueChange) mutableProps.onValueChange(date);
                        }}
                        value={date}
                        classNames={{
                            content: "bg-neutral-800",
                            base: "bg-neutral-800",
                            gridHeader: "bg-neutral-700",
                            header: "bg-neutral-700",
                            headerWrapper: "bg-neutral-700"
                        }}
                    />
                </DropdownItem>
            </DropdownMenu>
        </Dropdown>
    );
}