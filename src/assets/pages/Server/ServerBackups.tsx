import ExtendedBackupsList from "../../components/Server/ExtendedBackupsList.tsx";
import BackupOptionsPanel from "../../components/Server/BackupOptionsPanel.tsx";
import {setTitle} from "../../../main.tsx";

export default function ServerBackups()
{
    setTitle("Server Backups");
    return (
        <div className={"flex flex-row max-h-[calc(100dvh_-_60px)]"}>
            <ExtendedBackupsList/>
            <BackupOptionsPanel/>
        </div>
    );
}