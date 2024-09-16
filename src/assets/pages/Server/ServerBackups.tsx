import ExtendedBackupsList from "../../components/Server/ExtendedBackupsList.tsx";
import BackupOptionsPanel from "../../components/Server/BackupOptionsPanel.tsx";

export default function ServerBackups()
{
    return (
        <div className={"flex flex-row max-h-[calc(100dvh_-_60px)]"}>
            <ExtendedBackupsList/>
            <BackupOptionsPanel/>
        </div>
    );
}