import Java, {JavaVersion} from "../../ts/java.ts";
import {useEffect, useState} from "react";
import JavaVersionComponent from "./JavaVersionComponent.tsx";
import JavaInstallModal from "./JavaInstallModal.tsx";
import {Skeleton} from "@nextui-org/react";

export default function JavaSettings()
{
    const [versions, setVersions] = useState<JavaVersion[]>([]);
    const [installVersion, setInstallVersion] = useState<JavaVersion | null>(null);
    const [loading, setLoading] = useState<boolean>(true);
    const [error, setError] = useState<string | null>(null);

    useEffect(() =>
    {
        setLoading(true);
        setError("");
        Java.versions()
            .then(setVersions)
            .catch(e =>
            {
                console.error("Error loading Java versions", e);
                setError("An error occurred while loading Java versions. Please try again or check your network connection.");
            })
            .finally(() => setLoading(false));
    }, []);

    return (
        <>
            {<JavaInstallModal isOpen={installVersion !== null} onClose={() => setInstallVersion(null)} version={installVersion!} onCompleted={() => Java.versions().then(setVersions)}/>}
            <div id={"java-settings"} className={"flex flex-col bg-neutral-600 rounded-3xl shadow-lg p-8 w-full mx-2 overflow-y-auto"}>
                <p className={"text-lg font-semibold mr-auto mb-8"}>Java Settings</p>
                <div className={"flex flex-col gap-3"}>
                    <p className={"text-danger"}>{error}</p>
                    {loading ?
                        <>
                            {Array.from({length: 6}).map(() => (
                                <Skeleton>
                                    <div className={"h-16"}></div>
                                </Skeleton>
                            ))}
                        </> :
                        <>
                            {versions.map((version) => (
                                <JavaVersionComponent version={version} onInstall={setInstallVersion} onUninstall={version =>
                                {
                                    version.uninstall().then(() => Java.versions().then(setVersions));
                                }}/>
                            ))}
                        </>
                    }
                </div>

            </div>
        </>
    );
}