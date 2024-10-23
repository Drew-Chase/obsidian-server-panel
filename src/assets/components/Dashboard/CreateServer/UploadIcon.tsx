import {Button, Divider, Image} from "@nextui-org/react";
import {FontAwesomeIcon} from "@fortawesome/react-fontawesome";
import {faUpload} from "@fortawesome/free-solid-svg-icons";
import Resizer from "react-image-file-resizer";
import $ from "jquery";
import ImageCropModal from "./ImageCropModal.tsx";
import {useState} from "react";

export default function UploadIcon()
{
    const [isOpen, setIsOpen] = useState<boolean>(false);
    const [file, setFile] = useState<File | null>(null);

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

    const resizeFile = (file: File): Promise<File> =>
        new Promise((resolve) =>
        {
            Resizer.imageFileResizer(
                file,
                64,
                64,
                "PNG",
                100,
                0,
                (uri) =>
                {
                    resolve(uri as File);
                },
                "file"
            );
        });

    const handleFileUpload = async (file: File) =>
    {
        const result: File = await resizeFile(file);
        setFile(result);
    };

    return (
        <div className={"outline-2 outline-dotted outline-primary w-full min-h-[300px] rounded-2xl flex flex-row p-4 gap-8 items-center justify-center shadow-inner"}>
            <ImageCropModal image={file} isOpen={isOpen} onClose={file =>
            {
                if (file) resizeFile(file).then(handleFileUpload);
                setFile(file);
                setIsOpen(false);
            }}/>
            <div className={"text-4xl font-bold text-center p-4 "}>
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