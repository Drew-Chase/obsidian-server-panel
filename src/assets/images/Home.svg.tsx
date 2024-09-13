interface HomeIconProperties
{
    width?: number | string;
    height?: number | string;
    size?: number | string;
    fill?: string;
}

export default function Home(props: HomeIconProperties)
{
    return (
        <svg width={props.size || props.width || "15"} height={props.size || props.height || "15"} viewBox="0 0 15 15" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M0.0683594 6.83355C0.0683594 6.41283 0.249846 6.01216 0.567115 5.73244L6.07891 0.872996C6.643 0.375668 7.49372 0.375668 8.05781 0.872996L13.5696 5.73244C13.8869 6.01216 14.0684 6.41283 14.0684 6.83355V13.0259C14.0684 13.84 13.4021 14.5 12.5802 14.5H1.55656C0.73465 14.5 0.0683594 13.84 0.0683594 13.0259V6.83355Z" fill={props.fill || "currentColor"}/>
        </svg>
    );
}