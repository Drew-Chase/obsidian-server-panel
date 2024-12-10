import {useEffect, useState} from "react";
import JavaVersionComponent from "./JavaVersionComponent.tsx";
import {Skeleton} from "@nextui-org/react";
import JavaInstallModal from "../../Settings/JavaInstallModal.tsx";
import Java, {JavaVersion} from "../../../ts/java.ts";

interface JavaSettingsProps
{
    selected: JavaVersion|null;
    onSelect: (version: JavaVersion) => void;
}

export default function JavaSettings(props: JavaSettingsProps)
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
        <div className={"flex flex-col my-8"}>
            {<JavaInstallModal isOpen={installVersion !== null} onClose={() => setInstallVersion(null)} version={installVersion!} onCompleted={() => Java.versions().then(setVersions)}/>}
            <div id={"java-settings"} className={"flex flex-col w-full mx-2 min-h-[200px]"}>
                <p className={"text-lg font-semibold mr-auto mb-2"}>Java Settings</p>
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
                            {versions.sort((a, b) =>
                            {
                                let [majorA, minorA, patchA] = a.version.split(".").map(i => Number.parseInt(i));
                                let [majorB, minorB, patchB] = b.version.split(".").map(i => Number.parseInt(i));

                                return (majorB - majorA) || (minorB - minorA) || (patchB - patchA);
                            })
                                .map((version) => (
                                    <JavaVersionComponent
                                        version={version}
                                        onInstall={setInstallVersion}
                                        selected={props.selected === version}
                                        onSelect={props.onSelect}
                                    />
                                ))}
                        </>
                    }
                </div>

            </div>
        </div>
    );
}