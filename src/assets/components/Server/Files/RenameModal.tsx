import {Button, Modal, ModalBody, ModalContent, ModalFooter, ModalHeader} from "@nextui-org/react";
import {FileItem} from "../../../ts/file-system.ts";
import OInput from "../../Extends/OInput.tsx";
import {useEffect, useState} from "react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faEdit} from "@fortawesome/free-solid-svg-icons";

interface RenameModalProps
{
    onClose: (filename: string | null) => void;
    file: FileItem | null;
}

export default function RenameModal(props: RenameModalProps)
{
    const [value, setValue] = useState(props.file?.name ?? "");

    useEffect(() =>
    {
        setValue(props.file?.name ?? "");
    }, [props.file]);

    return (
        <Modal isOpen={props.file !== null} onClose={() => props.onClose(value === "" || value === props.file?.name ? null : value)}>
            <ModalContent>
                {onClose => (
                    <>
                        <ModalHeader>Rename {props.file?.name}</ModalHeader>
                        <ModalBody>
                            <OInput
                                label={"New Name"}
                                value={value}
                                onValueChange={setValue}
                                endContent={<FontAwesomeIcon icon={faEdit}/>}
                                description={"Please enter a new name for the file."}
                                autoFocus
                            />
                        </ModalBody>
                        <ModalFooter>
                            <Button color={"primary"} onClick={onClose} isDisabled={value === "" || value === props.file?.name}>Rename</Button>
                            <Button color={"danger"} variant={"light"} onClick={onClose}>Cancel</Button>
                        </ModalFooter>
                    </>
                )}
            </ModalContent>
        </Modal>
    );
}