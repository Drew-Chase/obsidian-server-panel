import ExtendedRecentConnectionsStat from "../../components/Server/ExtendedRecentConnectionsStat.tsx";
import ExtendedCurrentOnlinePlayersList from "../../components/Server/ExtendedCurrentOnlinePlayersList.tsx";
import WhiteListPlayers from "../../components/Server/WhiteListPlayers.tsx";
import BanPlayersList from "../../components/Server/BanPlayersList.tsx";
import {setTitle} from "../../../main.tsx";

export default function ServerPlayers()
{
    setTitle("Server Players");
    return (
        <div className={"flex flex-col gap-8"}>
            <div className={"flex flex-row"}>
                <p className={"text-xl font-semibold mr-auto"}>Players</p>
            </div>
            <ExtendedCurrentOnlinePlayersList/>
            <ExtendedRecentConnectionsStat/>
            <div className={"flex flex-row w-full gap-4"}>
                <WhiteListPlayers/>
                <BanPlayersList/>
            </div>
        </div>
    );
}