import {setTitle} from "../../../main.tsx";
import JavaSettings from "../../components/Settings/JavaSettings.tsx";
import {default as AppSettings} from "../../components/Settings/ApplicationSettings.tsx";

export default function ApplicationSettings()
{
    setTitle("Application Settings");
    return (
        <div className={"flex flex-col gap-4"}>
            <AppSettings/>
            <JavaSettings/>
        </div>
    );
}