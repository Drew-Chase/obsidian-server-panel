import {Input, Select, SelectItem, Slider} from "@nextui-org/react";
import ExtendedSwitch from "../Extends/ExtendedSwitch.tsx";

export default function BackupOptionsPanel()
{
    return (
        <div className={"flex flex-col bg-neutral-600 rounded-2xl shadow-lg p-8 max-w-md w-full mx-2 gap-4"}>
            <div className={"flex flex-row"}>
                <p className={"text-xl font-semibold mr-auto"}>Backup Options</p>
            </div>
            <ExtendedSwitch
                label={"WorldEdit Support"}
                description={"Enable WorldEdit support for backups."}
            />
            <Slider
                label={"Number of Backup Slots"}
                minValue={1}
                maxValue={32}
                step={1}
                defaultValue={10}
                showSteps
                showTooltip
            />
            <p className={"text-tiny text-neutral-400 opacity-70"}>The number of backups to keep.</p>
            <Select
                label={"Backup Type"}
                placeholder={"Select a backup type"}
                classNames={{
                    trigger: "bg-neutral-700"
                }}

                defaultSelectedKeys={["incremental"]}

            >
                <SelectItem key={"full"} description={"This will make a complete copy of the server, this will take up the most storage."}>Full Backup</SelectItem>
                <SelectItem key={"incremental"} description={"This will make an incremental backup using git, this will only backup files that change."}>Incremental Backup</SelectItem>
            </Select>
            <Select
                label={"Backup Interval"}
                placeholder={"Select a backup interval"}
                classNames={{
                    trigger: "bg-neutral-700"
                }}
                defaultSelectedKeys={["never"]}
            >
                <SelectItem key={"never"} description={"This will never create a backup"}>Never</SelectItem>
                <SelectItem key={"30min"} description={"This will create a backup every 30 minutes"}>Every Half-Hour</SelectItem>
                <SelectItem key={"hourly"} description={"This will create a backup at the top of every hour"}>Hourly</SelectItem>
                <SelectItem key={"3hours"} description={"This will create a backup every 3 hours"}>Every 3 Hours</SelectItem>
                <SelectItem key={"6hours"} description={"This will create a backup every 6 hours"}>Every 6 Hours</SelectItem>
                <SelectItem key={"12hours"} description={"This will create a backup every 12 hours"}>Every 12 Hours</SelectItem>
                <SelectItem key={"daily"} description={"This will create a backup every day"}>Every Day</SelectItem>
                <SelectItem key={"everyotherday"} description={"This will create a backup every other day"}>Every Other Day</SelectItem>
                <SelectItem key={"weekly"} description={"This will create a backup every week"}>Every Week</SelectItem>
                <SelectItem key={"everyotherweek"} description={"This will create a backup every other week"}>Every Other Week</SelectItem>
                <SelectItem key={"monthly"} description={"This will create a backup every month"}>Every Month</SelectItem>
                <SelectItem key={"everyothermonth"} description={"This will create a backup every other month"}>Every Other Month</SelectItem>
                <SelectItem key={"every3months"} description={"This will create a backup every 3 months"}>Every 3 Months</SelectItem>
                <SelectItem key={"every6months"} description={"This will create a backup every 6 months"}>Every 6 Months</SelectItem>
                <SelectItem key={"yearly"} description={"This will create a backup every year"}>Every Year</SelectItem>

            </Select>

        </div>
    );
}