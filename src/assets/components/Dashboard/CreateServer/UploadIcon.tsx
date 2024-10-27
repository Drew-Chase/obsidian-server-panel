import {Button, cn, Divider, Image} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faUpload} from "@fortawesome/free-solid-svg-icons";
import $ from "jquery";
import ImageCropModal from "./ImageCropModal.tsx";
import {useEffect, useState} from "react";
import {resizeImage} from "../../../ts/image-resizer.ts";

export default function UploadIcon({onUpload}: { onUpload: (file: File) => void })
{
    const [isOpen, setIsOpen] = useState<boolean>(false);
    const [file, setFile] = useState<File | null>(null);
    const [isDraggingOver, setIsDraggingOver] = useState(false);
    const key = `upload-icon-${Math.random().toString(36).substring(7)}`;

    const handleOpen = () =>
    {
        $("<input>")
            .attr("type", "file")
            .attr("accept", "image/*")
            .on("change", (e) =>
            {
                const files = (e.target as HTMLInputElement).files;
                if (files && files.length > 0)
                {
                    cropper(files[0]);
                }
            })
            .trigger("click");
    };

    const cropper = (file: File) =>
    {
        setFile(file);
        setIsOpen(true);
    };

    const resizeFile = (file: File): Promise<File | null> =>
        new Promise((resolve) =>
        {
            resizeImage(file, 64, 64).then(resolve).catch(e =>
            {
                console.error(e);
            });
        });

    useEffect(() =>
    {
        $(`#${key}`)
            .off("dragleave")
            .off("dragover")
            .off("drop")
            .on("dragover", e =>
            {
                if(isDraggingOver)return;
                e.preventDefault();
                setIsDraggingOver(true);
                console.log("dragover");
            })
            .on("dragleave", e =>
            {
                e.preventDefault();
                setIsDraggingOver(false);
                console.log("dragleave");
            })
            .on("drop", e=>{
                e.preventDefault();
                setIsDraggingOver(false);
                console.log("drop");
                const files = e.originalEvent?.dataTransfer?.files;
                if(files && files.length > 0)
                {
                    cropper(files[0]);
                }
            })
    }, []);

    return (
        <div
            id={key}
            className={
                cn(
                    "min-h-[200px] rounded-2xl flex flex-row p-4 gap-8 items-center justify-center shadow-inner",
                    "outline-2 outline-dotted outline-primary w-full",
                    "data-[dragging=true]:outline-double data-[dragging=true]:bg-primary/10"
                )
            }
            data-dragging={isDraggingOver}
        >
            <ImageCropModal image={file} isOpen={isOpen} onClose={file =>
            {
                if (file) resizeFile(file)
                    .then(file =>
                    {
                        if (!file) return;
                        setFile(file);
                        setFile(file);
                        setIsOpen(false);
                        onUpload(file);
                    });
                setFile(file);
                setIsOpen(false);
            }}/>
            <div className={"text-4xl font-bold text-center p-4 pointer-events-none"}>
                {
                    file ?
                        (
                            <>
                                <Image src={URL.createObjectURL(file)} draggable={false} radius={"lg"} height={200}/>
                            </>
                        ) :
                        (
                            <>
                                Drag<br/>&amp;<br/>Drop
                            </>
                        )
                }
            </div>
            <Divider orientation={"vertical"} className={"mx-[100px]"}/>
            <div className={"flex flex-col"}>
                <Button
                    size={"lg"}
                    variant={"ghost"}
                    color={"primary"}
                    className={"p-8"}
                    startContent={<FontAwesomeIcon icon={faUpload}/>}
                    onClick={handleOpen}
                >
                    Select Icon
                </Button>
            </div>
        </div>
    );
}