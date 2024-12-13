import {setTitle} from "../../../main.tsx";
import ServerSidePanel from "../../components/Server/Details/ServerSidePanel.tsx";
import BackupsList from "../../components/Server/Backups/BackupsList.tsx";
import CrashReportsStat from "../../components/Dashboard/CrashReportsStat.tsx";
import MinecraftVersionsList from "../../components/Server/Details/MinecraftVersionsList.tsx";
import LoaderVersionsList from "../../components/Server/Details/LoaderVersionsList.tsx";
import OverviewStatCard from "../../components/Dashboard/StatCards/OverviewStatCard.tsx";
import {useEffect} from "react";
import {useSelectedServer} from "../../providers/SelectedServerProvider.tsx";

export default function ServerDetails()
{
    setTitle("Server Details");
    let {server} = useSelectedServer();

    useEffect(() =>
    {
        server?.onServerUpdate((oldServer, newServer, diff) =>
        {
            console.log(oldServer, newServer, diff);
        });

        return () =>
        {
            server?.closeServerStateUpdateEvent();
        };
    }, [server]);

    return (
        <div className={"flex flex-col gap-8"}>
            <OverviewStatCard/>
            <div className={"flex flex-row w-full gap-8"}>
                <div className={"w-full flex flex-col gap-8"}>
                    <div className={"flex flex-row gap-4"}>
                        <BackupsList/>
                        <CrashReportsStat/>
                    </div>
                    <MinecraftVersionsList/>
                    <LoaderVersionsList/>
                </div>
                <ServerSidePanel/>
            </div>
        </div>
    );
}