interface DownloadFileIconProperties
{
    width?: number | string;
    height?: number | string;
    size?: number | string;
    fill?: string;
}

export default function DownloadFile(props: DownloadFileIconProperties)
{
    const {width, height, size, fill} = props;

    // Determine width and height from size if provided, else use width and height props
    const iconWidth = size || width || "12";
    const iconHeight = size || height || "13";
    const iconFill = fill || "currentColor";

    return (
        <svg
            width={iconWidth}
            height={iconHeight}
            viewBox="0 0 12 13"
            fill="none"
            xmlns="http://www.w3.org/2000/svg"
            style={{minWidth: iconWidth, minHeight: iconHeight, flexShrink: 0}}
        >
            <path
                d="M10.9488 5.29486H9.2148C7.7928 5.29486 6.6348 4.13686 6.6348 2.71486V0.980859C6.6348 0.650859 6.3648 0.380859 6.0348 0.380859H3.4908C1.6428 0.380859 0.148804 1.58086 0.148804 3.72286V9.03886C0.148804 11.1809 1.6428 12.3809 3.4908 12.3809H8.2068C10.0548 12.3809 11.5488 11.1809 11.5488 9.03886V5.89486C11.5488 5.56486 11.2788 5.29486 10.9488 5.29486ZM6.0168 8.64886L4.8168 9.84886C4.7748 9.89086 4.7208 9.92686 4.6668 9.94486C4.6128 9.96886 4.5588 9.98086 4.4988 9.98086C4.4388 9.98086 4.3848 9.96886 4.3308 9.94486C4.2828 9.92686 4.2348 9.89086 4.1988 9.85486C4.1928 9.84886 4.1868 9.84886 4.1868 9.84286L2.9868 8.64286C2.8128 8.46886 2.8128 8.18086 2.9868 8.00686C3.1608 7.83286 3.4488 7.83286 3.6228 8.00686L4.0488 8.44486V5.93086C4.0488 5.68486 4.2528 5.48086 4.4988 5.48086C4.7448 5.48086 4.9488 5.68486 4.9488 5.93086V8.44486L5.3808 8.01286C5.5548 7.83886 5.8428 7.83886 6.0168 8.01286C6.1908 8.18686 6.1908 8.47486 6.0168 8.64886Z"
                fill={iconFill}
            />
            <path
                d="M9.10679 4.46687C9.67679 4.47287 10.4688 4.47287 11.1468 4.47287C11.4888 4.47287 11.6688 4.07087 11.4288 3.83087C10.5648 2.96087 9.01679 1.39487 8.12879 0.506867C7.88279 0.260867 7.45679 0.428867 7.45679 0.770867V2.86487C7.45679 3.74087 8.20079 4.46687 9.10679 4.46687Z"
                fill={iconFill}
            />
        </svg>
    );
}