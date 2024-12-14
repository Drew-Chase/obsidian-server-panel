import {ScrollShadow} from "@nextui-org/react";
// @ts-ignore
import {Prism as SyntaxHighlighter} from "react-syntax-highlighter";
// @ts-ignore
import {duotoneDark} from "react-syntax-highlighter/dist/esm/styles/prism";
import {useEffect, useRef, useState} from "react";
import $ from "jquery";
import {useSelectedServer} from "../../../providers/SelectedServerProvider.tsx";
import {FileItem} from "../../../ts/file-system.ts";

interface LogOutputProps
{
    file: FileItem | null;
}

export default function LogOutput(props: LogOutputProps)
{
    const [log, setLog] = useState("");
    const [scrollLock, setScrollLock] = useState(true);
    const scrollLockRef = useRef(scrollLock); // Ref to track the latest scrollLock state
    const {server} = useSelectedServer();

    // Update the ref whenever scrollLock changes
    useEffect(() =>
    {
        scrollLockRef.current = scrollLock;
    }, [scrollLock]);

    useEffect(() =>
    {
        let view = $("#log-view");

        view.on("scroll", () =>
        {
            const isScrollLocked =
                (view.scrollTop() ?? 0) + (view.innerHeight() ?? 0) >= view[0].scrollHeight;

            // Update scrollLock state and the Ref
            setScrollLock(isScrollLocked);
        });


        return () =>
        {
            view.off("scroll");
        };
    }, []);


    useEffect(() =>
    {

        if (server && props.file)
        {
            const consoleServerSideEvent = new EventSource(`/api/server/${server.id}/console/sse?log_file=${props.file?.name ?? ""}`);
            consoleServerSideEvent.onopen = () => console.log("Connected to console server side event");
            consoleServerSideEvent.onerror = (e) => console.error("Error connecting to console server side event", e);
            consoleServerSideEvent.addEventListener("update_console", (event) =>
            {
                console.log("Update Console: ", event);
                setLog(event.data);
                handleScrollLock();
            });
            return () =>
            {
                console.log("Closing console server side event");
                consoleServerSideEvent.close();
            };
        }


    }, [server, props.file]);

    const handleScrollLock = () =>
    {
        let view = $("#log-view");
        console.log("Scroll lock is ", scrollLockRef.current); // Always get the latest scrollLock value
        if (scrollLockRef.current)
        {
            try
            {
                view.scrollTop(view[0].scrollHeight + 10);
            } catch
            {
                console.log("Error: ", view);
            }
        }
    };


    return (
        <ScrollShadow id={"log-view"} className={"max-h-[calc(100dvh_-_250px)] h-screen overflow-y-auto bg-neutral-800 rounded-2xl p-4"}>
            <SyntaxHighlighter language={"log"} style={duotoneDark} showLineNumbers wrapLines={true} wrapLongLines={true}
                               lineProps={{"className": "flex-wrap"}}
            >
                {log.trim()}
            </SyntaxHighlighter>
        </ScrollShadow>
    );
}