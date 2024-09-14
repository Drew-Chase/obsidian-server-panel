import ExtendedBackupsList from "../../components/Server/ExtendedBackupsList.tsx";
import BackupOptionsPanel from "../../components/Server/BackupOptionsPanel.tsx";

export default function ServerBackups()
{
    return (
        <div className={"flex flex-col gap-8"}>
            <div className={"flex flex-row"}>
                <p className={"text-xl font-semibold mr-auto"}>Server Backup</p>
            </div>
            <div className={"flex flex-row max-h-[calc(100dvh_-_130px)]"}>
                <ExtendedBackupsList/>
                <BackupOptionsPanel/>
            </div>
        </div>
    );
}