import {setTitle} from "../../../main.tsx";
import StatCard from "../../components/Dashboard/StatCard.tsx";
import CPU from "../../images/CPU.svg.tsx";
import RAM from "../../images/RAM.svg.tsx";
import Storage from "../../images/Storage.svg.tsx";
import ServerList from "../../components/Dashboard/ServerList.tsx";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faUser} from "@fortawesome/free-solid-svg-icons";

export default function DashboardServers()
{
    setTitle("Dashboard Servers");
    return (
        <div className={"flex flex-col gap-8"}>
            <div className={"flex flex-row"}>
                <p className={"text-xl font-semibold mr-auto"}>Servers</p>
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
            <ServerList/>
        </div>
    );
}