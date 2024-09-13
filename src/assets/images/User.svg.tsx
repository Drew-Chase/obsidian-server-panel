interface UserIconProperties
{
    width?: number | string;
    height?: number | string;
    size?: number | string;
    fill?: string;
}

export default function User(props: UserIconProperties)
{
    return (
        <svg width={props.size || props.width || "14"} height={props.size || props.height || "15"} viewBox="0 0 14 15" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M0.943573 12.7389C0.943573 10.5945 2.682 8.85605 4.82647 8.85605H9.17353C11.318 8.85605 13.0564 10.5945 13.0564 12.7389V12.7389C13.0564 13.4538 12.477 14.0332 11.7621 14.0332H2.23787C1.52305 14.0332 0.943573 13.4538 0.943573 12.7389V12.7389Z" fill={props.fill || "currentColor"}/>
            <path d="M7.00232 7.86963C8.9085 7.86963 10.4538 6.32435 10.4538 4.41817C10.4538 2.51198 8.9085 0.966705 7.00232 0.966705C5.09613 0.966705 3.55086 2.51198 3.55086 4.41817C3.55086 6.32435 5.09613 7.86963 7.00232 7.86963Z" fill={props.fill || "currentColor"}/>
        </svg>

    );
}