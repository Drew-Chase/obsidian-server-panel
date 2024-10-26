import {SelectItem, Slider} from "@nextui-org/react";
import ExtendedSwitch from "../../Extends/ExtendedSwitch.tsx";
import BackupIntervalSelector from "./BackupIntervalSelector.tsx";
import OSelect from "../../Extends/OSelect.tsx";

export default function BackupOptionsPanel()
{
    return (
        <div className={"flex flex-col bg-neutral-600 rounded-2xl shadow-lg p-8 max-w-md w-full mx-2 gap-4 overflow-y-auto max-h-[calc(100dvh_-_60px)] h-screen shrink-0"}>
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
            <OSelect
                label={"Backup Type"}
                placeholder={"Select a backup type"}
                defaultSelectedKeys={["incremental"]}
            >
                <SelectItem key={"full"} description={"This will make a complete copy of the server, this will take up the most storage."}>Full Backup</SelectItem>
                <SelectItem key={"incremental"} description={"This will make an incremental backup using git, this will only backup files that change."}>Incremental Backup</SelectItem>
            </OSelect>

            <BackupIntervalSelector/>
        </div>
    );
}