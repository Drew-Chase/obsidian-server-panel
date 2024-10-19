import Java, {JavaVersion} from "../../ts/java.ts";
import {useEffect, useState} from "react";
import JavaVersionComponent from "./JavaVersionComponent.tsx";
import JavaInstallModal from "./JavaInstallModal.tsx";

export default function JavaSettings()
{
    const [versions, setVersions] = useState<JavaVersion[]>([]);
    const [installVersion, setInstallVersion] = useState<JavaVersion | null>(null);

    useEffect(() =>
    {
        Java.versions().then(setVersions);
    }, []);

    return (
        <>
            {<JavaInstallModal isOpen={installVersion !== null} onClose={() => setInstallVersion(null)} version={installVersion!} onCompleted={() => Java.versions().then(setVersions)}/>}
            <div className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 w-full mx-2 overflow-y-auto"}>
                <p className={"text-lg font-semibold mr-auto mb-8"}>Java Settings</p>
                <div className={"flex flex-col gap-3"}>

                    {versions.map((version) => (
                        <JavaVersionComponent version={version} onInstall={setInstallVersion} onUninstall={version =>
                        {
                            version.uninstall().then(() => Java.versions().then(setVersions));
                        }}/>
                    ))}
                </div>

            </div>
        </>
    );
}