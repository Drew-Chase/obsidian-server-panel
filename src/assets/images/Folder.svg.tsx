interface FolderIconProperties
{
    width?: number | string;
    height?: number | string;
    size?: number | string;
    fill?: string;
}

export default function Folder(props: FolderIconProperties)
{
    return (
        <svg width={props.size || props.width || "15"} height={props.size || props.height || "12"} viewBox="0 0 15 12" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path
                d="M14.3545 9.79623C14.7047 7.77873 14.7267 5.71327 14.4196 3.688L14.3689 3.35348C14.2365 2.48066 13.5213 1.83844 12.6816 1.83844L5.78394 1.83844C5.75799 1.83844 5.73696 1.81629 5.73696 1.78898C5.73696 0.962038 5.10011 0.291672 4.31452 0.291672H2.50036C1.62699 0.291672 0.891353 0.978606 0.787223 1.89138L0.571505 3.78231C0.346924 5.75095 0.404885 7.74394 0.743444 9.69443C0.881606 10.4904 1.51403 11.0871 2.28076 11.1448L3.47935 11.2351C6.19484 11.4396 8.92074 11.4396 11.6362 11.2351L12.9329 11.1374C13.6419 11.084 14.2267 10.5323 14.3545 9.79623Z"
                fill={props.fill || "currentColor"}/>
        </svg>

    );
}