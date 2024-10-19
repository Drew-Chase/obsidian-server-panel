import {Button, Modal, ModalBody, ModalContent, ModalFooter, ModalHeader, Progress, Spinner} from "@nextui-org/react";
import {InstallItems, JavaVersion} from "../../ts/java.ts";
import {useEffect, useState} from "react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faCheck} from "@fortawesome/free-solid-svg-icons";

interface JavaInstallModalProps
{
    isOpen: boolean;
    onClose: () => void;
    onCompleted: () => void;
    version: JavaVersion;
}


export default function JavaInstallModal(props: JavaInstallModalProps)
{
    const [isInstalling, setIsInstalling] = useState<boolean>(false);
    const [installItems, setInstallItems] = useState<InstallItems[]>([]);
    const [progress, setProgress] = useState<number>(0);
    const [completed, setCompleted] = useState<boolean>(false);
    const install = async () =>
    {
        if (completed) return;
        setCompleted(false);
        setIsInstalling(true);
        await props.version.install(items =>
        {
            setInstallItems(() =>
            {
                items.sort((a, b) => a.completed === b.completed ? 0 : a.completed ? 1 : -1);
                setProgress(items.filter(item => item.completed).length / items.length);
                return items;
            });
        }, (msg) => console.error(msg), () =>
        {
            setProgress(1);
            if (props.version)
            {
                props.version.files().then(files =>
                {
                    setInstallItems(files.map(file => ({file: file, completed: false})));
                });
            }
            setIsInstalling(false);
            setCompleted(true);
            props.onCompleted();
        });
    };

    useEffect(() =>
    {
        if (props.version)
        {
            setCompleted(false);
            setProgress(0);
            props.version.files().then(files =>
            {
                setInstallItems(files.map(file => ({file: file, completed: false})));
            });
        }
    }, [props.version]);

    return (
        <Modal isOpen={props.isOpen} onClose={props.onClose} size={"5xl"} scrollBehavior={"inside"} isDismissable={!isInstalling}>
            <ModalContent>
                {onClose => (
                    <>
                        <ModalHeader className={"flex flex-col"}>
                            <p>Install Java {props.version.version}</p>
                            <p className={"text-sm opacity-50 font-light italic"}>Java {props.version.version} will be installed on your server. This process may take a few minutes.</p>
                            <Progress size={"sm"} value={progress} maxValue={1} className={"w-full mt-2 -mb-4"} aria-label={"installation progress"}/>
                        </ModalHeader>
                        <ModalBody>
                            <div className={"flex flex-col"}>
                                {completed ? (
                                        <>
                                            {installItems.map((item, index) => (
                                                <div key={index} className={"flex flex-row w-full h-16 rounded-md bg-neutral-700 items-center px-2 mt-2"}>
                                                    <p className={"text-neutral-200 font-bold"}>{item.file}</p>
                                                    <FontAwesomeIcon icon={faCheck} className={"ml-auto text-green-500"}/>
                                                </div>
                                            ))}
                                        </>
                                    ) :
                                    (
                                        <>
                                            {isInstalling ? (
                                                <>
                                                    {installItems.map((item, index) => (
                                                        <div key={index} className={"flex flex-row w-full h-16 rounded-md bg-neutral-700 items-center px-2 mt-2"}>
                                                            <p className={"text-neutral-200 font-bold"}>Downloading {item.file}</p>
                                                            {item.completed ? <FontAwesomeIcon icon={faCheck} className={"ml-auto text-green-500"}/> : <Spinner size={"sm"} className={"ml-auto"}/>}
                                                        </div>
                                                    ))}
                                                </>
                                            ) : (
                                                <>
                                                    {installItems.map((item, index) => (
                                                        <div key={index} className={"flex flex-row w-full h-16 rounded-md bg-neutral-700 items-center px-2 mt-2"}>
                                                            <p className={"text-neutral-200 font-bold"}>{item.file}</p>
                                                        </div>
                                                    ))}
                                                </>
                                            )}
                                        </>
                                    )
                                }
                            </div>
                        </ModalBody>
                        <ModalFooter>
                            {completed ? (<Button onClick={onClose}>Close</Button>) : (
                                <>
                                    <Button isLoading={isInstalling} onClick={install}>Install</Button>
                                    <Button variant={"light"} color={"danger"} onClick={onClose} isDisabled={isInstalling}>Cancel</Button>
                                </>
                            )}
                        </ModalFooter>
                    </>
                )}
            </ModalContent>
        </Modal>
    );
}