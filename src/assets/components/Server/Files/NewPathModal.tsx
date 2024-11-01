import {Button, Modal, ModalBody, ModalContent, ModalFooter, ModalHeader, Tab, Tabs} from "@nextui-org/react";
import {useEffect, useState} from "react";
import {useSelectedServer} from "../../../providers/SelectedServerProvider.tsx";
import OInput from "../../Extends/OInput.tsx";

interface NewPathModalOptions
{
    isOpen: boolean;
    onClose: () => void;
    currentPath: string;
}

export default function NewPathModal(props: NewPathModalOptions)
{
    const [createFile, setCreateFile] = useState(false);
    const [filename, setFilename] = useState("");
    const {server} = useSelectedServer();
    if (server == null) return null;

    useEffect(() =>
    {
        setFilename("");
        setCreateFile(false);
    }, [props.isOpen]);

    return (
        <Modal isOpen={props.isOpen} onClose={props.onClose}>
            <ModalContent>
                {onClose => (
                    <>
                        <ModalHeader>Create New {createFile ? "File" : "Directory"}</ModalHeader>
                        <ModalBody>
                            <Tabs
                                selectedKey={createFile ? "file" : "directory"}
                                onSelectionChange={key => setCreateFile(key === "file")}
                            >
                                <Tab key={"directory"} title={"Directory"}/>
                                <Tab key={"file"} title={"File"}/>
                            </Tabs>
                            <OInput
                                label={`${createFile ? "File" : "Directory"} Name`}
                                placeholder={`Enter the name of the new ${createFile ? "file" : "directory"}`}
                                description={`The name of the new ${createFile ? "file" : "directory"}`}
                                value={filename}
                                onValueChange={setFilename}
                            />
                        </ModalBody>
                        <ModalFooter>
                            <Button
                                color={"primary"}
                                onClick={() =>
                                {
                                    if (createFile) server?.filesystem().createFile(props.currentPath, filename);
                                    else server?.filesystem().createDirectory(props.currentPath, filename);
                                    onClose();
                                }}
                            >
                                Create {createFile ? "File" : "Directory"}
                            </Button>
                            <Button
                                color={"danger"}
                                onClick={onClose}
                                variant={"light"}
                            >
                                Cancel
                            </Button>
                        </ModalFooter>
                    </>
                )}
            </ModalContent>
        </Modal>
    );
}