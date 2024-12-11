import {Autocomplete, AutocompleteItem, Button, ScrollShadow, SelectItem} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faPaperPlane, faTerminal} from "@fortawesome/free-solid-svg-icons";
// @ts-ignore
import {Prism as SyntaxHighlighter} from "react-syntax-highlighter";
// @ts-ignore
import {duotoneDark} from "react-syntax-highlighter/dist/esm/styles/prism";

import "../../scss/logpage.scss";
import {useEffect, useState} from "react";

import $ from "jquery";
import {setTitle} from "../../../main.tsx";
import {useSelectedServer} from "../../providers/SelectedServerProvider.tsx";
import {FileItem} from "../../ts/file-system.ts";
import OSelect from "../../components/Extends/OSelect.tsx";

export default function ServerConsole()
{
    setTitle("Server Console");
    let scrollLock: boolean = true;
    const [log, setLog] = useState("");
    const [files, setFiles] = useState<FileItem[]>([]);
    const [selectedFile, setSelectedFile] = useState<FileItem | null>(null);
    const [updateTimer, setUpdateTimer] = useState(0);

    const {server} = useSelectedServer();

    useEffect(() =>
    {
        const view = $("#log-view");
        view.on("scroll", () =>
        {
            scrollLock = (view.scrollTop() ?? 0) + (view.innerHeight() ?? 0) >= view[0].scrollHeight;
            console.log("Scroll Lock: ", scrollLock);
        });
        const scrollInterval = setInterval(() =>
        {
            if (scrollLock)
            {
                view.scrollTop(view[0].scrollHeight + 10);
                console.log("scrolling");
            }
        }, 500);


        if (server)
            server
                .filesystem()
                .files("/logs/")
                .then(files => files.entries)
                .then(files => files.filter(i => i.type === "Log File"))
                .then(files =>
                {
                    setFiles(files);
                    if (files.length > 0 && !selectedFile)
                        setSelectedFile(files[0]);
                });


        return () =>
        {
            clearInterval(scrollInterval);
            clearInterval(updateTimer);
            view.off("scroll");
        };

    }, []);

    useEffect(() =>
    {
        const updateLog = () =>
        {
            if (!selectedFile) return;
            server?.filesystem().getFileContents(selectedFile).then(setLog);
        };
        updateLog();
        clearInterval(updateTimer);
        const updateLogInterval = setInterval(updateLog, 1000);
        setUpdateTimer(updateLogInterval);
        return () => clearInterval(updateLogInterval);
    }, [selectedFile]);

    return (
        <div className={"flex flex-col bg-neutral-600 rounded-2xl shadow-lg p-8 grow w-full mx-2 gap-4 max-h-[calc(100dvh_-_125px)] h-screen"}>
            <div className={"flex flex-row"}>
                <p className={"text-xl font-semibold mr-auto"}>Server Console</p>
                <OSelect
                    selectedKeys={[selectedFile ? `log-file-${selectedFile.name}` : ""]}
                    className={"w-[200px]"}
                    label={"Log File"}
                    placeholder={"Select a log file..."}
                    description={"Select a log file to view"}
                    disallowEmptySelection
                >
                    {files.map((file) => (
                        <SelectItem key={`log-file-${file.name}`}>{file.name}</SelectItem>
                    ))}
                </OSelect>
            </div>
            <ScrollShadow id={"log-view"} className={"max-h-[calc(100dvh_-_250px)] h-screen overflow-y-auto bg-neutral-800 rounded-2xl p-4 max-w-full"}>
                <SyntaxHighlighter language={"log"} style={duotoneDark} wrapLongLines wrapLines showLineNumbers className={"h-[calc(100%_-_40px)] max-w-full"}>
                    {log}
                </SyntaxHighlighter>
            </ScrollShadow>
            <Autocomplete
                label={"Command"}
                placeholder={"Enter command..."}
                startContent={<FontAwesomeIcon icon={faTerminal}/>}
                endContent={<Button variant={"light"}><FontAwesomeIcon icon={faPaperPlane}/></Button>}
                className={"w-full drop-shadow-lg shrink-0 pr-0"}
                inputProps={{
                    classNames: {
                        inputWrapper: "bg-neutral-700"
                    }
                }}
            >
                {Array.from({length: 20}, (_, i) => (<AutocompleteItem key={`say-${i}`} value={"say"} textValue={`say ${i}`}>say {i}</AutocompleteItem>))}
            </Autocomplete>
        </div>
    );
}
