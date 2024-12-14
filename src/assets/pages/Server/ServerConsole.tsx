import "../../scss/logpage.scss";
import {useState} from "react";
import {setTitle} from "../../../main.tsx";
import {FileItem} from "../../ts/file-system.ts";
import LogSelector from "../../components/Server/Console/LogSelector.tsx";
import LogOutput from "../../components/Server/Console/LogOutput.tsx";
import CommandInput from "../../components/Server/Console/CommandInput.tsx";

export default function ServerConsole()
{
    setTitle("Server Console");
    const [selectedLogFile, setSelectedLogFile] = useState(null as FileItem | null);


    return (
        <div className={"flex flex-col bg-neutral-600 rounded-2xl shadow-lg p-8 grow w-full mx-2 gap-4 max-h-[calc(100dvh_-_125px)] h-screen"}>
            <div className={"flex flex-row"}>
                <p className={"text-xl font-semibold mr-auto"}>Server Console</p>
                <LogSelector selectedLogFile={selectedLogFile} onSelectionChange={setSelectedLogFile}/>
            </div>

            <LogOutput file={selectedLogFile}/>
            <CommandInput/>
        </div>
    );
}
