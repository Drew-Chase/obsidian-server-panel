import {Button, Image, Modal, ModalBody, ModalContent, ModalFooter, ModalHeader} from "@nextui-org/react";

interface ImageModalProps
{
    title?: string;
    image?: string | null;
    type?: "base64" | "url";
    onClose?: () => void;
}

export default function ImageModal(props: ImageModalProps)
{
    return (
        <Modal isOpen={props.image !== null} onClose={props.onClose}>
            <ModalContent>
                {onClose => (
                    <>
                        <ModalHeader>{props.title || "Image Preview"}</ModalHeader>
                        <ModalBody>
                            <div className={"rounded-lg bg-neutral-800/50 text-white/75 text-[150px] flex justify-center items-center p-4 w-[200px] mx-auto"}>
                                <Image src={props.image || ""} alt={props.title} />
                            </div>
                        </ModalBody>

                        <ModalFooter>
                            <Button color={"primary"} onClick={onClose}>Close</Button>
                        </ModalFooter>

                    </>
                )}
            </ModalContent>
        </Modal>
    );
}