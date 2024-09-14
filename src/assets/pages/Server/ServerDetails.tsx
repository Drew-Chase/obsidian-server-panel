import {setTitle} from "../../../main.tsx";
import StatCard from "../../components/Dashboard/StatCard.tsx";
import CPU from "../../images/CPU.svg.tsx";
import RAM from "../../images/RAM.svg.tsx";
import Storage from "../../images/Storage.svg.tsx";
import ServerSidePanel from "../../components/Server/ServerSidePanel.tsx";
import BackupsList from "../../components/Server/BackupsList.tsx";
import CrashReportsStat from "../../components/Dashboard/CrashReportsStat.tsx";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faUser} from "@fortawesome/free-solid-svg-icons";
import MinecraftVersionsList from "../../components/Server/MinecraftVersionsList.tsx";
import LoaderVersionsList from "../../components/Server/LoaderVersionsList.tsx";

export default function ServerDetails()
{
    setTitle("Server Details");
    return (
        <div className={"flex flex-col gap-8"}>
            <div className={"flex flex-row"}>
                <p className={"text-xl font-semibold mr-auto"}>SMP Server</p>
            </div>
            <div className={"flex flex-row w-full justify-between"}>
                <StatCard
                    title={"CPU Usage"}
                    value={`40%`}
                    maxValue={100}
                    icon={<CPU size={24}/>}
                />
                <StatCard
                    title={"Memory Usage"}
                    value={`28 GB`}
                    maxValue={64}
                    icon={<RAM size={24}/>}
                />
                <StatCard
                    title={"Storage Usage"}
                    value={`800 GB`}
                    maxValue={1000}
                    icon={<Storage size={24}/>}
                />
                <StatCard
                    title={"Online Players"}
                    value={`4 Players`}
                    maxValue={20}
                    icon={<FontAwesomeIcon icon={faUser}/>}
                />
            </div>

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