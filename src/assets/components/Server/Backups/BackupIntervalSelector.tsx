import {cn, SelectItem} from "@nextui-org/react";
import DurationInput, {Duration} from "../../Extends/DurationInput.tsx";
import {useState} from "react";
import OSelect from "../../Extends/OSelect.tsx";


export default function BackupIntervalSelector()
{
    const [backupInterval, setBackupInterval] = useState("never");
    const [customDuration, setCustomDuration] = useState<Duration>({days: 0, hours: 0, minutes: 15});
    const [customDurationError, setCustomDurationError] = useState("");
    const [customDurationTimeMessage, setCustomDurationTimeMessage] = useState("");

    return (
        <div className={"flex flex-col bg-neutral-700 rounded-lg overflow-hidden p-1 shrink-0"}>
            <OSelect
                label={"Backup Interval"}
                placeholder={"Select a backup interval"}
                className="bg-neutral-700"
                onSelectionChange={(key) => setBackupInterval((key.currentKey ?? "") as string)}
                defaultSelectedKeys={["never"]}
                disallowEmptySelection
            >
                <SelectItem key={"never"} description={"This will never create a backup"}>Never</SelectItem>
                <SelectItem key={"custom"} description={"Create a custom backup interval"}>Custom</SelectItem>
                <SelectItem key={"cron"} description={"Create a custom backup interval using cron"}>Cron</SelectItem>
                <SelectItem key={"30min"} value={"*/30 * * * *"} description={"This will create a backup every 30 minutes"}>Every Half-Hour</SelectItem>
                <SelectItem key={"hourly"} value={"0 * * * *"} description={"This will create a backup at the top of every hour"}>Hourly</SelectItem>
                <SelectItem key={"3hours"} value={"0 */3 * * *"} description={"This will create a backup every 3 hours"}>Every 3 Hours</SelectItem>
                <SelectItem key={"6hours"} value={"0 */6 * * *"} description={"This will create a backup every 6 hours"}>Every 6 Hours</SelectItem>
                <SelectItem key={"12hours"} value={"0 */12 * * *"} description={"This will create a backup every 12 hours"}>Every 12 Hours</SelectItem>
                <SelectItem key={"daily"} value={"0 0 * * *"} description={"This will create a backup every day"}>Every Day</SelectItem>
                <SelectItem key={"everyotherday"} value={"0 0 */2 * *"} description={"This will create a backup every other day"}>Every Other Day</SelectItem>
                <SelectItem key={"weekly"} value={"0 0 * * 0"} description={"This will create a backup every week"}>Every Week</SelectItem>
                <SelectItem key={"everyotherweek"} value={"0 0 * * 0/2"} description={"This will create a backup every other week"}>Every Other Week</SelectItem>
                <SelectItem key={"monthly"} value={"0 0 1 * *"} description={"This will create a backup every month"}>Every Month</SelectItem>
                <SelectItem key={"everyothermonth"} value={"0 0 1 */2 *"} description={"This will create a backup every other month"}>Every Other Month</SelectItem>
                <SelectItem key={"every3months"} value={"0 0 1 */3 *"} description={"This will create a backup every 3 months"}>Every 3 Months</SelectItem>
                <SelectItem key={"every6months"} value={"0 0 1 */6 *"} description={"This will create a backup every 6 months"}>Every 6 Months</SelectItem>
                <SelectItem key={"yearly"} value={"0 0 1 1 *"} description={"This will create a backup every year"}>Every Year</SelectItem>
            </OSelect>
            <div
                data-custom={backupInterval === "custom"}
                className={
                    cn(
                        "opacity-0 overflow-hidden max-h-0 h-[400px] shrink-0 grow w-full",
                        "transition-all duration-300",
                        "data-[custom=true]:opacity-100 data-[custom=true]:max-h-[90px]"
                    )
                }
            >
                <DurationInput
                    isInvalid={customDurationError.length > 0}
                    errorMessage={customDurationError}
                    value={customDuration}
                    description={customDurationTimeMessage}
                    onChange={(value) =>
                    {
                        if (value.days < 0 || value.hours < 0 || value.minutes < 0)
                        {
                            setCustomDurationError("Invalid duration");
                            return;
                        } else if (value.days === 0 && value.hours === 0 && value.minutes === 0)
                        {
                            setCustomDurationError("Please specify a duration!");
                            return;
                        }
                        setCustomDurationError("");
                        setCustomDuration(value);
                        let message = `Every `;
                        if (value.days > 0) message += `${value.days} day${value.days > 1 ? "s" : ""} `;
                        if (value.hours > 0) message += `${value.hours} hour${value.hours > 1 ? "s" : ""} `;
                        if (value.minutes > 0) message += `${value.minutes} minute${value.minutes > 1 ? "s" : ""}`;
                        setCustomDurationTimeMessage(message);
                    }}
                />
            </div>
        </div>
    );
}