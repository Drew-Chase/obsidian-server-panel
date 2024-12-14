import {useEffect, useState} from "react";
import {FileItem} from "../../../ts/file-system.ts";
import {useSelectedServer} from "../../../providers/SelectedServerProvider.tsx";
import {SelectItem} from "@nextui-org/react";
import OSelect from "../../Extends/OSelect.tsx";

interface LogSelectorProps
{
    selectedLogFile: FileItem | null;
    onSelectionChange: (file: FileItem) => void;
}

export default function LogSelector(props: LogSelectorProps)
{
    const [files, setFiles] = useState<FileItem[]>([]);
    const {server} = useSelectedServer();
    useEffect(() =>
    {
        if (server)
            server
                .filesystem()
                .files("/logs/")
                .then(files => files.entries)
                .then(files => files.filter(i => i.type === "Log File"))
                .then(files =>
                {
                    setFiles(files);
                    if (files.length > 0 && !props.selectedLogFile)
                        props.onSelectionChange(files[0]);
                });
    }, []);

    return (
        <OSelect
            selectedKeys={[props.selectedLogFile ? `log-file-${props.selectedLogFile.name}` : ""]}
            className={"w-[200px]"}
            label={"Log File"}
            placeholder={"Select a log file..."}
            description={"Select a log file to view"}
            disallowEmptySelection
            onSelectionChange={(key) =>
            {
                const file = files.find(i => i.name === key.currentKey?.replace("log-file-", ""));
                if (file)
                    props.onSelectionChange(file);

            }}
        >
            {files.map((file) => (
                <SelectItem key={`log-file-${file.name}`}>{file.name}</SelectItem>
            ))}
        </OSelect>
    );
}