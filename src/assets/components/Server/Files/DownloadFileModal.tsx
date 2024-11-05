import {FileItem} from "../../../ts/file-system.ts";
import {Button, Modal, ModalBody, ModalContent, ModalFooter, ModalHeader} from "@nextui-org/react";
import {useSelectedServer} from "../../../providers/SelectedServerProvider.tsx";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faImage} from "@fortawesome/free-solid-svg-icons";

interface DownloadFileModalProps
{
    file: FileItem | null;
    onClose: () => void;
}

export default function DownloadFileModal(props: DownloadFileModalProps)
{
    const {server} = useSelectedServer();
    return (
        <Modal isOpen={props.file !== null} onClose={() => props.onClose()}>
            <ModalContent>
                {onClose => (
                    <>
                        <ModalHeader>Preview Not Available</ModalHeader>
                        <ModalBody>
                            <div className={"rounded-lg bg-neutral-800/50 text-white/75 text-[150px] flex justify-center items-center p-4 w-[200px] mx-auto"}>
                                <FontAwesomeIcon icon={faImage}/>
                            </div>
                            <p>Preview is not available for this file type. Would you like to download the file instead?</p>
                        </ModalBody>
                        <ModalFooter>
                            <Button color={"primary"} onClick={() =>
                            {
                                if (props.file == null) return;
                                onClose();
                                server?.filesystem().download(props.file);
                            }}>Download</Button>
                            <Button variant={"light"} color={"danger"} onClick={onClose}>Cancel</Button>
                        </ModalFooter>
                    </>
                )}
            </ModalContent>
        </Modal>
    );
}