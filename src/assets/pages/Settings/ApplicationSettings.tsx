import {setTitle} from "../../../main.tsx";
import JavaSettings from "../../components/Settings/JavaSettings.tsx";

export default function ApplicationSettings()
{
    setTitle("Application Settings");
    return (
        <div className={"flex flex-col gap-4"}>
            <div className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 w-full mx-2 overflow-y-auto"}>
                <p className={"text-lg font-semibold mr-auto mb-8"}>Application Settings</p>
            </div>
            <JavaSettings/>
        </div>
    );
}