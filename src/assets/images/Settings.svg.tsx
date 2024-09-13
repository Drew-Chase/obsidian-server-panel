interface SettingsIconProperties
{
    width?: number | string;
    height?: number | string;
    size?: number | string;
    fill?: string;
}

export default function Settings(props: SettingsIconProperties)
{
    return (
        <svg width={props.size ?? props.width ?? "14"} height={props.size ?? props.height ?? "14"} viewBox="0 0 14 14" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M6.92444 4.57214C5.58358 4.57214 4.49659 5.65912 4.49659 6.99999C4.49659 8.34085 5.58358 9.42784 6.92444 9.42784C8.26531 9.42784 9.35229 8.34085 9.35229 6.99999C9.35229 5.65912 8.26531 4.57214 6.92444 4.57214Z" fill={props.fill ?? "currentColor"}/>
            <path fillRule="evenodd" clipRule="evenodd"
                  d="M7.43237 0.131612C7.1165 -0.0438703 6.73242 -0.0438707 6.41656 0.131612L1.03794 3.11973C0.705918 3.30419 0.5 3.65415 0.5 4.03396V9.96604C0.5 10.3459 0.705917 10.6958 1.03794 10.8803L6.41655 13.8684C6.73242 14.0439 7.1165 14.0439 7.43237 13.8684L12.811 10.8803C13.143 10.6958 13.3489 10.3459 13.3489 9.96604V4.03396C13.3489 3.65415 13.143 3.30419 12.811 3.11973L7.43237 0.131612ZM3.37605 6.99999C3.37605 5.04026 4.96472 3.4516 6.92444 3.4516C8.88416 3.4516 10.4728 5.04026 10.4728 6.99999C10.4728 8.95971 8.88416 10.5484 6.92444 10.5484C4.96472 10.5484 3.37605 8.95971 3.37605 6.99999Z"
                  fill={props.fill ?? "currentColor"}/>
        </svg>

    );
}