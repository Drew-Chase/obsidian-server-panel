import {Tab, Tabs} from "@nextui-org/react";
import {useState} from "react";
import InstalledModsList from "../../components/Server/InstalledModsList.tsx";
import DiscoverMods from "../../components/Server/DiscoverMods.tsx";
import {setTitle} from "../../../main.tsx";

export default function ServerMods()
{
    setTitle("Server Mods");
    const [tab, setTab] = useState("installed");
    return (
        <div className={"flex flex-col bg-neutral-600 rounded-2xl shadow-lg p-8 grow w-full gap-4 max-h-[calc(100dvh_-_130px)]"}>
            <div className={"flex flex-row items-center"}>
                <p className={"text-xl font-semibold mr-10"}>Server Mods</p>
                <Tabs
                    defaultSelectedKey={"installed"}
                    selectedKey={tab}
                    className={"flex-grow"}
                    onSelectionChange={(e) => setTab(e as string)}
                >
                    <Tab title={"Installed"} key={"installed"}/>
                    <Tab title={"Discover"} key={"discover"}/>
                </Tabs>
            </div>

            <div className={"flex flex-col gap-4"}>
                {tab === "installed" ? (<InstalledModsList/>) : (<DiscoverMods/>)}
            </div>
        </div>
    );
}