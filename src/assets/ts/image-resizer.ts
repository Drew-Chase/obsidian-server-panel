export function resizeImage(file: File, width: number, height: number): Promise<File>
{
    return new Promise((resolve, reject) =>
    {
        const reader = new FileReader();
        reader.onload = () =>
        {
            const image = new Image();
            image.src = reader.result as string;
            image.onload = () =>
            {
                const canvas = document.createElement("canvas");
                canvas.width = width;
                canvas.height = height;
                const ctx = canvas.getContext("2d");
                if (ctx)
                {
                    ctx.drawImage(image, 0, 0, width, height);
                    canvas.toBlob((blob) =>
                    {
                        if (blob)
                        {
                            const resizedFile = new File([blob], file.name, {
                                type: file.type,
                                lastModified: Date.now()
                            });
                            resolve(resizedFile);
                        } else
                        {
                            reject(new Error("Failed to resize the image"));
                        }
                    }, file.type);
                } else
                {
                    reject(new Error("Failed to get canvas context"));
                }
            };
            image.onerror = (error) => reject(new Error("Failed to load the image: " + error));
        };
        reader.onerror = (error) => reject(new Error("Failed to read the file: " + error));
        reader.readAsDataURL(file);
    });
}

